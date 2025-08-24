use crate::games::{RunnerInstance, RUNNERS};
use std::{collections::HashMap, str::FromStr};

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

    pub fn as_strarr(&self) -> Vec<String> {
        if let Self::StrArr(s) = self {
            s.to_vec()
        } else {
            panic!()
        }
    }

    pub fn as_hashmap(&self) -> HashMap<String, String> {
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

    pub fn as_bool(&self) -> bool {
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

pub static CONFIG_ORDER: std::sync::LazyLock<Vec<(String, Vec<String>)>> =
    std::sync::LazyLock::new(|| {
        let mut out = vec![
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
                    #[cfg(unix)]
                    "gamemode".to_owned(),
                ],
            ),
        ];
        for runner in RUNNERS.iter() {
            let co = runner.get_config_order();
            if !co.1.is_empty() {
                out.push(co)
            }
        }
        out
    });

pub static DEFAULT_CONFIG: std::sync::LazyLock<HashMap<String, (String, CValue)>> =
    std::sync::LazyLock::new(|| {
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
                        .map(|a| a.get_runner_id())
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
        #[cfg(unix)]
        out.insert(
            "gamemode".to_owned(),
            ("gamemode".to_owned(), CValue::Bool(true)),
        );

        for runner in RUNNERS.iter() {
            out.extend(runner.get_default_config())
        }
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

pub fn opt(s: String) -> Option<String> {
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
        let toml = std::fs::read_to_string(path).unwrap_or_default();
        let value: toml::Value = toml::from_str(&toml[..]).unwrap();
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

    pub fn get_or_default(&self, key: &str, default: &HashMap<String, (String, CValue)>) -> CValue {
        let Self(s) = self;
        s.get(key)
            .unwrap_or(
                &default
                    .get(key)
                    .unwrap_or_else(|| {
                        log::error!("missing key {}", key);
                        panic!()
                    })
                    .1,
            )
            .clone()
    }
    pub fn into_game(
        self,
        default: &std::path::Path,
        toml: std::path::PathBuf,
        play_time_db: &HashMap<String, std::time::Duration>,
        play_time_ty_db: &HashMap<String, std::time::Duration>,
    ) -> crate::games::Game {
        let default = get_default_config_with_vals(default);
        let box_art = self.get_or_default("box_art", &default).as_string();
        let path = self.get_or_default("path_to_game", &default).as_string();

        log::info!(
            "found config for game \"{}\"",
            self.get_or_default("name", &default).as_string()
        );

        let runner_id = self.get_or_default("runner", &default).as_string();
        let runner = RUNNERS
            .iter()
            .find(|runner| runner.get_runner_id() == runner_id)
            .expect("unknown runner")
            .create_instance(&self, path.clone(), &default);

        crate::games::Game {
            name: self.get_or_default("name", &default).as_string(),
            box_art: opt(box_art),
            release_year: isize::from_str(
                &self.get_or_default("release_year", &default).as_string()[..],
            )
            .ok(),
            path_to_game: path.into(),
            runner_id,
            runner_instance: runner,
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
                #[cfg(unix)]
                gamemode: self.get_or_default("gamemode", &default).as_bool(),
            },

            bare_config: self,
            path_to_toml: toml.clone(),

            // process_handle: None,
            current_log: String::new(),
            // process_reader: None,
            no_sleep: None,

            time_played: *play_time_db
                .get(
                    &toml
                        .file_name()
                        .unwrap_or_default()
                        .to_str()
                        .unwrap_or_default()
                        .to_owned(),
                )
                .unwrap_or(&std::time::Duration::from_secs(0)),
            time_played_this_year: *play_time_ty_db
                .get(
                    &toml
                        .file_name()
                        .unwrap_or_default()
                        .to_str()
                        .unwrap_or_default()
                        .to_owned(),
                )
                .unwrap_or(&std::time::Duration::from_secs(0)),
            time_started: None,

            is_running: false,
            cmd_to_run: None,
            psub_sender: None,

            managed_processes: Vec::new(),
        }
    }
}
