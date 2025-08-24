use std::collections::HashMap;

use crate::{config::CValue, games::*};

/// runner for PS3 games via rpcs3
pub struct Rpcs3Runner;

impl Runner for Rpcs3Runner {
    fn create_instance(
        &self,
        cfg: &crate::config::Cfg,
        path: String,
        default: &HashMap<String, (String, CValue)>,
    ) -> Box<dyn RunnerInstance> {
        Box::new(Rpcs3RunnerInstance {
            path: path,
            path_to_rpcs3: cfg
                .get_or_default("rpcs3:path_to_rpcs3", default)
                .as_string(),
        })
    }

    fn get_config_order(&self) -> (String, Vec<String>) {
        (
            "rpcs3:rpcs3".to_owned(),
            vec!["rpcs3:path_to_rpcs3".to_owned()],
        )
    }

    fn get_default_config(
        &self,
    ) -> std::collections::HashMap<String, (String, crate::config::CValue)> {
        let mut out = HashMap::new();
        out.insert(
            "rpcs3:path_to_rpcs3".to_owned(),
            ("path to rpcs3".to_owned(), CValue::PickFile("".to_owned())),
        );
        out
    }

    fn get_runner_id(&self) -> String {
        "rpcs3".to_owned()
    }
}

#[derive(Debug, Clone)]
pub struct Rpcs3RunnerInstance {
    pub path: String,
    pub path_to_rpcs3: String,
}

impl RunnerInstance for Rpcs3RunnerInstance {
    fn get_command(&self) -> Command {
        let mut envs = std::collections::HashMap::new();
        envs.insert("QT_QPA_PLATFORM".to_owned(), "xcb".to_owned());

        Command {
            program: self.path_to_rpcs3.clone(),
            args: vec!["--no-gui".to_owned(), self.path.clone()],
            envs,
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
