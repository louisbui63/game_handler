use crate::games::*;

/// runner for PS3 games via rpcs3
#[derive(Debug, Clone)]
pub struct Rpcs3Runner {
    pub path: String,
    pub path_to_rpcs3: String,
}
impl Rpcs3Runner {
    fn complete(&self, gamefile: String, new: &toml::Value) -> Self {
        let mut out = self.clone();

        out.path = gamefile;
        if let Some(v) = new.get("path_to_rpcs3") {
            if let Some(s) = v.as_str() {
                out.path_to_rpcs3 = s.to_owned()
            }
        }

        out
    }
}
impl Runner for Rpcs3Runner {
    fn get_command(&self) -> Command {
        Command {
            program: self.path_to_rpcs3.clone(),
            args: vec!["--no-gui".to_owned(), self.path.clone()],
            envs: std::collections::HashMap::new(),
        }
    }
    fn get_subcommands(&self) -> Vec<String> {
        return vec!["rpcs3".to_owned()];
    }
    fn get_subcommand_command(&self, command: String) -> Option<Command> {
        match &command[..] {
            "rpcs3" => Some(Command {
                program: self.path_to_rpcs3.clone(),
                args: vec![],
                envs: std::collections::HashMap::new(),
            }),
            _ => None,
        }
    }
}
