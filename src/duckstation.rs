use crate::games::*;

/// runner for Sony PlayStation games via DuckStation emulator.
#[derive(Debug, Clone)]
pub struct DuckStationRunner {
    pub path: String,
    pub path_to_duckstation: String,
    pub fullscreen: bool,
}

impl Runner for DuckStationRunner {
    fn get_command(&self) -> Command {
        let mut args = vec!["--".to_owned(), self.path.clone()];
        if self.fullscreen {
            args.insert(0, "-fullscreen".to_owned())
        }
        Command {
            program: self.path_to_duckstation.clone(),
            args,
            envs: std::collections::HashMap::new(),
            cwd: None,
        }
    }
    fn get_subcommands(&self) -> Vec<String> {
        vec!["duckstation".to_owned()]
    }
    fn get_subcommand_command(&self, command: String) -> Option<Command> {
        match &command[..] {
            "duckstation" => Some(Command {
                program: self.path_to_duckstation.clone(),
                args: vec![],
                envs: std::collections::HashMap::new(),
                cwd: None,
            }),
            _ => None,
        }
    }
}
