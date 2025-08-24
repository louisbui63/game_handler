use std::collections::HashMap;

use crate::{config::CValue, games::*};

/// runner for Nintendo 3DS games via Citra emulator. Should work with modern forks too. Note that
/// one should use the "QT" version of the executable with this runner.
pub struct CitraRunner;

impl Runner for CitraRunner {
    fn create_instance(
        &self,
        cfg: &crate::config::Cfg,
        path: String,
        default: &HashMap<String, (String, CValue)>,
    ) -> Box<dyn RunnerInstance> {
        Box::new(CitraRunnerInstance {
            path: path,
            path_to_citra: cfg
                .get_or_default("citra:path_to_citra", default)
                .as_string(),
            fullscreen: cfg.get_or_default("citra:fullscreen", default).as_bool(),
        })
    }

    fn get_config_order(&self) -> (String, Vec<String>) {
        (
            "citra:citra".to_owned(),
            vec![
                "citra:path_to_citra".to_owned(),
                "citra:fullscreen".to_owned(),
            ],
        )
    }

    fn get_default_config(
        &self,
    ) -> std::collections::HashMap<String, (String, crate::config::CValue)> {
        let mut out = HashMap::new();
        out.insert(
            "citra:path_to_citra".to_owned(),
            ("path to citra".to_owned(), CValue::PickFile("".to_owned())),
        );
        out.insert(
            "citra:fullscreen".to_owned(),
            ("fullscreen".to_owned(), CValue::Bool(false)),
        );
        out
    }

    fn get_runner_id(&self) -> String {
        "citra".to_owned()
    }
}

#[derive(Debug, Clone)]
pub struct CitraRunnerInstance {
    pub path: String,
    pub path_to_citra: String,
    pub fullscreen: bool,
}

impl RunnerInstance for CitraRunnerInstance {
    fn get_command(&self) -> Command {
        let mut args = vec![self.path.clone()];
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
        vec!["citra".to_owned()]
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
