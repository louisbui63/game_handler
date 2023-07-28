use std::{collections::HashMap, str::FromStr};

use toml::Value;

#[cfg(unix)]
pub const RUNNERS: [&str; 7] = [
    "dummy", "native", "wine", "ryujinx", "rpcs3", "mame", "pcsx2",
];
#[cfg(windows)]
pub const RUNNERS: [&str; 6] = ["dummy", "native", "ryujinx", "rpcs3", "mame", "pcsx2"];

#[derive(Default, Debug)]
pub struct Command {
    pub program: String,
    pub cwd: Option<std::path::PathBuf>,
    pub args: Vec<String>,
    pub envs: HashMap<String, String>,
}

impl Command {
    fn run(&self) -> Option<subprocess::Popen> {
        let cmd = subprocess::Exec::cmd(self.program.clone())
            .args(self.args.as_slice())
            .env_extend(self.envs.iter().collect::<Vec<_>>().as_slice())
            .stdout(subprocess::Redirection::Pipe)
            .stderr(subprocess::Redirection::Merge)
            .cwd(if let Some(cwd) = self.cwd.clone() {
                cwd
            } else {
                std::env::current_dir().unwrap()
            })
            .detached();
        log::info!("running command : {:?}", cmd);
        let out = cmd.popen();

        if let Err(e) = out {
            log::error!("error {e} while running command");
            None
        } else {
            let out = out.unwrap();
            Some(out)
        }
    }

    fn apply_config(&mut self, cfg: &Config) {
        if cfg.mangohud {
            self.args.insert(0, self.program.clone());
            self.args.insert(0, "--dlsym".to_owned()); // this solves compatibility issues with some openGL games
            self.program = "mangohud".to_owned();
        }
        if cfg.mesa_prime {
            self.envs.insert("DRI_PRIME".to_owned(), "1".to_owned());
        }
        if cfg.nv_prime {
            self.envs
                .insert("__NV_PRIME_RENDER_OFFLOAD".to_owned(), "1".to_owned());
            self.envs
                .insert("__GLX_VENDOR_LIBRARY_NAME".to_owned(), "nvidia".to_owned());
            self.envs
                .insert("__VK_LAYER_NV_optimus".to_owned(), "NVIDIA_only".to_owned());
            if cfg.gamescope {
                if let Ok(pci) = std::process::Command::new("sh")
                    .args(&[
                        "-c",
                        "lspci -nn | grep -e VGA -e 3D | grep NVIDIA | cut -d ']' -f 3 | cut -c 3-",
                    ])
                    .output()
                {
                    let a = String::from_utf8_lossy(&pci.stdout).to_string();
                    if !a.is_empty() {
                        self.envs.insert("MESA__VK_DEVICE_SELECT".to_owned(), a);
                    } else {
                        log::warn!("couldn't find NVIDIA GPU. Starting gamescope with default GPU.")
                    }
                } else {
                    log::warn!("couldn't find NVIDIA GPU. Starting gamescope with default GPU. Perhaps lspci isn't installed ?")
                }
            }
        }
        if let Some(path) = &cfg.vk_icd_loader {
            self.envs
                .insert("VK_DRIVER_FILES".to_owned(), path.to_string());
        }

        if cfg.gamescope {
            self.args.insert(0, self.program.clone());
            self.args.insert(0, "--".to_owned());
            let mut args = cfg.gamescope_params.clone();
            args.append(&mut self.args);
            self.args = args;
            self.program = "gamescope".to_owned();
        }

        for (k, v) in cfg.envs.iter() {
            self.envs.insert(k.to_owned(), v.to_owned());
        }
    }
}

pub trait Runner {
    fn get_command(&self) -> Command {
        Command::default()
    }
    fn get_subcommands(&self) -> Vec<String> {
        vec![]
    }
    fn get_subcommand_command(&self, _command: String) -> Option<Command> {
        None
    }
}

pub struct DummyRunner();
impl Runner for DummyRunner {}

pub struct Game {
    pub name: String,
    pub box_art: Option<String>,
    pub release_year: Option<isize>,
    pub image: image::RgbaImage,
    pub path_to_game: std::path::PathBuf,
    pub runner_id: String,
    pub runner: Box<dyn Runner>,
    pub config: Config,

    pub path_to_toml: std::path::PathBuf,

    pub bare_config: crate::config::Cfg,

    pub process_handle: Option<subprocess::Popen>,
    #[cfg(unix)]
    pub process_reader: Option<std::io::BufReader<timeout_readwrite::TimeoutReader<std::fs::File>>>,
    #[cfg(windows)]
    pub process_reader: Option<std::io::BufReader<std::fs::File>>,
    pub current_log: String,
    pub no_sleep: Option<nosleep::NoSleep>,

    pub time_played: std::time::Duration,
    pub time_started: Option<std::time::Instant>,
}

impl Game {
    pub fn from_toml(
        path: &std::path::PathBuf,
        default: &std::path::PathBuf,
        play_time_db: &HashMap<String, std::time::Duration>,
    ) -> Self {
        crate::config::Cfg::from_toml(path).to_game(default, path.to_owned(), play_time_db)
    }

    pub fn run(&mut self) {
        let mut cmd = self.runner.get_command();
        cmd.apply_config(&self.config);
        // eprintln!("{:?}", cmd);
        self.process_handle = cmd.run();
    }

    pub fn run_subcommand(&mut self, a: String) {
        let cmd = self.runner.get_subcommand_command(a);
        if let Some(mut cmd) = cmd {
            cmd.apply_config(&self.config);
            // eprintln!("{:?}", cmd);
            self.process_handle = cmd.run();
        }
    }

    pub fn get_subcommands(&self) -> Vec<String> {
        self.runner.get_subcommands()
    }
}

#[derive(Default, Debug, Clone)]
pub struct Config {
    pub mangohud: bool,
    pub mesa_prime: bool, // prime render offload for mesa drivers => DRI_PRIME=1
    pub nv_prime: bool, // prime render offload for nvidia proprietary drivers => __NV_PRIME_RENDER_OFFLOAD=1 __GLX_VENDOR_LIBRARY_NAME=nvidia __VK_LAYER_NV_optimus=NVIDIA_only
    pub vk_icd_loader: Option<String>, //  VK_DRIVER_FILES="path/to/loader.json" used to be VK_ICD_FILENAMES, but this is deprecated
    pub envs: HashMap<String, String>,
    pub no_sleep_enabled: bool,
    pub gamescope: bool,
    pub gamescope_params: Vec<String>,
}
impl Config {
    fn new() -> Self {
        Config {
            mangohud: true,
            ..Config::default()
        }
    }
}

fn get_bool_or(f: Option<&toml::Value>, key: &str, default: bool) -> bool {
    if let Some(f) = f {
        if let Some(val) = f.get(key) {
            val.as_bool().unwrap_or(default)
        } else {
            default
        }
    } else {
        default
    }
}
fn get_string_or(f: Option<&toml::Value>, key: &str, default: &str) -> String {
    if let Some(f) = f {
        if let Some(val) = f.get(key) {
            val.as_str().unwrap_or(default).to_owned()
        } else {
            default.to_owned()
        }
    } else {
        default.to_owned()
    }
}
fn get_string(f: Option<&toml::Value>, key: &str) -> Option<String> {
    if let Some(f) = f {
        if let Some(val) = f.get(key) {
            val.as_str().map(|a| a.to_owned())
        } else {
            None
        }
    } else {
        None
    }
}

fn get_string_vec(f: Option<&toml::Value>, key: &str) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(f) = f {
        if let Some(val) = f.get(key) {
            if let Some(a) = val.as_array() {
                for i in a {
                    if let Some(ad) = i.as_str() {
                        out.push(ad.to_owned())
                    }
                }
            }
        }
    }
    out
}

fn get_hashmap(f: Option<&toml::Value>, key: &str) -> std::collections::HashMap<String, String> {
    let mut out = std::collections::HashMap::new();
    if let Some(f) = f {
        if let Some(val) = f.get(key) {
            if let Some(a) = val.as_array() {
                let mut first: Option<String> = None;
                for i in a {
                    if let Some(ad) = i.as_str() {
                        if let Some(fst) = first {
                            out.insert(fst, ad.to_owned());
                            first = None
                        } else {
                            first = Some(ad.to_owned())
                        }
                    }
                }
                if let Some(fst) = first {
                    out.insert(fst, "".to_owned());
                }
            }
        }
    }
    out
}

pub struct DefaultCfg {
    box_art: String,
    cfg: Config,
    native: crate::native::NativeRunner,
    ryujinx: crate::ryujinx::RyujinxRunner,
    wine: crate::wine::WineRunner,
    rpcs3: crate::rpcs3::Rpcs3Runner,
    mame: crate::mame::MameRunner,
}

impl DefaultCfg {
    pub fn new(path: &str) -> Self {
        let toml = std::fs::read_to_string(path).unwrap_or(String::new());
        let value = Value::from_str(&toml[..]).unwrap();
        let tot = value.as_table().unwrap();
        let metadata = tot.get("metadata");
        let general = tot.get("general");
        let native = tot.get("native");
        let ryujinx = tot.get("ryujinx");
        let rpcs3 = tot.get("rpcs3");
        let wine = tot.get("wine");
        let mame = tot.get("mame");

        Self {
            box_art: get_string_or(metadata, "box_art", "/"),

            cfg: Config {
                mangohud: get_bool_or(general, "mangohud", false),
                mesa_prime: get_bool_or(general, "mesa_prime", false),
                nv_prime: get_bool_or(general, "nv_prime", false),
                vk_icd_loader: get_string(general, "vk_icd_loader"),
                envs: get_hashmap(general, "env_variables"),
                no_sleep_enabled: get_bool_or(general, "no_sleep_enabled", false),
                gamescope: get_bool_or(general, "gamescope", false),
                gamescope_params: get_string_vec(general, "gamescope_params"),
            },

            native: crate::native::NativeRunner {
                path: "".to_owned(),
                args: get_string_vec(native, "args"),
            },

            ryujinx: crate::ryujinx::RyujinxRunner {
                path: "".to_owned(),
                path_to_ryujinx: get_string_or(ryujinx, "path_to_ryujinx", "ryujinx"),
            },

            wine: crate::wine::WineRunner {
                path: "".to_owned(),
                path_to_wine: get_string_or(wine, "path_to_wine", "wine"),
                wineprefix: None,
                use_vkd3d: get_bool_or(wine, "use_vkd3d", false),
                vkd3d_path: get_string(wine, "vkd3d_path"),
                use_dxvk: get_bool_or(wine, "use_dxvk", false),
                dxvk_path: get_string(wine, "dxvk_path"),
                use_dxvk_nvapi: get_bool_or(wine, "use_dxvk_nvapi", false),
                dxvk_nvapi_path: get_string(wine, "dxvk_nvapi_path"),
                fsync: get_bool_or(wine, "fsync", false),
                esync: get_bool_or(wine, "esync", false),
                use_fsr: get_bool_or(wine, "use_fsr", false),
                fsr_strength: get_string_or(wine, "fsr_strength", ""),
            },

            rpcs3: crate::rpcs3::Rpcs3Runner {
                path: "".to_owned(),
                path_to_rpcs3: get_string_or(rpcs3, "path_to_rpcs3", "rpcs3"),
            },

            mame: crate::mame::MameRunner {
                path: "".to_owned(),
                path_to_mame: get_string_or(mame, "path_to_mame", "mame"),
                machine_name: "".to_owned(),
                fullscreen: get_bool_or(mame, "fullscreen", false),
            },
        }
    }
}
