use std::collections::HashMap;

use crate::{config::CValue, games::*};

/// runner for Nintendo Switch games via Ryujinx emulator
pub struct RyujinxRunner;

impl Runner for RyujinxRunner {
    fn create_instance(
        &self,
        cfg: &crate::config::Cfg,
        path: String,
        default: &HashMap<String, (String, CValue)>,
    ) -> Box<dyn RunnerInstance> {
        Box::new(RyujinxRunnerInstance {
            path: path,
            path_to_ryujinx: cfg
                .get_or_default("ryujinx:path_to_ryujinx", default)
                .as_string(),
        })
    }

    fn get_config_order(&self) -> (String, Vec<String>) {
        (
            "ryujinx:ryujinx".to_owned(),
            vec!["ryujinx:path_to_ryujinx".to_owned()],
        )
    }

    fn get_default_config(
        &self,
    ) -> std::collections::HashMap<String, (String, crate::config::CValue)> {
        let mut out = HashMap::new();
        out.insert(
            "ryujinx:path_to_ryujinx".to_owned(),
            (
                "path to ryujinx executable".to_owned(),
                CValue::PickFile("ryujinx".to_owned()),
            ),
        );
        out
    }

    fn get_runner_id(&self) -> String {
        "ryujinx".to_owned()
    }
}

#[derive(Debug, Clone)]
pub struct RyujinxRunnerInstance {
    pub path: String,
    pub path_to_ryujinx: String,
}

impl RunnerInstance for RyujinxRunnerInstance {
    fn get_command(&self) -> Command {
        Command {
            program: self.path_to_ryujinx.clone(),
            args: vec![self.path.clone()],
            envs: std::collections::HashMap::new(),
            cwd: None,
        }
    }
    fn get_subcommands(&self) -> Vec<String> {
        vec!["ryujinx".to_owned()]
    }
    fn get_subcommand_command(&self, command: String) -> Option<Command> {
        match &command[..] {
            "ryujinx" => Some(Command {
                program: self.path_to_ryujinx.clone(),
                args: vec![],
                envs: std::collections::HashMap::new(),
                cwd: None,
            }),
            _ => None,
        }
    }
}
