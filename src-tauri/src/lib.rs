use std::process::Command; // Command is basically a terminal prompt  

#[tauri::command]
fn ask_claude(prompt: String) -> Result<String, String> {
    let output = Command::new("claude") 
        .args(["-p", "--output-format", "json", &prompt]) // p for prompt, output-format puts it in json, 
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
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![ask_claude])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
