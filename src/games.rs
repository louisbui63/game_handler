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
