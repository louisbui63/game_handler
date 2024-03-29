use crate::games::*;

/// runner for PC games on Steam.
#[derive(Debug, Clone)]
pub struct SteamRunner {
    pub path: String,
}

impl Runner for SteamRunner {
    fn get_command(&self) -> Command {
        Command {
            program: "steam".to_owned(),
            args: vec![format!("steam://rungameid/{}", self.path)],
            envs: std::collections::HashMap::new(),
            cwd: None,
        }
    }
    fn get_subcommands(&self) -> Vec<String> {
        return vec!["steam".to_owned()];
    }
    fn get_subcommand_command(&self, command: String) -> Option<Command> {
        match &command[..] {
            "steam" => Some(Command {
                program: "steam".to_owned(),
                args: vec!["steam://open/main".to_owned()],
                envs: std::collections::HashMap::new(),
                cwd: None,
            }),
            _ => None,
        }
    }
}
