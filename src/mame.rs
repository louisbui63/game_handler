use crate::games::*;

/// runner for arcade games via MAME emulator
#[derive(Debug, Clone)]
pub struct MameRunner {
    /// here, path must contain the path to the folder containing the machine bios !
    pub path: String,
    pub machine_name: String,
    pub path_to_mame: String,
    pub fullscreen: bool,
}
impl Runner for MameRunner {
    fn get_command(&self) -> Command {
        Command {
            program: self.path_to_mame.clone(),
            args: vec![
                if self.fullscreen { "-now" } else { "-w" }.to_owned(),
                "-rompath".to_owned(),
                self.path.clone(),
                self.machine_name.clone(),
            ],
            envs: std::collections::HashMap::new(),
        }
    }
    fn get_subcommands(&self) -> Vec<String> {
        return vec!["MAME".to_owned()];
    }
    fn get_subcommand_command(&self, command: String) -> Option<Command> {
        match &command[..] {
            "MAME" => Some(Command {
                program: self.path_to_mame.clone(),
                args: vec![if self.fullscreen { "-now" } else { "-w" }.to_owned()],
                envs: std::collections::HashMap::new(),
            }),
            _ => None,
        }
    }
}
