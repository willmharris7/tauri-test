use std::net::TcpStream; // Creates a TCP connection between app and localhost for data transmission
use std::os::unix::process::CommandExt;
use std::process::Command;
use std::sync::Mutex;
use tauri::{Emitter, Manager};
    // Emitter sends data from a Rust process to the Javascript 
    // Manager handles the lifecycle of the app
use std::thread; // This spawns concurrent threads that can do work simultaneously rather than in sequence 
use std::time::{Duration, Instant};

struct ScreenpipeProcess(Mutex<Option<u32>>);
    // Every process in Unix is assigned a Procces ID, a 32-bit integer hence u32.
    // Option allows for null if it's not already running 
    // Mutex means this process can only be accessed by one thread at a time, to prevent two threads sending contradicting simultaneous instructions

fn get_screenpipe_path(app_handle: &tauri::AppHandle) -> std::path::PathBuf { //PathBuf is Rust's String-like version of a file path 
    app_handle.path().resource_dir() 
        // .resource_dir() is where the binary is copied to, changes for dev/prod
        // for dev it's src-tauri/target/debug
        // for prod it's MyApp.app/Contents/Resources
        // This is necessary because the /binaries folder where screenpipe actually lives is considered source code. Source code is never actually accessible to a running app. 
        .expect("Failed to get resource dir")
        .join("screenpipe") // the platform part of the original binary filename is automatically stripped when copied over
}

fn start_screenpipe_background(app_handle: tauri::AppHandle) {
    thread::spawn(move || {
        let binary_path = get_screenpipe_path(&app_handle);

        let child = Command::new(&binary_path)
            .args(["record"])
            .process_group(0) // make it its own process group leader so we can kill the whole tree
            .spawn()
            .expect("Failed to start screenpipe");

        *app_handle.state::<ScreenpipeProcess>().0.lock().unwrap() = Some(child.id());
        // * skips the Mutex and jumps straight to the u32 PID 
        // .0 accesses the first field of the struct, which is Mutex<Option<u32>>
        // .lock() selects this as the sole thread currentlly allowed to use the Mutex 
        // .unwrap() panics if mutex is poisoined i.e. another thread panicked first while using it 
        // Some() necessary to allow null values

        let timeout = Duration::from_secs(60);
        let start = Instant::now();

        loop { // Tries to connect, waiting 1 seconds. Errors if fails after 1 second. Sleeps 0.5 seconds. tries again. 
            if TcpStream::connect_timeout(
                &"127.0.0.1:3030".parse().unwrap(),
                Duration::from_secs(1),
            ).is_ok() {
                break;
            }
            if start.elapsed() > timeout {
                let _ = app_handle.emit("screenpipe-ready", false);
                // let _ means there is a return value, but I'm not using it. Otherwise Rust throws an error 
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
        // This is preping the Screenpipe Process to exist, but the None means no PID is input yet 
        .setup(|app| {
            start_screenpipe_background(app.handle().clone()); // .clone() allows you to move ownership away from app
            Ok(()) // Ok() is the success result needed from a .setup() and the interior () means there's no result to return 
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![ask_claude])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| {
            if let tauri::RunEvent::Exit = event {
                if let Some(pid) = app_handle.state::<ScreenpipeProcess>().0.lock().unwrap().take() { 
                    // kill the whole process group (negative pid = process group)
                    Command::new("kill").args(["-TERM", &format!("-{}", pid)]).status().ok(); // -TERM for teminate
                }
            }
        });
}
