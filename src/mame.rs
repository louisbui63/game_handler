use std::collections::HashMap;

use crate::{config::CValue, games::*};

/// runner for arcade games via MAME emulator
pub struct MameRunner;

impl Runner for MameRunner {
    fn create_instance(
        &self,
        cfg: &crate::config::Cfg,
        path: String,
        default: &HashMap<String, (String, CValue)>,
    ) -> Box<dyn RunnerInstance> {
        Box::new(MameRunnerInstance {
            path: path,
            machine_name: cfg.get_or_default("mame:machine_name", default).as_string(),
            path_to_mame: cfg.get_or_default("mame:path_to_mame", default).as_string(),
            fullscreen: cfg.get_or_default("mame:fullscreen", default).as_bool(),
        })
    }

    fn get_config_order(&self) -> (String, Vec<String>) {
        (
            "mame:mame".to_owned(),
            vec![
                "mame:path_to_mame".to_owned(),
                "mame:machine_name".to_owned(),
                "mame:fullscreen".to_owned(),
            ],
        )
    }

    fn get_default_config(
        &self,
    ) -> std::collections::HashMap<String, (String, crate::config::CValue)> {
        let mut out = HashMap::new();
        out.insert(
            "mame:path_to_mame".to_owned(),
            ("path to mame".to_owned(), CValue::PickFile("".to_owned())),
        );
        out.insert(
            "mame:machine_name".to_owned(),
            ("machine name".to_owned(), CValue::Str("".to_owned())),
        );
        out.insert(
            "mame:fullscreen".to_owned(),
            ("fullscreen".to_owned(), CValue::Bool(false)),
        );
        out
    }

    fn get_runner_id(&self) -> String {
        "mame".to_owned()
    }
}

#[derive(Debug, Clone)]
pub struct MameRunnerInstance {
    /// here, path must contain the path to the folder containing the machine bios !
    pub path: String,
    pub machine_name: String,
    pub path_to_mame: String,
    pub fullscreen: bool,
}
impl RunnerInstance for MameRunnerInstance {
    fn get_command(&self) -> Command {
        Command {
            program: self.path_to_mame.clone(),
            args: vec![
                if self.fullscreen { "-now" } else { "-w" }.to_owned(),
                "-rompath".to_owned(),
                self.path.clone(),
                self.machine_name.clone(),
            ],
            envs: std::collections::HashMap::new(),
            cwd: Some(crate::DIRS.data_dir().join("mame")),
        }
    }
    fn get_subcommands(&self) -> Vec<String> {
        vec!["MAME".to_owned()]
    }
    fn get_subcommand_command(&self, command: String) -> Option<Command> {
        match &command[..] {
            "MAME" => Some(Command {
                program: self.path_to_mame.clone(),
                args: vec![if self.fullscreen { "-now" } else { "-w" }.to_owned()],
                envs: std::collections::HashMap::new(),
                cwd: None,
            }),
            _ => None,
        }
    }
}
