use crate::games::*;

/// runner for Windows games via the Proton based umu launcher
#[derive(Debug, Clone)]
pub struct UmuRunner {
    pub path: String,
    pub path_to_umu: String,
    pub path_to_proton: String,
    pub gameid: String,
    pub store: String,
}

impl Runner for UmuRunner {
    fn get_command(&self) -> Command {
        let mut envs = std::collections::HashMap::new();

        envs.insert("GAMEID".to_owned(), self.gameid.clone());
        if !self.store.is_empty() {
            envs.insert("STORE".to_owned(), self.store.clone());
        }
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
