use std::collections::HashMap;

use crate::{config::CValue, games::*};

/// runner for PS2 games via pcsx2. Specifically made for newer-ish QT-based version of pcsx2.
pub struct Pcsx2Runner;

impl Runner for Pcsx2Runner {
    fn create_instance(
        &self,
        cfg: &crate::config::Cfg,
        path: String,
        default: &HashMap<String, (String, CValue)>,
    ) -> Box<dyn RunnerInstance> {
        Box::new(Pcsx2RunnerInstance {
            path: path,
            path_to_pcsx2: cfg
                .get_or_default("pcsx2:path_to_pcsx2", default)
                .as_string(),
            fullscreen: cfg.get_or_default("pcsx2:fullscreen", default).as_bool(),
        })
    }

    fn get_config_order(&self) -> (String, Vec<String>) {
        (
            "pcsx2:pcsx2".to_owned(),
            vec![
                "pcsx2:path_to_pcsx2".to_owned(),
                "pcsx2:fullscreen".to_owned(),
            ],
        )
    }

    fn get_default_config(
        &self,
    ) -> std::collections::HashMap<String, (String, crate::config::CValue)> {
        let mut out = HashMap::new();
        out.insert(
            "pcsx2:path_to_pcsx2".to_owned(),
            ("path to pcsx2".to_owned(), CValue::PickFile("".to_owned())),
        );
        out.insert(
            "pcsx2:fullscreen".to_owned(),
            ("fullscreen".to_owned(), CValue::Bool(false)),
        );
        out
    }

    fn get_runner_id(&self) -> String {
        "pcsx2".to_owned()
    }
}

#[derive(Debug, Clone)]
pub struct Pcsx2RunnerInstance {
    pub path: String,
    pub path_to_pcsx2: String,
    pub fullscreen: bool,
}

impl RunnerInstance for Pcsx2RunnerInstance {
    fn get_command(&self) -> Command {
        let mut args = vec![];
        if self.fullscreen {
            args.push("-fullscreen".to_owned())
        }
        args.push("--".to_owned());
        args.push(self.path.clone());

        Command {
            program: self.path_to_pcsx2.clone(),
            args,
            envs: std::collections::HashMap::new(),
            cwd: None,
        }
    }
    fn get_subcommands(&self) -> Vec<String> {
        vec!["pcsx2".to_owned()]
    }
    fn get_subcommand_command(&self, command: String) -> Option<Command> {
        match &command[..] {
            "pcsx2" => Some(Command {
                program: self.path_to_pcsx2.clone(),
                args: vec![],
                envs: std::collections::HashMap::new(),
                cwd: None,
            }),
            _ => None,
        }
    }
}
