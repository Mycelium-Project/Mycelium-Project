use tauri::{Window, WindowUrl, WindowBuilder};


#[tauri::command]
async fn create_child_window(id: String, window: Window) {
    let child = WindowBuilder::new(&window, id, WindowUrl::default())
        .title("Child")
        .inner_size(400.0, 300.0);

    #[cfg(target_os = "macos")]
    let child = child.parent_window(window.ns_window().unwrap());
    #[cfg(target_os = "windows")]
    let child = child.parent_window(window.hwnd().unwrap());

    child.build().unwrap();
}