use crate::games::*;

/// runner for native apps
#[derive(Debug, Clone)]
pub struct NativeRunner {
    pub path: String,
    pub args: Vec<String>,
}

impl Runner for NativeRunner {
    fn get_command(&self) -> Command {
        Command {
            program: self.path.clone(),
            args: self.args.clone(),
            envs: std::collections::HashMap::new(),
            cwd: None,
        }
    }
}
