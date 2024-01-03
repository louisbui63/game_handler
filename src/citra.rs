use crate::games::*;

/// runner for Nintendo 3DS games via Citra emulator
#[derive(Debug, Clone)]
pub struct CitraRunner {
    pub path: String,
    pub path_to_citra: String,
    pub fullscreen: bool,
}

impl Runner for CitraRunner {
    fn get_command(&self) -> Command {
        let mut args = vec!["-g".to_owned(), self.path.clone()];
        if self.fullscreen {
            args.insert(0, "-f".to_owned())
        }
        Command {
            program: self.path_to_citra.clone(),
            args,
            envs: std::collections::HashMap::new(),
            cwd: None,
        }
    }
    fn get_subcommands(&self) -> Vec<String> {
        return vec!["citra".to_owned()];
    }
    fn get_subcommand_command(&self, command: String) -> Option<Command> {
        match &command[..] {
            "citra" => Some(Command {
                program: self.path_to_citra.clone(),
                args: vec![],
                envs: std::collections::HashMap::new(),
                cwd: None,
            }),
            _ => None,
        }
    }
}
