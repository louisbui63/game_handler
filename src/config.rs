use std::{collections::HashMap, str::FromStr};

#[cfg(unix)]
use crate::wine::WineRunner;
use crate::{
    games::{DummyRunner, Runner},
    mame::MameRunner,
    native::NativeRunner,
    pcsx2::Pcsx2Runner,
    rpcs3::Rpcs3Runner,
    ryujinx::RyujinxRunner,
    yuzu::YuzuRunner,
};

#[derive(Clone, Debug)]
pub enum CValue {
    Str(String),
    Bool(bool),
    StrArr(Vec<String>),
    OneOff(Vec<String>, usize),
    PickFile(String),
    PickFolder(String),
}

impl CValue {
    fn from_toml_value(v: &toml::Value) -> Self {
        match v {
            toml::Value::String(s) => Self::Str(s.to_owned()),
            toml::Value::Integer(i) => Self::Str(format!("{i}")),
            toml::Value::Float(f) => Self::Str(format!("{f}")),
            toml::Value::Boolean(b) => Self::Bool(*b),
            toml::Value::Datetime(d) => Self::Str(format!("{d}")),
            toml::Value::Array(a) => {
                let mut out = Vec::new();
                for i in a {
                    if let Some(s) = i.as_str() {
                        out.push(s.to_owned())
                    } else {
                        log::warn!("toml array contained value {:?}, which is not a string", i)
                    }
                }
                Self::StrArr(out)
            }
            toml::Value::Table(_) => {
                log::error!("toml config contains unexpected table");
                panic!()
            }
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            CValue::Str(s) => s.to_owned(),
            CValue::OneOff(c, i) => c[*i].clone(),
            CValue::PickFile(s) => s.to_owned(),
            CValue::PickFolder(s) => s.to_owned(),
            _ => panic!(),
        }
    }

    fn as_strarr(&self) -> Vec<String> {
        if let Self::StrArr(s) = self {
            s.to_vec()
        } else {
            panic!()
        }
    }

    fn as_hashmap(&self) -> HashMap<String, String> {
        if let Self::StrArr(s) = self {
            let mut out = HashMap::new();
            let mut first: Option<String> = None;
            for i in s {
                if let Some(fst) = first {
                    out.insert(fst, i.to_owned());
                    first = None
                } else {
                    first = Some(i.to_owned())
                }
            }
            if let Some(fst) = first {
                out.insert(fst, "".to_owned());
            }
            out
        } else {
            panic!()
        }
    }

    fn as_bool(&self) -> bool {
        if let Self::Bool(b) = self {
            *b
        } else {
            panic!()
        }
    }

    fn try_coerce(self, to_match: &Self) -> Option<Self> {
        if std::mem::discriminant(&self) == std::mem::discriminant(to_match) {
            Some(self)
        } else if let Self::OneOff(choices, _) = to_match {
            if let Self::Str(chosen) = self {
                if let Some((i, _)) = choices
                    .iter()
                    .enumerate()
                    .find(|(_, b)| b.to_string() == chosen)
                {
                    Some(Self::OneOff(choices.to_vec(), i))
                } else {
                    None
                }
            } else {
                None
            }
        } else if let Self::PickFile(_) = to_match {
            if let Self::Str(chosen) = self {
                Some(Self::PickFile(chosen))
            } else {
                None
            }
        } else if let Self::PickFolder(_) = to_match {
            if let Self::Str(chosen) = self {
                Some(Self::PickFolder(chosen))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn to_toml_value(&self) -> toml::Value {
        match self {
            CValue::Str(s) => toml::Value::String(s.to_owned()),
            CValue::Bool(b) => toml::Value::Boolean(*b),
            CValue::StrArr(arr) => toml::Value::Array(
                arr.iter()
                    .map(|a| toml::Value::String(a.to_owned()))
                    .collect(),
            ),
            CValue::OneOff(c, i) => toml::Value::String(c[*i].clone()),
            CValue::PickFile(s) => toml::Value::String(s.to_owned()),
            CValue::PickFolder(s) => toml::Value::String(s.to_owned()),
        }
    }
}

pub static CONFIG_ORDER: once_cell::sync::Lazy<Vec<(String, Vec<String>)>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            (
                "launcher:launcher".to_owned(),
                vec!["launcher:sgdb_api_key".to_owned()],
            ),
            (
                "metadata".to_owned(),
                vec![
                    "name".to_owned(),
                    "box_art".to_owned(),
                    "release_year".to_owned(),
                    "path_to_game".to_owned(),
                    "runner".to_owned(),
                ],
            ),
            (
                "general".to_owned(),
                vec![
                    #[cfg(unix)]
                    "mangohud".to_owned(),
                    #[cfg(unix)]
                    "vk_icd_loader".to_owned(),
                    #[cfg(unix)]
                    "mesa_prime".to_owned(),
                    #[cfg(unix)]
                    "nv_prime".to_owned(),
                    "env_variables".to_owned(),
                    "no_sleep_enabled".to_owned(),
                    #[cfg(unix)]
                    "gamescope".to_owned(),
                    #[cfg(unix)]
                    "gamescope_params".to_owned(),
                ],
            ),
            ("native:native".to_owned(), vec!["native:args".to_owned()]),
            (
                "ryujinx:ryujinx".to_owned(),
                vec!["ryujinx:path_to_ryujinx".to_owned()],
            ),
            #[cfg(unix)]
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
                ],
            ),
            (
                "rpcs3:rpcs3".to_owned(),
                vec!["rpcs3:path_to_rpcs3".to_owned()],
            ),
            (
                "mame:mame".to_owned(),
                vec![
                    "mame:path_to_mame".to_owned(),
                    "mame:machine_name".to_owned(),
                    "mame:fullscreen".to_owned(),
                ],
            ),
            (
                "pcsx2:pcsx2".to_owned(),
                vec![
                    "pcsx2:path_to_pcsx2".to_owned(),
                    "pcsx2:fullscreen".to_owned(),
                ],
            ),
            (
                "yuzu:yuzu".to_owned(),
                vec!["yuzu:path_to_yuzu".to_owned(), "yuzu:fullscreen".to_owned()],
            ),
        ]
    });

pub static DEFAULT_CONFIG: once_cell::sync::Lazy<HashMap<String, (String, CValue)>> =
    once_cell::sync::Lazy::new(|| {
        let mut out = HashMap::new();

        out.insert(
            "launcher:sgdb_api_key".to_owned(),
            ("SteamGridDB API key".to_owned(), CValue::Str(String::new())),
        );
        out.insert(
            "name".to_owned(),
            ("name".to_owned(), CValue::Str(String::new())),
        );
        out.insert(
            "box_art".to_owned(),
            ("box art".to_owned(), CValue::PickFile(String::new())),
        );
        out.insert(
            "release_year".to_owned(),
            ("release year".to_owned(), CValue::Str(String::new())),
        );
        out.insert(
            "path_to_game".to_owned(),
            ("path to game".to_owned(), CValue::PickFile(String::new())),
        );
        out.insert(
            "runner".to_owned(),
            (
                "runner".to_owned(),
                CValue::OneOff(
                    crate::games::RUNNERS
                        .iter()
                        .map(|a| a.to_string())
                        .collect(),
                    0,
                ),
            ),
        );
        #[cfg(unix)]
        out.insert(
            "vk_icd_loader".to_owned(),
            (
                "path to vulkan icd loader".to_owned(),
                CValue::PickFile(String::new()),
            ),
        );
        #[cfg(unix)]
        out.insert(
            "mangohud".to_owned(),
            ("mangohud".to_owned(), CValue::Bool(false)),
        );
        #[cfg(unix)]
        out.insert(
            "mesa_prime".to_owned(),
            (
                "use prime render offload (for amd gpus)".to_owned(),
                CValue::Bool(false),
            ),
        );
        #[cfg(unix)]
        out.insert(
            "nv_prime".to_owned(),
            (
                "use prime render offload (for nvidia gpus)".to_owned(),
                CValue::Bool(false),
            ),
        );
        out.insert(
            "env_variables".to_owned(),
            (
                "additional environment variables".to_owned(),
                CValue::StrArr(Vec::new()),
            ),
        );
        out.insert(
            "no_sleep_enabled".to_owned(),
            ("disable sleep".to_owned(), CValue::Bool(false)),
        );
        #[cfg(unix)]
        out.insert(
            "gamescope".to_owned(),
            ("enable gamescope".to_owned(), CValue::Bool(false)),
        );
        #[cfg(unix)]
        out.insert(
            "gamescope_params".to_owned(),
            ("gamescope parameters".to_owned(), CValue::StrArr(vec![])),
        );

        out.insert(
            "native:args".to_owned(),
            (
                "additional arguments".to_owned(),
                CValue::StrArr(Vec::new()),
            ),
        );

        out.insert(
            "ryujinx:path_to_ryujinx".to_owned(),
            (
                "path to ryujinx executable".to_owned(),
                CValue::PickFile("ryujinx".to_owned()),
            ),
        );

        #[cfg(unix)]
        {
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
        }
        out.insert(
            "rpcs3:path_to_rpcs3".to_owned(),
            ("path to rpcs3".to_owned(), CValue::PickFile("".to_owned())),
        );
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
        out.insert(
            "pcsx2:path_to_pcsx2".to_owned(),
            ("path to pcsx2".to_owned(), CValue::PickFile("".to_owned())),
        );
        out.insert(
            "pcsx2:fullscreen".to_owned(),
            ("fullscreen".to_owned(), CValue::Bool(false)),
        );
        out.insert(
            "yuzu:path_to_yuzu".to_owned(),
            ("path to yuzu".to_owned(), CValue::PickFile("".to_owned())),
        );
        out.insert(
            "yuzu:fullscreen".to_owned(),
            ("fullscreen".to_owned(), CValue::Bool(false)),
        );

        out
    });

pub fn get_default_config_with_vals(path: &std::path::Path) -> HashMap<String, (String, CValue)> {
    let mut out = DEFAULT_CONFIG.clone();
    let cfg = Cfg::from_toml(path);

    for (k, v) in cfg.0 {
        let (label, _) = out.get(&k).unwrap();
        out.insert(k, (label.to_owned(), v));
    }

    out
}

fn opt(s: String) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

#[derive(Clone, Debug)]
pub struct Cfg(pub HashMap<String, CValue>);

impl Cfg {
    pub fn minimal() -> Self {
        let mut out = HashMap::new();
        let def = DEFAULT_CONFIG.get(&"runner".to_owned()).unwrap().clone();
        out.insert("runner".to_owned(), def.1);
        Cfg(out)
    }
    pub fn from_toml(path: &std::path::Path) -> Self {
        let toml = std::fs::read_to_string(path).unwrap_or(String::new());
        let value = toml::Value::from_str(&toml[..]).unwrap();
        let tot = value.as_table().unwrap();

        let mut out = HashMap::new();

        let default = DEFAULT_CONFIG.clone();

        for (k, content) in tot.iter() {
            let prefix = if k == "metadata" || k == "general" {
                "".to_owned()
            } else {
                k.to_owned() + ":"
            };

            for (k2, c2) in content.as_table().unwrap().iter() {
                let name = prefix.clone() + k2;
                if let Some(a) = default.get(&name) {
                    if let Some(to_add) = CValue::from_toml_value(c2).try_coerce(&a.1) {
                        out.insert(name, to_add);
                    } else {
                        log::warn!("config for {name} exists but is of invalid type")
                    }
                } else {
                    log::warn!("config {name} is not known to the software")
                }
            }
        }

        Cfg(out)
    }

    pub fn to_toml(&self) -> String {
        let order = CONFIG_ORDER.clone();

        let mut table = toml::map::Map::new();

        for (name, content) in order.clone() {
            let mut itable = toml::map::Map::new();
            let name = name
                .split(':')
                .collect::<Vec<_>>()
                .last()
                .unwrap()
                .to_string();

            for label in content {
                if let Some(value) = self.0.get(&label) {
                    let label = label
                        .split(':')
                        .collect::<Vec<_>>()
                        .last()
                        .unwrap()
                        .to_string();
                    itable.insert(label, value.to_toml_value());
                }
            }
            if !itable.is_empty() {
                table.insert(name, toml::Value::Table(itable));
            }
        }
        toml::to_string_pretty(&toml::Value::Table(table)).unwrap()
    }

    fn get_or_default(&self, key: &str, default: &HashMap<String, (String, CValue)>) -> CValue {
        let Self(s) = self;
        s.get(key).unwrap_or(&default.get(key).unwrap().1).clone()
    }
    pub fn to_game(
        self,
        default: &std::path::Path,
        toml: std::path::PathBuf,
        play_time_db: &HashMap<String, std::time::Duration>,
    ) -> crate::games::Game {
        let default = get_default_config_with_vals(default);
        let box_art = self.get_or_default("box_art", &default).as_string();
        let path = self.get_or_default("path_to_game", &default).as_string();

        log::info!(
            "found config for game \"{}\"",
            self.get_or_default("name", &default).as_string()
        );
        let image = if let Ok(a) = image::io::Reader::open(box_art.clone()) {
            a.decode()
                .unwrap()
                .thumbnail(crate::IMAGE_WIDTH, crate::IMAGE_HEIGHT)
        } else {
            image::DynamicImage::new_rgba8(crate::IMAGE_WIDTH, crate::IMAGE_HEIGHT)
        }
        // .resize(200, 300, FilterType::Triangle)
        .to_rgba8();

        let runner_id = self.get_or_default("runner", &default).as_string();
        let runner = match &runner_id[..] {
            "dummy" => Box::new(DummyRunner()) as Box<dyn Runner>,
            "native" => Box::new(NativeRunner {
                path: path.clone(),
                args: self.get_or_default("native:args", &default).as_strarr(),
            }) as Box<dyn Runner>,
            "ryujinx" => Box::new(RyujinxRunner {
                path: path.clone(),
                path_to_ryujinx: self
                    .get_or_default("ryujinx:path_to_ryujinx", &default)
                    .as_string(),
            }) as Box<dyn Runner>,
            #[cfg(unix)]
            "wine" => Box::new(WineRunner {
                path: path.clone(),
                path_to_wine: self
                    .get_or_default("wine:path_to_wine", &default)
                    .as_string(),
                wineprefix: opt(self.get_or_default("wine:wineprefix", &default).as_string()),
                use_vkd3d: self.get_or_default("wine:use_vkd3d", &default).as_bool(),
                vkd3d_path: opt(self.get_or_default("wine:vkd3d_path", &default).as_string()),
                use_dxvk: self.get_or_default("wine:use_dxvk", &default).as_bool(),
                dxvk_path: opt(self.get_or_default("wine:dxvk_path", &default).as_string()),
                use_dxvk_nvapi: self
                    .get_or_default("wine:use_dxvk_nvapi", &default)
                    .as_bool(),
                dxvk_nvapi_path: opt(self
                    .get_or_default("wine:dxvk_nvapi_path", &default)
                    .as_string()),
                fsync: self.get_or_default("wine:esync", &default).as_bool(),
                esync: self.get_or_default("wine:fsync", &default).as_bool(),
                use_fsr: self.get_or_default("wine:use_fsr", &default).as_bool(),
                fsr_strength: self
                    .get_or_default("wine:fsr_strength", &default)
                    .as_string(),
            }) as Box<dyn Runner>,
            "rpcs3" => Box::new(Rpcs3Runner {
                path: path.clone(),
                path_to_rpcs3: self
                    .get_or_default("rpcs3:path_to_rpcs3", &default)
                    .as_string(),
            }),
            "mame" => Box::new(MameRunner {
                path: path.clone(),
                machine_name: self
                    .get_or_default("mame:machine_name", &default)
                    .as_string(),
                path_to_mame: self
                    .get_or_default("mame:path_to_mame", &default)
                    .as_string(),
                fullscreen: self.get_or_default("mame:fullscreen", &default).as_bool(),
            }),
            "pcsx2" => Box::new(Pcsx2Runner {
                path: path.clone(),
                path_to_pcsx2: self
                    .get_or_default("pcsx2:path_to_pcsx2", &default)
                    .as_string(),
                fullscreen: self.get_or_default("pcsx2:fullscreen", &default).as_bool(),
            }),
            "yuzu" => Box::new(YuzuRunner {
                path: path.clone(),
                path_to_yuzu: self
                    .get_or_default("yuzu:path_to_yuzu", &default)
                    .as_string(),
                fullscreen: self.get_or_default("yuzu:fullscreen", &default).as_bool(),
            }),
            _ => panic!("unknown runner"),
        };

        crate::games::Game {
            name: self.get_or_default("name", &default).as_string(),
            box_art: opt(box_art),
            release_year: isize::from_str(
                &self.get_or_default("release_year", &default).as_string()[..],
            )
            .ok(),
            image,
            path_to_game: path.into(),
            runner_id,
            runner,
            config: crate::games::Config {
                #[cfg(unix)]
                mangohud: self.get_or_default("mangohud", &default).as_bool(),
                #[cfg(unix)]
                mesa_prime: self.get_or_default("mesa_prime", &default).as_bool(),
                #[cfg(unix)]
                nv_prime: self.get_or_default("nv_prime", &default).as_bool(),
                #[cfg(unix)]
                vk_icd_loader: opt(self.get_or_default("vk_icd_loader", &default).as_string()),
                envs: self.get_or_default("env_variables", &default).as_hashmap(),
                no_sleep_enabled: self.get_or_default("no_sleep_enabled", &default).as_bool(),
                #[cfg(unix)]
                gamescope: self.get_or_default("gamescope", &default).as_bool(),
                #[cfg(unix)]
                gamescope_params: self
                    .get_or_default("gamescope_params", &default)
                    .as_strarr(),
            },

            bare_config: self,
            path_to_toml: toml.clone(),

            // process_handle: None,
            current_log: String::new(),
            // process_reader: None,
            no_sleep: None,

            time_played: play_time_db
                .get(
                    &toml
                        .file_name()
                        .unwrap_or_default()
                        .to_str()
                        .unwrap_or_default()
                        .to_owned(),
                )
                .unwrap_or(&std::time::Duration::from_secs(0))
                .clone(),
            time_started: None,

            is_running: false,
            cmd_to_run: None,
            psub_sender: None,
        }
    }
}
