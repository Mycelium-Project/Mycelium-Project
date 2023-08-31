use std::net::Ipv4Addr;


pub enum TeamNumbers {
    /// Team number of the user
    Main{team_number: u16},
    /// Frequent team numbers you would like to support
    Alternate{priority: u8, team_number: u16},
    /// Team numbers that won't be saved and just exist for the current session
    Temporary{priority: u8, team_number: u16},
}

impl TeamNumbers {
    pub fn get_team_number(&self) -> u16 {
        match self {
            TeamNumbers::Main{team_number} => *team_number,
            TeamNumbers::Alternate{team_number, ..} => *team_number,
            TeamNumbers::Temporary{team_number, ..} => *team_number,
        }
    }
}

/// Converts the given team number into a String containing the IP of the roboRIO
/// Assumes the roboRIO will exist at 10.TE.AM.2
pub(crate) fn ip_from_team_number(team: u16) -> Ipv4Addr {
    let s = team.to_string();

    let v4str = match s.len() {
        1 | 2 => format!("10.0.{}.2", team),
        3 => format!("10.{}.{}.2", &s[0..1], &s[1..3]),
        4 => format!("10.{}.{}.2", &s[0..2], &s[2..4]),
        5 => format!("10.{}.{}.2", &s[0..3], &s[3..5]),
        _ => unreachable!(), // Team numbers shouldn't be >5 characters
    };

    v4str.parse().unwrap()
}