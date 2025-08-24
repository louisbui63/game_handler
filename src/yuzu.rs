use std::collections::HashMap;

use crate::{config::CValue, games::*};

/// runner for Nintendo Switch games via Yuzu emulator. Should work with Suyu and other forks too.
pub struct YuzuRunner;

impl Runner for YuzuRunner {
    fn create_instance(
        &self,
        cfg: &crate::config::Cfg,
        path: String,
        default: &HashMap<String, (String, CValue)>,
    ) -> Box<dyn RunnerInstance> {
        Box::new(YuzuRunnerInstance {
            path: path,
            path_to_yuzu: cfg.get_or_default("yuzu:path_to_yuzu", default).as_string(),
            fullscreen: cfg.get_or_default("yuzu:fullscreen", default).as_bool(),
        })
    }

    fn get_config_order(&self) -> (String, Vec<String>) {
        (
            "yuzu:yuzu".to_owned(),
            vec!["yuzu:path_to_yuzu".to_owned(), "yuzu:fullscreen".to_owned()],
        )
    }

    fn get_default_config(
        &self,
    ) -> std::collections::HashMap<String, (String, crate::config::CValue)> {
        let mut out = HashMap::new();
        out.insert(
            "yuzu:path_to_yuzu".to_owned(),
            ("path to yuzu".to_owned(), CValue::PickFile("".to_owned())),
        );
        out.insert(
            "yuzu:fullscreen".to_owned(),
            ("fullscreen".to_owned(), CValue::Bool(false)),
        );
        out
    }

    fn get_runner_id(&self) -> String {
        "yuzu".to_owned()
    }
}

#[derive(Debug, Clone)]
pub struct YuzuRunnerInstance {
    pub path: String,
    pub path_to_yuzu: String,
    pub fullscreen: bool,
}

impl RunnerInstance for YuzuRunnerInstance {
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
