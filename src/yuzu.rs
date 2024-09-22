use crate::games::*;

/// runner for Nintendo Switch games via Yuzu emulator. Should work with Suyu and other forks too.
#[derive(Debug, Clone)]
pub struct YuzuRunner {
    pub path: String,
    pub path_to_yuzu: String,
    pub fullscreen: bool,
}

impl Runner for YuzuRunner {
    fn get_command(&self) -> Command {
        let mut args = vec!["-g".to_owned(), self.path.clone()];
        if self.fullscreen {
            args.insert(0, "-f".to_owned())
        }
        Command {
            program: self.path_to_yuzu.clone(),
            args,
            envs: std::collections::HashMap::new(),
            cwd: None,
        }
    }
    fn get_subcommands(&self) -> Vec<String> {
        vec!["yuzu".to_owned()]
    }
    fn get_subcommand_command(&self, command: String) -> Option<Command> {
        match &command[..] {
            "yuzu" => Some(Command {
                program: self.path_to_yuzu.clone(),
                args: vec![],
                envs: std::collections::HashMap::new(),
                cwd: None,
            }),
            _ => None,
        }
    }
}
