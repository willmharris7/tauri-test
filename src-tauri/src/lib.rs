use std::net::TcpStream;
use std::os::unix::process::CommandExt;
use std::process::Command;
use std::sync::Mutex;
use tauri::{Emitter, Manager};
use std::thread;
use std::time::{Duration, Instant};

struct ScreenpipeProcess(Mutex<Option<u32>>);

fn start_screenpipe_background(app_handle: tauri::AppHandle) {
    thread::spawn(move || {
        let child = Command::new("npx")
            .args(["screenpipe@latest", "record"])
            .process_group(0) // make it its own process group leader so we can kill the whole tree
            .spawn()
            .expect("Failed to start screenpipe");

        *app_handle.state::<ScreenpipeProcess>().0.lock().unwrap() = Some(child.id());

        let timeout = Duration::from_secs(60);
        let start = Instant::now();

        loop {
            if TcpStream::connect_timeout(
                &"127.0.0.1:3030".parse().unwrap(),
                Duration::from_secs(1),
            ).is_ok() {
                break;
            }
            if start.elapsed() > timeout {
                let _ = app_handle.emit("screenpipe-ready", false);
                return;
            }
            thread::sleep(Duration::from_millis(500));
        }

        let _ = app_handle.emit("screenpipe-ready", true);
    });
}

#[tauri::command]
fn ask_claude(prompt: String) -> Result<String, String> {
    let output = Command::new("claude")
        .args(["-p", "--allowedTools", "Bash(*localhost:3030*)", "--output-format", "json", &prompt]) // p for prompt, output-format puts it in json,
        .output()
        .map_err(|e| format!("Failed to run claude: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Claude error: {}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
        // stdout is the raw bytes, utf8 converts to english, lossy throws error characters for problems intead of crashing
        // Cow is Clone-on-write: if fine borrows the original string, otherwise creates a new string. The '_ stands in for 'str but the compilers knows what it is
    let json: serde_json::Value = serde_json::from_str(&stdout) //serde_json is json in any format
        .map_err(|_| format!("Unexpected output: {}", stdout))?;

    json["result"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| format!("No result in output: {}", stdout))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(ScreenpipeProcess(Mutex::new(None)))
        .setup(|app| {
            start_screenpipe_background(app.handle().clone());
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![ask_claude])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| {
            if let tauri::RunEvent::Exit = event {
                if let Some(pid) = app_handle.state::<ScreenpipeProcess>().0.lock().unwrap().take() {
                    // kill the whole process group (negative pid = process group)
                    Command::new("kill").args(["-TERM", &format!("-{}", pid)]).status().ok();
                }
            }
        });
}
