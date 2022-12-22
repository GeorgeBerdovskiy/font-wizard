#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

#[tauri::command]
fn collect_boxes() -> Vec<String> {
	// TODO - Replace with database queries
	let mut boxes = Vec::<String>::new();
	boxes.push("Modern Fonts".to_string());
	boxes.push("Classic Fonts".to_string());

    boxes
}

fn main() {
    tauri::Builder::default()
		.setup(|app| {
       		let window = app.get_window("main").unwrap();

			#[cfg(target_os = "macos")]
			apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
				.expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

    		Ok(())
        })
		.invoke_handler(tauri::generate_handler![collect_boxes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
