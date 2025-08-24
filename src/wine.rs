use std::collections::HashMap;

use crate::{
    config::{opt, CValue},
    games::*,
};

/// runner for Windows applications via Wine compatibility layer
/// provides access to vkd3d, dxvk and dxvk_nvapi, among other.
pub struct WineRunner;

impl Runner for WineRunner {
    fn create_instance(
        &self,
        cfg: &crate::config::Cfg,
        path: String,
        default: &HashMap<String, (String, CValue)>,
    ) -> Box<dyn RunnerInstance> {
        Box::new(WineRunnerInstance {
            path: path,
            path_to_wine: cfg.get_or_default("wine:path_to_wine", default).as_string(),
            wineprefix: opt(cfg.get_or_default("wine:wineprefix", default).as_string()),
            use_vkd3d: cfg.get_or_default("wine:use_vkd3d", default).as_bool(),
            vkd3d_path: opt(cfg.get_or_default("wine:vkd3d_path", default).as_string()),
            use_dxvk: cfg.get_or_default("wine:use_dxvk", default).as_bool(),
            dxvk_path: opt(cfg.get_or_default("wine:dxvk_path", default).as_string()),
            use_dxvk_nvapi: cfg.get_or_default("wine:use_dxvk_nvapi", default).as_bool(),
            dxvk_nvapi_path: opt(cfg
                .get_or_default("wine:dxvk_nvapi_path", default)
                .as_string()),
            fsync: cfg.get_or_default("wine:esync", default).as_bool(),
            esync: cfg.get_or_default("wine:fsync", default).as_bool(),
            use_fsr: cfg.get_or_default("wine:use_fsr", default).as_bool(),
            fsr_strength: cfg.get_or_default("wine:fsr_strength", default).as_string(),
            args: cfg.get_or_default("wine:args", default).as_strarr(),
        })
    }

    fn get_config_order(&self) -> (String, Vec<String>) {
        (
            "wine:wine".to_owned(),
            vec![
                "wine:path_to_wine".to_owned(),
                "wine:wineprefix".to_owned(),
                "wine:use_dxvk".to_owned(),
                "wine:dxvk_path".to_owned(),
                "wine:use_vkd3d".to_owned(),
                "wine:vkd3d_path".to_owned(),
                "wine:use_dxvk_nvapi".to_owned(),
                "wine:dxvk_nvapi_path".to_owned(),
                "wine:esync".to_owned(),
                "wine:fsync".to_owned(),
                "wine:use_fsr".to_owned(),
                "wine:fsr_strength".to_owned(),
                "wine:args".to_owned(),
            ],
        )
    }

    fn get_default_config(
        &self,
    ) -> std::collections::HashMap<String, (String, crate::config::CValue)> {
        let mut out = HashMap::new();
        out.insert(
            "wine:args".to_owned(),
            ("arguments".to_owned(), CValue::StrArr(Vec::new())),
        );
        out.insert(
            "wine:path_to_wine".to_owned(),
            (
                "path to wine executable".to_owned(),
                CValue::PickFile("wine".to_owned()),
            ),
        );
        out.insert(
            "wine:wineprefix".to_owned(),
            (
                "path to wineprefix".to_owned(),
                CValue::PickFolder("".to_owned()),
            ),
        );
        out.insert(
            "wine:use_vkd3d".to_owned(),
            ("enable vkd3d".to_owned(), CValue::Bool(false)),
        );
        out.insert(
            "wine:vkd3d_path".to_owned(),
            (
                "path to vkd3d".to_owned(),
                CValue::PickFolder("".to_owned()),
            ),
        );
        out.insert(
            "wine:use_dxvk".to_owned(),
            ("enable dxvk".to_owned(), CValue::Bool(false)),
        );
        out.insert(
            "wine:dxvk_path".to_owned(),
            ("path to dxvk".to_owned(), CValue::PickFolder("".to_owned())),
        );
        out.insert(
            "wine:use_dxvk_nvapi".to_owned(),
            ("enable dxvk_nvapi".to_owned(), CValue::Bool(false)),
        );
        out.insert(
            "wine:dxvk_nvapi_path".to_owned(),
            (
                "path to dxvk_nvapi".to_owned(),
                CValue::PickFolder("".to_owned()),
            ),
        );
        out.insert(
            "wine:esync".to_owned(),
            ("enable esync".to_owned(), CValue::Bool(false)),
        );
        out.insert(
            "wine:fsync".to_owned(),
            ("enable fsync".to_owned(), CValue::Bool(false)),
        );
        out.insert(
            "wine:use_fsr".to_owned(),
            ("enable FSR upscaling".to_owned(), CValue::Bool(false)),
        );
        out.insert(
            "wine:fsr_strength".to_owned(),
            ("FSR strength".to_owned(), CValue::Str("".to_owned())),
        );
        out
    }

    fn get_runner_id(&self) -> String {
        "wine".to_owned()
    }
}

#[derive(Debug, Clone)]
pub struct WineRunnerInstance {
    pub path: String,
    pub path_to_wine: String,
    pub wineprefix: Option<String>,
    pub use_vkd3d: bool,
    pub vkd3d_path: Option<String>,
    pub use_dxvk: bool,
    pub dxvk_path: Option<String>,
    pub use_dxvk_nvapi: bool,
    pub dxvk_nvapi_path: Option<String>,
    pub fsync: bool,
    pub esync: bool,
    pub use_fsr: bool,
    pub fsr_strength: String,
    pub args: Vec<String>,
}

impl RunnerInstance for WineRunnerInstance {
    fn get_subcommands(&self) -> Vec<String> {
        vec![
            "Wine Control Panel".to_owned(),
            "winecfg".to_owned(),
            "cmd".to_owned(),
            "pkill wineserver".to_owned(),
        ]
    }
    fn get_subcommand_command(&self, command: String) -> Option<Command> {
        match &command[..] {
            "Wine Control Panel" => Some(self.real_get_command(Some("control".to_owned()))),
            "winecfg" => Some(self.real_get_command(Some("winecfg".to_owned()))),
            "cmd" => Some(self.real_get_command(Some("cmd".to_owned()))),
            "pkill wineserver" => Some(Command {
                program: "pkill".to_owned(),
                cwd: None,
                args: vec!["wineserver".to_owned()],
                envs: std::collections::HashMap::new(),
            }),
            _ => None,
        }
    }
    fn get_command(&self) -> Command {
        self.real_get_command(None)
    }
}
impl WineRunnerInstance {
    fn real_get_command(&self, command_override: Option<String>) -> Command {
        let mut dlloverrides = vec![];

        let wineprefix = self.wineprefix.clone().unwrap_or(
            directories::BaseDirs::new()
                .unwrap()
                .home_dir()
                .join(".wine")
                .to_str()
                .unwrap()
                .to_owned(),
        );

        let is_32bit =
            !std::path::Path::new(&(wineprefix.clone() + "/drive_c/windows/syswow64")[..]).is_dir();
        if self.use_vkd3d {
            let mut dlls = vec![];
            if !is_32bit {
                for p in std::fs::read_dir(self.vkd3d_path.clone().unwrap() + "/x64").unwrap() {
                    let from = p.unwrap().path();
                    let to = wineprefix.clone()
                        + "/drive_c/windows/system32/"
                        + from.file_name().unwrap().to_str().unwrap();
                    let success = std::fs::copy(from.clone(), to.clone());
                    log::info!(
                        "copied {:?} to {to} as part of vkd3d. This resulted in {:?}",
                        from,
                        success
                    );
                    dlls.push(
                        from.with_extension("")
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_owned(),
                    )
                }
            }
            for p in std::fs::read_dir(self.vkd3d_path.clone().unwrap() + "/x86").unwrap() {
                let from = p.unwrap().path();
                let to = wineprefix.clone()
                    + if !is_32bit {
                        "/drive_c/windows/syswow64/"
                    } else {
                        "/drive_c/windows/system32/"
                    }
                    + from.file_name().unwrap().to_str().unwrap();
                let success = std::fs::copy(from.clone(), to.clone());
                log::info!(
                    "copied {:?} to {to} as part of vkd3d. This resulted in {:?}",
                    from,
                    success
                );
                dlls.push(
                    from.with_extension("")
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_owned(),
                )
            }
            dlls.sort();
            dlls.dedup();
            dlloverrides.push(dlls.join(",") + "=n");
        }

        if self.use_dxvk {
            let mut dlls = vec![];
            if !is_32bit {
                for p in std::fs::read_dir(self.dxvk_path.clone().unwrap() + "/x64").unwrap() {
                    let from = p.unwrap().path();
                    let to = wineprefix.clone()
                        + "/drive_c/windows/system32/"
                        + from.file_name().unwrap().to_str().unwrap();
                    let success = std::fs::copy(from.clone(), to.clone());
                    log::info!(
                        "copied {:?} to {to} as part of vkd3d. This resulted in {:?}",
                        from,
                        success
                    );
                    dlls.push(
                        from.with_extension("")
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_owned(),
                    )
                }
            }
            for p in std::fs::read_dir(self.dxvk_path.clone().unwrap() + "/x32").unwrap() {
                let from = p.unwrap().path();
                let to = wineprefix.clone()
                    + if !is_32bit {
                        "/drive_c/windows/syswow64/"
                    } else {
                        "/drive_c/windows/system32/"
                    }
                    + from.file_name().unwrap().to_str().unwrap();
                let success = std::fs::copy(from.clone(), to.clone());
                log::info!(
                    "copied {:?} to {to} as part of vkd3d. This resulted in {:?}",
                    from,
                    success
                );
                dlls.push(
                    from.with_extension("")
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_owned(),
                )
            }
            dlls.sort();
            dlls.dedup();
            dlloverrides.push(dlls.join(",") + "=n");
        }

        if self.use_dxvk_nvapi {
            let mut dlls = vec![];
            log::info!("{:?}", self.dxvk_nvapi_path);
            if !is_32bit {
                for p in std::fs::read_dir(self.dxvk_nvapi_path.clone().unwrap() + "/x64").unwrap()
                {
                    let from = p.unwrap().path();
                    let to = wineprefix.clone()
                        + "/drive_c/windows/system32/"
                        + from.file_name().unwrap().to_str().unwrap();
                    let success = std::fs::copy(from.clone(), to.clone());
                    log::info!(
                        "copied {:?} to {to} as part of vkd3d. This resulted in {:?}",
                        from,
                        success
                    );
                    dlls.push(
                        from.with_extension("")
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_owned(),
                    )
                }
            }
            for p in std::fs::read_dir(self.dxvk_nvapi_path.clone().unwrap() + "/x32").unwrap() {
                let from = p.unwrap().path();
                let to = wineprefix.clone()
                    + if !is_32bit {
                        "/drive_c/windows/syswow64/"
                    } else {
                        "/drive_c/windows/system32/"
                    }
                    + from.file_name().unwrap().to_str().unwrap();
                let success = std::fs::copy(from.clone(), to.clone());
                log::info!(
                    "copied {:?} to {to} as part of vkd3d. This resulted in {:?}",
                    from,
                    success
                );
                dlls.push(
                    from.with_extension("")
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_owned(),
                )
            }
            dlls.sort();
            dlls.dedup();
            dlloverrides.push(dlls.join(",") + "=n");
        }

        let mut envs = std::collections::HashMap::new();
        if !dlloverrides.is_empty() {
            envs.insert("WINEDLLOVERRIDES".to_owned(), dlloverrides.join(";"));
        }
        if self.use_dxvk_nvapi {
            envs.insert("DXVK_ENABLE_NVAPI".to_owned(), "1".to_owned());
        }
        if self.fsync {
            envs.insert("WINEFSYNC".to_owned(), "1".to_owned());
        }
        if self.esync {
            envs.insert("WINEESYNC".to_owned(), "1".to_owned());
        }
        if let Some(wineprefix) = &self.wineprefix {
            envs.insert("WINEPREFIX".to_owned(), wineprefix.to_owned());
        }
        if self.use_fsr {
            envs.insert("WINE_FULLSCREEN_FSR".to_owned(), "1".to_owned());
            // might want to replace with WINE_FULLSCREEN_FSR_MODE
            envs.insert(
                "WINE_FULLSCREEN_FSR_STRENGTH".to_owned(),
                self.fsr_strength.clone(),
            );
        }
        envs.insert("WINE_LARGE_ADDRESS_AWARE".to_owned(), "1".to_owned());

        if let Some(Some(p)) = std::path::Path::new(&self.path_to_wine)
            .parent()
            .map(|a| a.parent())
        {
            let mut ld_path = String::new();
            let p1 = p.join("lib");
            if p1.exists() {
                if !ld_path.is_empty() {
                    ld_path.push(';')
                }
                ld_path += p1.to_str().unwrap_or("");
            }
            let p2 = p.join("lib32");
            if p2.exists() {
                if !ld_path.is_empty() {
                    ld_path.push(';')
                }
                ld_path += p2.to_str().unwrap_or("");
            }
            let p3 = p.join("lib64");
            if p3.exists() {
                if !ld_path.is_empty() {
                    ld_path.push(';')
                }
                ld_path += p3.to_str().unwrap_or("");
            }
        }
        // envs.insert("GST_PLUGIN_SYSTEM_PATH_1_0".to_owned(), )

        Command {
            program: self.path_to_wine.clone(),
            args: if let Some(a) = command_override {
                vec![a]
            } else {
                let mut a = vec![self.path.clone()];
                a.extend(self.args.clone());
                a
            },
            envs,
            cwd: std::path::PathBuf::from(self.path.clone())
                .parent()
                .map(|a| a.to_owned()), // .ok()
                                        // .map(|a| a.parent().map(|a| a.to_owned()))
                                        // .unwrap_or(None),
        }
    }
}
