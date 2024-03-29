use crate::games::*;

/// runner for Windows applications via Wine compatibility layer
/// provides access to vkd3d, dxvk and dxvk_nvapi, among other.
#[derive(Debug, Clone)]
pub struct WineRunner {
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

impl Runner for WineRunner {
    fn get_subcommands(&self) -> Vec<String> {
        vec![
            "Wine Control Panel".to_owned(),
            "winecfg".to_owned(),
            "cmd".to_owned(),
        ]
    }
    fn get_subcommand_command(&self, command: String) -> Option<Command> {
        match &command[..] {
            "Wine Control Panel" => Some(self.real_get_command(Some("control".to_owned()))),
            "winecfg" => Some(self.real_get_command(Some("winecfg".to_owned()))),
            "cmd" => Some(self.real_get_command(Some("cmd".to_owned()))),
            _ => None,
        }
    }
    fn get_command(&self) -> Command {
        self.real_get_command(None)
    }
}
impl WineRunner {
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
        if dlloverrides.len() != 0 {
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
            cwd: std::path::PathBuf::try_from(self.path.clone())
                .ok()
                .map(|a| a.parent().map(|a| a.to_owned()))
                .unwrap_or(None),
        }
    }
}
