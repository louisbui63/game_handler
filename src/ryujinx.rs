use crate::games::*;

/// runner for Nintendo Switch games via Ryujinx emulator
#[derive(Debug, Clone)]
pub struct RyujinxRunner {
    pub path: String,
    pub path_to_ryujinx: String,
}

impl Runner for RyujinxRunner {
    fn get_command(&self) -> Command {
        Command {
            program: self.path_to_ryujinx.clone(),
            args: vec![self.path.clone()],
            envs: std::collections::HashMap::new(),
            cwd: None,
        }
    }
    fn get_subcommands(&self) -> Vec<String> {
        return vec!["ryujinx".to_owned()];
    }
    fn get_subcommand_command(&self, command: String) -> Option<Command> {
        match &command[..] {
            "ryujinx" => Some(Command {
                program: self.path_to_ryujinx.clone(),
                args: vec![],
                envs: std::collections::HashMap::new(),
                cwd: None,
            }),
            _ => None,
        }
    }
}
