use std::collections::HashMap;

use crate::{config::CValue, games::*};

/// runner for native apps
pub struct NativeRunner;

impl Runner for NativeRunner {
    fn create_instance(
        &self,
        cfg: &crate::config::Cfg,
        path: String,
        default: &HashMap<String, (String, CValue)>,
    ) -> Box<dyn RunnerInstance> {
        Box::new(NativeRunnerInstance {
            path: path,
            args: cfg.get_or_default("native:args", default).as_strarr(),
        })
    }

    fn get_config_order(&self) -> (String, Vec<String>) {
        ("native:native".to_owned(), vec!["native:args".to_owned()])
    }

    fn get_default_config(
        &self,
    ) -> std::collections::HashMap<String, (String, crate::config::CValue)> {
        let mut out = HashMap::new();
        out.insert(
            "native:args".to_owned(),
            (
                "additional arguments".to_owned(),
                CValue::StrArr(Vec::new()),
            ),
        );
        out
    }

    fn get_runner_id(&self) -> String {
        "native".to_owned()
    }
}

#[derive(Debug, Clone)]
pub struct NativeRunnerInstance {
    pub path: String,
    pub args: Vec<String>,
}

impl RunnerInstance for NativeRunnerInstance {
    fn get_command(&self) -> Command {
        Command {
            program: self.path.clone(),
            args: self.args.clone(),
            envs: std::collections::HashMap::new(),
            cwd: None,
        }
    }
}
