use std::collections::HashMap;

use crate::{config::CValue, games::*};

/// runner for Windows games via the Proton based umu launcher
pub struct UmuRunner;

impl Runner for UmuRunner {
    fn create_instance(
        &self,
        cfg: &crate::config::Cfg,
        path: String,
        default: &HashMap<String, (String, CValue)>,
    ) -> Box<dyn RunnerInstance> {
        Box::new(UmuRunnerInstance {
            path: path,
            path_to_umu: cfg.get_or_default("umu:path_to_umu", default).as_string(),
            path_to_proton: cfg
                .get_or_default("umu:path_to_proton", default)
                .as_string(),
            gameid: cfg.get_or_default("umu:gameid", default).as_string(),
            store: cfg.get_or_default("umu:store", default).as_string(),
        })
    }

    fn get_config_order(&self) -> (String, Vec<String>) {
        (
            "umu:umu".to_owned(),
            vec![
                "umu:path_to_umu".to_owned(),
                "umu:path_to_proton".to_owned(),
                "umu:gameid".to_owned(),
                "umu:store".to_owned(),
            ],
        )
    }

    fn get_default_config(
        &self,
    ) -> std::collections::HashMap<String, (String, crate::config::CValue)> {
        let mut out = HashMap::new();
        out.insert(
            "umu:path_to_umu".to_owned(),
            ("path to umu".to_owned(), CValue::PickFile("".to_owned())),
        );
        out.insert(
            "umu:path_to_proton".to_owned(),
            (
                "path to proton".to_owned(),
                CValue::PickFolder("".to_owned()),
            ),
        );
        out.insert(
            "umu:gameid".to_owned(),
            ("gameid".to_owned(), CValue::Str("0".to_owned())),
        );
        out.insert(
            "umu:store".to_owned(),
            ("store".to_owned(), CValue::Str("".to_owned())),
        );
        out
    }

    fn get_runner_id(&self) -> String {
        "umu".to_owned()
    }
}

#[derive(Debug, Clone)]
pub struct UmuRunnerInstance {
    pub path: String,
    pub path_to_umu: String,
    pub path_to_proton: String,
    pub gameid: String,
    pub store: String,
}

impl RunnerInstance for UmuRunnerInstance {
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
