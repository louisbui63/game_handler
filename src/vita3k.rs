use std::collections::HashMap;

use crate::{config::CValue, games::*};

/// runner for PlayStation Vita games via the Vita3K emulator
pub struct Vita3kRunner;

impl Runner for Vita3kRunner {
    fn create_instance(
        &self,
        cfg: &crate::config::Cfg,
        path: String,
        default: &HashMap<String, (String, CValue)>,
    ) -> Box<dyn RunnerInstance> {
        Box::new(Vita3kRunnerInstance {
            path: path,
            path_to_vita3k: cfg
                .get_or_default("vita3k:path_to_vita3k", default)
                .as_string(),
            fullscreen: cfg.get_or_default("vita3k:fullscreen", default).as_bool(),
        })
    }

    fn get_config_order(&self) -> (String, Vec<String>) {
        (
            "vita3k:vita3k".to_owned(),
            vec![
                "vita3k:path_to_vita3k".to_owned(),
                "vita3k:fullscreen".to_owned(),
            ],
        )
    }

    fn get_default_config(
        &self,
    ) -> std::collections::HashMap<String, (String, crate::config::CValue)> {
        let mut out = HashMap::new();
        out.insert(
            "vita3k:path_to_vita3k".to_owned(),
            ("path to vita3k".to_owned(), CValue::PickFile("".to_owned())),
        );
        out.insert(
            "vita3k:fullscreen".to_owned(),
            ("fullscreen".to_owned(), CValue::Bool(false)),
        );
        out
    }

    fn get_runner_id(&self) -> String {
        "vita3k".to_owned()
    }
}

#[derive(Debug, Clone)]
pub struct Vita3kRunnerInstance {
    pub path: String,
    pub path_to_vita3k: String,
    pub fullscreen: bool,
}

impl RunnerInstance for Vita3kRunnerInstance {
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
        vec!["vita3k".to_owned()]
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
