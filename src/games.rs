use std::{collections::HashMap, str::FromStr};

use toml::{map::Map, Value};

pub const RUNNERS: [&str; 4] = ["dummy", "native", "wine", "ryujinx"];

#[derive(Default, Debug)]
struct Command {
    program: String,
    args: Vec<String>,
    envs: HashMap<String, String>,
}

impl Command {
    fn run(&self) {
        std::process::Command::new(self.program.clone())
            .args(self.args.clone())
            .envs(self.envs.clone())
            .spawn();
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
        }
        if let Some(path) = &cfg.vk_icd_loader {
            self.envs
                .insert("VK_DRIVER_FILES".to_owned(), path.to_string());
        }
    }
}

pub trait Runner {
    fn get_command(&self) -> Command {
        Command::default()
    }
}

pub struct DummyRunner();
impl Runner for DummyRunner {}

pub struct Game {
    pub name: String,
    pub box_art: Option<String>,
    pub release_year: Option<isize>,
    pub image: image::RgbaImage,
    pub path_to_game: String,
    pub runner_id: String,
    pub runner: Box<dyn Runner>,
    pub config: Config,

    pub bare_config: crate::config::Cfg,
}

impl Game {
    pub fn from_toml(path: &str, default: &DefaultCfg) -> Self {
        let toml = std::fs::read_to_string(path).unwrap_or(String::new());
        let value = Value::from_str(&toml[..]).unwrap();
        let tot = value.as_table().unwrap();
        let metadata = tot.get("metadata").unwrap();
        let general = tot.get("general").unwrap();

        let gamefile = metadata
            .get("path_to_game")
            .unwrap()
            .as_str()
            .unwrap()
            .to_owned();

        println!("{:?}", value);
        let box_art = metadata
            .get("box_art")
            .map(|a| a.as_str().unwrap_or(&default.box_art[..]))
            .unwrap_or(&default.box_art[..]);
        let image = image::io::Reader::open(box_art)
            .unwrap()
            .decode()
            .unwrap()
            .to_rgba8();

        let runner_id = metadata.get("runner").unwrap().as_str().unwrap();
        let runner = match runner_id {
            "dummy" => Box::new(DummyRunner()) as Box<dyn Runner>,
            "native" => Box::new(default.native.complete(
                gamefile.clone(),
                tot.get("native").unwrap_or(&Value::Table(Map::new())),
            )) as Box<dyn Runner>,
            "ryujinx" => Box::new(default.ryujinx.complete(
                gamefile.clone(),
                tot.get("ryujinx").unwrap_or(&Value::Table(Map::new())),
            )) as Box<dyn Runner>,
            "wine" => Box::new(default.wine.complete(
                gamefile.clone(),
                tot.get("wine").unwrap_or(&Value::Table(Map::new())),
            )) as Box<dyn Runner>,
            _ => panic!("unknown runner"),
        };

        Self {
            bare_config: crate::config::Cfg(HashMap::new()),

            name: metadata.get("name").unwrap().as_str().unwrap().to_owned(),
            box_art: metadata
                .get("box_art")
                .unwrap()
                .as_str()
                .map(|a| a.to_owned()),
            release_year: metadata
                .get("release_year")
                .unwrap()
                .as_integer()
                .map(|a| a as isize),
            image,

            path_to_game: gamefile,

            runner_id: runner_id.to_owned(),

            runner,

            config: Config {
                mangohud: general
                    .get("mangohud")
                    .map(|a| a.as_bool().unwrap_or(default.cfg.mangohud))
                    .unwrap_or(default.cfg.mangohud),
                mesa_prime: general
                    .get("mesa_prime")
                    .map(|a| a.as_bool().unwrap_or(default.cfg.mesa_prime))
                    .unwrap_or(default.cfg.mesa_prime),

                nv_prime: general
                    .get("nv_prime")
                    .map(|a| a.as_bool().unwrap_or(default.cfg.nv_prime))
                    .unwrap_or(default.cfg.nv_prime),

                vk_icd_loader: {
                    let icd = general.get("vk_icd_loader");
                    if let Some(v) = icd {
                        v.as_str().map(|a| a.to_owned())
                    } else {
                        default.cfg.vk_icd_loader.clone()
                    }
                },
            },
        }
    }

    pub fn run(&self) {
        let mut cmd = self.runner.get_command();
        cmd.apply_config(&self.config);
        eprintln!("{:?}", cmd);
        cmd.run()
    }
}

#[derive(Default, Debug, Clone)]
pub struct Config {
    pub mangohud: bool,
    pub mesa_prime: bool, // prime render offload for mesa drivers => DRI_PRIME=1
    pub nv_prime: bool, // prime render offload for nvidia proprietary drivers => __NV_PRIME_RENDER_OFFLOAD=1 __GLX_VENDOR_LIBRARY_NAME=nvidia __VK_LAYER_NV_optimus=NVIDIA_only
    pub vk_icd_loader: Option<String>, //  VK_DRIVER_FILES="path/to/loader.json" used to be VK_ICD_FILENAMES, but this is deprecated
}
impl Config {
    fn new() -> Self {
        Config {
            mangohud: true,
            ..Config::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct NativeRunner {
    pub path: String,
    pub args: Vec<String>,
}
impl NativeRunner {
    fn complete(&self, gamefile: String, new: &Value) -> Self {
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
            envs: HashMap::new(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct RyujinxRunner {
    pub path: String,
    pub path_to_ryujinx: String,
}
impl RyujinxRunner {
    fn complete(&self, gamefile: String, new: &Value) -> Self {
        let mut out = self.clone();

        out.path = gamefile;
        if let Some(v) = new.get("path_to_ryujinx") {
            if let Some(s) = v.as_str() {
                out.path_to_ryujinx = s.to_owned()
            }
        }

        out
    }
}
impl Runner for RyujinxRunner {
    fn get_command(&self) -> Command {
        Command {
            program: self.path_to_ryujinx.clone(),
            args: vec![self.path.clone()],
            envs: HashMap::new(),
        }
    }
}

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
}
impl WineRunner {
    fn complete(&self, gamefile: String, new: &Value) -> Self {
        let mut out = self.clone();

        out.path = gamefile;
        if let Some(v) = new.get("path_to_wine") {
            if let Some(s) = v.as_str() {
                out.path_to_wine = s.to_owned()
            }
        }

        if let Some(v) = new.get("use_vkd3d") {
            out.use_vkd3d = v.as_bool().unwrap()
        }

        if out.use_vkd3d {
            if let Some(v) = new.get("vkd3d_path") {
                out.vkd3d_path = Some(v.as_str().unwrap().to_owned())
            }
        }
        if let Some(v) = new.get("use_dxvk") {
            out.use_dxvk = v.as_bool().unwrap()
        }

        if out.use_dxvk {
            if let Some(v) = new.get("dxvk_path") {
                out.dxvk_path = Some(v.as_str().unwrap().to_owned())
            }
        }
        if let Some(v) = new.get("use_dxvk_nvapi") {
            out.use_dxvk_nvapi = v.as_bool().unwrap()
        }

        if out.use_dxvk {
            if let Some(v) = new.get("dxvk_nvapi_path") {
                out.dxvk_nvapi_path = Some(v.as_str().unwrap().to_owned())
            }
        }

        if let Some(v) = new.get("fsync") {
            out.fsync = v.as_bool().unwrap();
        }
        if let Some(v) = new.get("esync") {
            out.esync = v.as_bool().unwrap();
        }

        out
    }
}
impl Runner for WineRunner {
    fn get_command(&self) -> Command {
        let mut dlloverrides = vec![];

        let wineprefix = self
            .wineprefix
            .clone()
            .unwrap_or("/home/louisbui63/.wine".to_owned());
        if self.use_vkd3d {
            let mut dlls = vec![];
            for p in std::fs::read_dir(self.vkd3d_path.clone().unwrap() + "/x64").unwrap() {
                let from = p.unwrap().path();
                let to = wineprefix.clone()
                    + "/drive_c/windows/system32/"
                    + from.file_name().unwrap().to_str().unwrap();
                let success = std::fs::copy(from.clone(), to.clone());
                eprintln!(
                    "copied {:?} to {to} as part of vkd3d. This resulted in {:?}",
                    from, success
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
            for p in std::fs::read_dir(self.vkd3d_path.clone().unwrap() + "/x86").unwrap() {
                let from = p.unwrap().path();
                let to = wineprefix.clone()
                    + "/drive_c/windows/syswow64/"
                    + from.file_name().unwrap().to_str().unwrap();
                let success = std::fs::copy(from.clone(), to.clone());
                eprintln!(
                    "copied {:?} to {to} as part of vkd3d. This resulted in {:?}",
                    from, success
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
            for p in std::fs::read_dir(self.dxvk_path.clone().unwrap() + "/x64").unwrap() {
                let from = p.unwrap().path();
                let to = wineprefix.clone()
                    + "/drive_c/windows/system32/"
                    + from.file_name().unwrap().to_str().unwrap();
                let success = std::fs::copy(from.clone(), to.clone());
                eprintln!(
                    "copied {:?} to {to} as part of vkd3d. This resulted in {:?}",
                    from, success
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
            for p in std::fs::read_dir(self.dxvk_path.clone().unwrap() + "/x32").unwrap() {
                let from = p.unwrap().path();
                let to = wineprefix.clone()
                    + "/drive_c/windows/syswow64/"
                    + from.file_name().unwrap().to_str().unwrap();
                let success = std::fs::copy(from.clone(), to.clone());
                eprintln!(
                    "copied {:?} to {to} as part of vkd3d. This resulted in {:?}",
                    from, success
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
            eprintln!("{:?}", self.dxvk_nvapi_path);
            for p in std::fs::read_dir(self.dxvk_nvapi_path.clone().unwrap() + "/x64").unwrap() {
                let from = p.unwrap().path();
                let to = wineprefix.clone()
                    + "/drive_c/windows/system32/"
                    + from.file_name().unwrap().to_str().unwrap();
                let success = std::fs::copy(from.clone(), to.clone());
                eprintln!(
                    "copied {:?} to {to} as part of vkd3d. This resulted in {:?}",
                    from, success
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
            for p in std::fs::read_dir(self.dxvk_nvapi_path.clone().unwrap() + "/x32").unwrap() {
                let from = p.unwrap().path();
                let to = wineprefix.clone()
                    + "/drive_c/windows/syswow64/"
                    + from.file_name().unwrap().to_str().unwrap();
                let success = std::fs::copy(from.clone(), to.clone());
                eprintln!(
                    "copied {:?} to {to} as part of vkd3d. This resulted in {:?}",
                    from, success
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

        let mut envs = HashMap::new();
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
        Command {
            program: self.path_to_wine.clone(),
            args: vec![self.path.clone()],
            envs,
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

pub struct DefaultCfg {
    box_art: String,
    cfg: Config,
    native: NativeRunner,
    ryujinx: RyujinxRunner,
    wine: WineRunner,
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
        let wine = tot.get("wine");

        Self {
            box_art: get_string_or(metadata, "box_art", "/"),

            cfg: Config {
                mangohud: get_bool_or(general, "mangohud", false),
                mesa_prime: get_bool_or(general, "mesa_prime", false),
                nv_prime: get_bool_or(general, "nv_prime", false),
                vk_icd_loader: get_string(general, "vk_icd_loader"),
            },

            native: NativeRunner {
                path: "".to_owned(),
                args: get_string_vec(native, "args"),
            },

            ryujinx: RyujinxRunner {
                path: "".to_owned(),
                path_to_ryujinx: get_string_or(ryujinx, "path_to_ryujinx", "ryujinx"),
            },

            wine: WineRunner {
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
            },
        }
    }
}
