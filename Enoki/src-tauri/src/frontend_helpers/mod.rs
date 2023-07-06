
pub mod logging;
pub mod keybinds;
pub mod app_vars;
pub mod windows;

use app_vars::*;

pub fn appvars_plugin<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R> {
    tauri::plugin::Builder::new("appvars")
        .setup(|_app_handle| {tracing::info!("Setting up appvars plugin"); Ok(())})
        .invoke_handler(tauri::generate_handler![
            get_team_numer,
            get_dark_mode,
            get_theme,
            get_coproc_clients,
            get_robot_address,
            get_coproc_ssh_logins,
            get_window_size_pose
        ])
        .build()
}