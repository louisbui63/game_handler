use std::collections::HashMap;

use crate::{config::CValue, games::*};

/// runner for PC games on Steam.
pub struct SteamRunner;

impl Runner for SteamRunner {
    fn create_instance(
        &self,
        _cfg: &crate::config::Cfg,
        path: String,
        _default: &HashMap<String, (String, CValue)>,
    ) -> Box<dyn RunnerInstance> {
        Box::new(SteamRunnerInstance { path })
    }

    fn get_config_order(&self) -> (String, Vec<String>) {
        ("steam:steam".to_owned(), vec![])
    }

    fn get_default_config(
        &self,
    ) -> std::collections::HashMap<String, (String, crate::config::CValue)> {
        HashMap::new()
    }

    fn get_runner_id(&self) -> String {
        "steam".to_owned()
    }
}

#[derive(Debug, Clone)]
pub struct SteamRunnerInstance {
    pub path: String,
}

impl RunnerInstance for SteamRunnerInstance {
    fn get_command(&self) -> Command {
        Command {
            program: "steam".to_owned(),
            args: vec![format!("steam://rungameid/{}", self.path)],
            envs: std::collections::HashMap::new(),
            cwd: None,
        }
    }
    fn get_subcommands(&self) -> Vec<String> {
        vec!["steam".to_owned()]
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
