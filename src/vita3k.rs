use crate::games::*;

/// runner for PlayStation Vita games via the Vita3K emulator
#[derive(Debug, Clone)]
pub struct Vita3kRunner {
    pub path: String,
    pub path_to_vita3k: String,
    pub fullscreen: bool,
}

impl Runner for Vita3kRunner {
    fn get_command(&self) -> Command {
        let mut args = vec!["-r".to_owned(), self.path.clone()];
        if self.fullscreen {
            args.insert(0, "--fullscreen".to_owned())
        }
        Command {
            program: self.path_to_vita3k.clone(),
            args,
            envs: std::collections::HashMap::new(),
            cwd: None,
        }
    }
    fn get_subcommands(&self) -> Vec<String> {
        return vec!["vita3k".to_owned()];
    }
    fn get_subcommand_command(&self, command: String) -> Option<Command> {
        match &command[..] {
            "vita3k" => Some(Command {
                program: self.path_to_vita3k.clone(),
                args: vec![],
                envs: std::collections::HashMap::new(),
                cwd: None,
            }),
            _ => None,
        }
    }
}
