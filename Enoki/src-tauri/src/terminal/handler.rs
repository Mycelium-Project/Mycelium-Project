use std::{path::PathBuf, process::Child, io::{Write, Read}};

#[derive(Debug)]
pub struct CLIWrapper {
    pub cli_exe: PathBuf,
    pub cli_args: Vec<String>,
    pub cli: Option<Child>
}

impl CLIWrapper {
    pub fn new(cli_exe: PathBuf, cli_args: Vec<String>) -> Self {
        Self {
            cli_exe,
            cli_args,
            cli: None
        }
    }

    pub fn run(&mut self) -> Result<(), ()> {
        let mut command = std::process::Command::new(&self.cli_exe);
        command.args(&self.cli_args);
        let res = command.spawn();
        if res.is_err() {
            tracing::error!("Failed to start cli: {}", res.unwrap_err());
            return Err(());
        }
        self.cli = Some(res.unwrap());
        Ok(())
    }

    pub fn kill(&mut self) -> Result<(), ()> {
        if let Some(cli) = &mut self.cli {
            let res = cli.kill();
            if res.is_err() {
                tracing::error!("Failed to kill cli: {}", res.unwrap_err());
                return Err(());
            }
            self.cli = None;
            self.cli_args.clear();
        }
        Ok(())
    }

    pub fn write_command(&mut self, command: &str) -> Result<(), ()> {
        if let Some(cli) = &mut self.cli {
            let res = cli.stdin.as_mut().unwrap().write(command.as_bytes());
            if res.is_err() {
                tracing::error!("Failed to write to cli: {}", res.unwrap_err());
                return Err(());
            }
        }
        Ok(())
    }

    pub fn read_output(&mut self) -> Result<String, ()> {
        if let Some(cli) = &mut self.cli {
            let mut output = String::new();
            let res = cli.stdout.as_mut().unwrap().read_to_string(&mut output);
            if res.is_err() {
                tracing::error!("Failed to read from cli: {}", res.unwrap_err());
                return Err(());
            }
            return Ok(output);
        }
        Ok(String::new())
    }

    pub fn read_error(&mut self) -> Result<String, ()> {
        if let Some(cli) = &mut self.cli {
            let mut output = String::new();
            let res = cli.stderr.as_mut().unwrap().read_to_string(&mut output);
            if res.is_err() {
                tracing::error!("Failed to read from cli: {}", res.unwrap_err());
                return Err(());
            }
            return Ok(output);
        }
        Ok(String::new())
    }

    pub fn is_running(&mut self) -> bool {
        if let Some(cli) = &mut self.cli {
            match cli.try_wait() {
                Ok(Some(_)) => {
                    return false;
                }
                Ok(None) => {
                    return true;
                }
                Err(e) => {
                    tracing::error!("Failed to check if cli is running: {}", e);
                    return false;
                }
            }
        }
        false
    }

    pub fn exe_name(&self) -> String {
        if let Some(name) = self.cli_exe.file_name() {
            return name.to_str().unwrap_or("unkown").to_string();
        } else {
            "unknown".to_string()
        }
    }
}