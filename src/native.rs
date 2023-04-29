use crate::games::*;

/// runner for native apps
#[derive(Debug, Clone)]
pub struct NativeRunner {
    pub path: String,
    pub args: Vec<String>,
}
impl NativeRunner {
    fn complete(&self, gamefile: String, new: &toml::Value) -> Self {
        let mut out = self.clone();

        out.path = gamefile;
        if let Some(v) = new.get("args") {
            if let Some(arr) = v.as_array() {
                out.args = arr
                    .iter()
                    .map(|a| a.as_str().unwrap_or("").to_owned())
                    .collect()
            }
        }

        out
    }
}
impl Runner for NativeRunner {
    fn get_command(&self) -> Command {
        Command {
            program: self.path.clone(),
            args: self.args.clone(),
            envs: std::collections::HashMap::new(),
        }
    }
}
