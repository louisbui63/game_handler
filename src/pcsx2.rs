use crate::games::*;

/// runner for PS2 games via pcsx2. Specifically made for newer-ish QT-based version of pcsx2.
#[derive(Debug, Clone)]
pub struct Pcsx2Runner {
    pub path: String,
    pub path_to_pcsx2: String,
    pub fullscreen: bool,
}
impl Pcsx2Runner {
    fn complete(&self, gamefile: String, new: &toml::Value) -> Self {
        let mut out = self.clone();

        out.path = gamefile;
        if let Some(v) = new.get("path_to_pcsx2") {
            if let Some(s) = v.as_str() {
                out.path_to_pcsx2 = s.to_owned()
            }
        }

        out
    }
}
impl Runner for Pcsx2Runner {
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
        return vec!["pcsx2".to_owned()];
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
