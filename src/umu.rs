use crate::games::*;

/// runner for PlayStation Vita games via the Vita3K emulator
#[derive(Debug, Clone)]
pub struct UmuRunner {
    pub path: String,
    pub path_to_umu: String,
    pub path_to_proton: String,
}

impl Runner for UmuRunner {
    fn get_command(&self) -> Command {
        let mut envs = std::collections::HashMap::new();

        envs.insert("GAMEID".to_owned(), "0".to_owned());
        envs.insert("PROTONPATH".to_owned(), self.path_to_proton.clone());

        let args = vec![self.path.clone()];
        Command {
            program: self.path_to_umu.clone(),
            args,
            envs,
            cwd: None,
        }
    }
    fn get_subcommands(&self) -> Vec<String> {
        vec![]
    }
    fn get_subcommand_command(&self, _command: String) -> Option<Command> {
        None
    }
}
