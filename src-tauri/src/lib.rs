use tauri::{Emitter, Manager};
use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::CommandChild;
use std::sync::Mutex;

pub struct DaemonState {
    pub child: Mutex<Option<CommandChild>>,
}

#[tauri::command]
async fn restart_daemon(app: tauri::AppHandle, state: tauri::State<'_, DaemonState>) -> Result<(), String> {
    // Kill existing daemon
    if let Some(child) = state.child.lock().unwrap().take() {
        let _ = child.kill();
    }

    // Start new daemon
    start_daemon_internal(&app, &state).await
}

#[tauri::command]
async fn stop_daemon(state: tauri::State<'_, DaemonState>) -> Result<(), String> {
    if let Some(child) = state.child.lock().unwrap().take() {
        child.kill().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn get_daemon_status(state: tauri::State<'_, DaemonState>) -> bool {
    state.child.lock().unwrap().is_some()
}

async fn start_daemon_internal(app: &tauri::AppHandle, state: &tauri::State<'_, DaemonState>) -> Result<(), String> {
    let shell = app.shell();

    let sidecar = shell
        .sidecar("centy-daemon")
        .map_err(|e| format!("Failed to create sidecar command: {}", e))?;

    let (mut rx, child) = sidecar
        .spawn()
        .map_err(|e| format!("Failed to spawn sidecar: {}", e))?;

    // Store the child process
    *state.child.lock().unwrap() = Some(child);

    // Spawn a task to handle daemon output
    let app_handle = app.clone();
    tokio::spawn(async move {
        use tauri_plugin_shell::process::CommandEvent;

        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(line) => {
                    println!("[daemon stdout] {}", String::from_utf8_lossy(&line));
                }
                CommandEvent::Stderr(line) => {
                    eprintln!("[daemon stderr] {}", String::from_utf8_lossy(&line));
                }
                CommandEvent::Error(error) => {
                    eprintln!("[daemon error] {}", error);
                }
                CommandEvent::Terminated(payload) => {
                    println!("[daemon terminated] code: {:?}, signal: {:?}", payload.code, payload.signal);
                    // Emit event to frontend about daemon termination
                    let _ = app_handle.emit("daemon-terminated", &payload);
                }
                _ => {}
            }
        }
    });

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(DaemonState {
            child: Mutex::new(None),
        })
        .setup(|app| {
            let app_handle = app.handle().clone();
            let state = app.state::<DaemonState>();

            // Start daemon on app launch
            tauri::async_runtime::block_on(async {
                if let Err(e) = start_daemon_internal(&app_handle, &state).await {
                    eprintln!("Failed to start daemon: {}", e);
                } else {
                    println!("Daemon started successfully");
                }
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                // Stop daemon when window is closed
                let state = window.state::<DaemonState>();
                let mut guard = state.child.lock().unwrap();
                if let Some(child) = guard.take() {
                    let _ = child.kill();
                    println!("Daemon stopped");
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            restart_daemon,
            stop_daemon,
            get_daemon_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
