use std::{net::{SocketAddrV4, Ipv4Addr}, hash::Hash, collections::{HashMap, HashSet}, sync::Arc};

use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;

static APP_VARS: Lazy<Arc<Mutex<HashSet<ApplicationVariables>>>> = Lazy::new(|| Arc::new(Mutex::new(default_appvar_set())));

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CoprocAddress {
    pub address: SocketAddrV4,
    pub identity: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ApplicationVariables {
    TeamNumber(u16),
    DarkMode(bool),
    Theme(Color, Color, Color, Color),
    CoprocClients(Vec<CoprocAddress>),
    RobotAddress(SocketAddrV4),
    CoprocSshLogins(HashMap<String, String>),
    WindowSizePose(u32, u32, u32, u32)
}

impl Hash for ApplicationVariables {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            //hash just based off of the enum variant
            ApplicationVariables::TeamNumber(_) => 0.hash(state),
            ApplicationVariables::DarkMode(_) => 1.hash(state),
            ApplicationVariables::Theme(_, _, _, _) => 2.hash(state),
            ApplicationVariables::CoprocClients(_) => 3.hash(state),
            ApplicationVariables::RobotAddress(_) => 4.hash(state),
            ApplicationVariables::CoprocSshLogins(_) => 5.hash(state),
            ApplicationVariables::WindowSizePose(_, _, _, _) => 6.hash(state),
        }
    }
}

impl PartialEq for ApplicationVariables {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ApplicationVariables::TeamNumber(_), ApplicationVariables::TeamNumber(_)) => true,
            (ApplicationVariables::DarkMode(_), ApplicationVariables::DarkMode(_)) => true,
            (ApplicationVariables::Theme(_, _, _, _), ApplicationVariables::Theme(_, _, _, _)) => true,
            (ApplicationVariables::CoprocClients(_), ApplicationVariables::CoprocClients(_)) => true,
            (ApplicationVariables::RobotAddress(_), ApplicationVariables::RobotAddress(_)) => true,
            (ApplicationVariables::CoprocSshLogins(_), ApplicationVariables::CoprocSshLogins(_)) => true,
            (ApplicationVariables::WindowSizePose(_, _, _, _), ApplicationVariables::WindowSizePose(_, _, _, _)) => true,
            _ => false
        }
    }
}

impl Eq for ApplicationVariables {}

pub fn default_appvar_set() -> HashSet<ApplicationVariables> {
    let mut set = HashSet::new();

    set.insert(ApplicationVariables::TeamNumber(0));
    set.insert(ApplicationVariables::DarkMode(false));
    set.insert(ApplicationVariables::Theme(Default::default(), Default::default(), Default::default(), Default::default()));
    set.insert(ApplicationVariables::CoprocClients(Vec::new()));
    set.insert(ApplicationVariables::RobotAddress(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0)));
    set.insert(ApplicationVariables::CoprocSshLogins(HashMap::new()));
    set.insert(ApplicationVariables::WindowSizePose(0, 0, 0, 0));

    set
}

impl ApplicationVariables {
    pub fn get_team_numer(&self) -> Option<u16> {
        match self {
            ApplicationVariables::TeamNumber(num) => Some(*num),
            _ => None
        }
    }

    pub fn get_dark_mode(&self) -> Option<bool> {
        match self {
            ApplicationVariables::DarkMode(mode) => Some(*mode),
            _ => None
        }
    }

    pub fn get_theme(&self) -> Option<(Color, Color, Color, Color)> {
        match self {
            ApplicationVariables::Theme(c1, c2, c3, c4) => Some((*c1, *c2, *c3, *c4)),
            _ => None
        }
    }

    pub fn get_coproc_clients(&self) -> Option<Vec<CoprocAddress>> {
        match self {
            ApplicationVariables::CoprocClients(clients) => Some(clients.clone()),
            _ => None
        }
    }

    pub fn get_robot_address(&self) -> Option<SocketAddrV4> {
        match self {
            ApplicationVariables::RobotAddress(addr) => Some(*addr),
            _ => None
        }
    }

    pub fn get_coproc_ssh_logins(&self) -> Option<HashMap<String, String>> {
        match self {
            ApplicationVariables::CoprocSshLogins(logins) => Some(logins.clone()),
            _ => None
        }
    }

    pub fn get_window_size_pose(&self) -> Option<(u32, u32, u32, u32)> {
        match self {
            ApplicationVariables::WindowSizePose(x, y, w, h) => Some((*x, *y, *w, *h)),
            _ => None
        }
    }
}

pub fn set_appvar(var: ApplicationVariables) {
    let mut set = APP_VARS.blocking_lock();
    set.insert(var);
}

#[tauri::command]
pub fn get_team_numer() -> u16 {
    let set = APP_VARS.blocking_lock();
    set.iter().find_map(|var| var.get_team_numer()).unwrap()
}

#[tauri::command]
pub fn get_dark_mode() -> bool {
    let set = APP_VARS.blocking_lock();
    set.iter().find_map(|var| var.get_dark_mode()).unwrap()
}

#[tauri::command]
pub fn get_theme() -> (Color, Color, Color, Color) {
    let set = APP_VARS.blocking_lock();
    set.iter().find_map(|var| var.get_theme()).unwrap()
}

#[tauri::command]
pub fn get_coproc_clients() -> Vec<CoprocAddress> {
    let set = APP_VARS.blocking_lock();
    set.iter().find_map(|var| var.get_coproc_clients()).unwrap()
}

#[tauri::command]
pub fn get_robot_address() -> SocketAddrV4 {
    let set = APP_VARS.blocking_lock();
    set.iter().find_map(|var| var.get_robot_address()).unwrap()
}

#[tauri::command]
pub fn get_coproc_ssh_logins() -> HashMap<String, String> {
    let set = APP_VARS.blocking_lock();
    set.iter().find_map(|var| var.get_coproc_ssh_logins()).unwrap()
}

#[tauri::command]
pub fn get_window_size_pose() -> (u32, u32, u32, u32) {
    let set = APP_VARS.blocking_lock();
    set.iter().find_map(|var| var.get_window_size_pose()).unwrap()
}