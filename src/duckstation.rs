use std::collections::HashMap;

use crate::{config::CValue, games::*};

/// runner for Sony PlayStation games via DuckStation emulator.
pub struct DuckStationRunner;

impl Runner for DuckStationRunner {
    fn create_instance(
        &self,
        cfg: &crate::config::Cfg,
        path: String,
        default: &HashMap<String, (String, CValue)>,
    ) -> Box<dyn RunnerInstance> {
        Box::new(DuckStationRunnerInstance {
            path: path,
            path_to_duckstation: cfg
                .get_or_default("duckstation:path_to_duckstation", default)
                .as_string(),
            fullscreen: cfg
                .get_or_default("duckstation:fullscreen", default)
                .as_bool(),
        })
    }

    fn get_config_order(&self) -> (String, Vec<String>) {
        (
            "duckstation:duckstation".to_owned(),
            vec![
                "duckstation:path_to_duckstation".to_owned(),
                "duckstation:fullscreen".to_owned(),
            ],
        )
    }

    fn get_default_config(
        &self,
    ) -> std::collections::HashMap<String, (String, crate::config::CValue)> {
        let mut out = HashMap::new();
        out.insert(
            "duckstation:path_to_duckstation".to_owned(),
            (
                "path to duckstation".to_owned(),
                CValue::PickFile("".to_owned()),
            ),
        );
        out.insert(
            "duckstation:fullscreen".to_owned(),
            ("fullscreen".to_owned(), CValue::Bool(false)),
        );
        out
    }

    fn get_runner_id(&self) -> String {
        "duckstation".to_owned()
    }
}

#[derive(Debug, Clone)]
pub struct DuckStationRunnerInstance {
    pub path: String,
    pub path_to_duckstation: String,
    pub fullscreen: bool,
}

impl RunnerInstance for DuckStationRunnerInstance {
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
