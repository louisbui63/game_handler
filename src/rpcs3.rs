use crate::games::*;

/// runner for PS3 games via rpcs3
#[derive(Debug, Clone)]
pub struct Rpcs3Runner {
    pub path: String,
    pub path_to_rpcs3: String,
}

impl Runner for Rpcs3Runner {
    fn get_command(&self) -> Command {
        Command {
            program: self.path_to_rpcs3.clone(),
            args: vec!["--no-gui".to_owned(), self.path.clone()],
            envs: std::collections::HashMap::new(),
            cwd: None,
        }
    }
    fn get_subcommands(&self) -> Vec<String> {
        vec!["rpcs3".to_owned()]
    }
    fn get_subcommand_command(&self, command: String) -> Option<Command> {
        match &command[..] {
            "rpcs3" => Some(Command {
                program: self.path_to_rpcs3.clone(),
                args: vec![],
                envs: std::collections::HashMap::new(),
                cwd: None,
            }),
            _ => None,
        }
    }
}
