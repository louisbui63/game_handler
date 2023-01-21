use std::{collections::HashMap, str::FromStr};

use crate::games::{DummyRunner, NativeRunner, Runner, RyujinxRunner, WineRunner};

#[derive(Clone)]
pub enum CValue {
    Str(String),
    Bool(bool),
    StrArr(Vec<String>),
    OneOff(Vec<String>, usize),
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

    fn as_string(&self) -> String {
        match self {
            CValue::Str(s) => s.to_owned(),
            CValue::OneOff(c, i) => c[*i].clone(),
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
        } else {
            None
        }
    }
}

pub fn get_default_config() -> HashMap<String, (String, CValue)> {
    let mut out = HashMap::new();

    out.insert(
        "name".to_owned(),
        ("name".to_owned(), CValue::Str(String::new())),
    );
    out.insert(
        "box_art".to_owned(),
        ("box art".to_owned(), CValue::Str(String::new())),
    );
    out.insert(
        "release_year".to_owned(),
        ("release year".to_owned(), CValue::Str(String::new())),
    );
    out.insert(
        "path_to_game".to_owned(),
        ("path to game".to_owned(), CValue::Str(String::new())),
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
    out.insert(
        "vk_icd_loader".to_owned(),
        (
            "path to vulkan icd loader".to_owned(),
            CValue::Str(String::new()),
        ),
    );
    out.insert(
        "mangohud".to_owned(),
        ("mangohud".to_owned(), CValue::Bool(false)),
    );
    out.insert(
        "mesa_prime".to_owned(),
        (
            "use prime render offload (for amd gpus)".to_owned(),
            CValue::Bool(false),
        ),
    );
    out.insert(
        "nv_prime".to_owned(),
        (
            "use prime render offload (for nvidia gpus)".to_owned(),
            CValue::Bool(false),
        ),
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
            CValue::Str("ryujinx".to_owned()),
        ),
    );

    out.insert(
        "wine:path_to_wine".to_owned(),
        (
            "path to wine executable".to_owned(),
            CValue::Str("wine".to_owned()),
        ),
    );
    out.insert(
        "wine:use_vkd3d".to_owned(),
        ("enable vkd3d".to_owned(), CValue::Bool(false)),
    );
    out.insert(
        "wine:vkd3d_path".to_owned(),
        ("path to vkd3d".to_owned(), CValue::Str("".to_owned())),
    );
    out.insert(
        "wine:use_dxvk".to_owned(),
        ("enable dxvk".to_owned(), CValue::Bool(false)),
    );
    out.insert(
        "wine:dxvk_path".to_owned(),
        ("path to dxvk".to_owned(), CValue::Str("".to_owned())),
    );
    out.insert(
        "wine:use_dxvk_nvapi".to_owned(),
        ("enable dxvk_nvapi".to_owned(), CValue::Bool(false)),
    );
    out.insert(
        "wine:dxvk_nvapi_path".to_owned(),
        ("path to dxvk_nvapi".to_owned(), CValue::Str("".to_owned())),
    );
    out.insert(
        "wine:esync".to_owned(),
        ("enable esync".to_owned(), CValue::Bool(false)),
    );
    out.insert(
        "wine:fsync".to_owned(),
        ("enable fsync".to_owned(), CValue::Bool(false)),
    );

    out
}

fn opt(s: String) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}
pub struct Cfg(pub HashMap<String, CValue>);

impl Cfg {
    pub fn from_toml(path: &str) -> Self {
        let toml = std::fs::read_to_string(path).unwrap_or(String::new());
        let value = toml::Value::from_str(&toml[..]).unwrap();
        let tot = value.as_table().unwrap();

        let mut out = HashMap::new();

        let default = get_default_config();

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
    fn get_or_default(&self, key: &str, default: &HashMap<String, (String, CValue)>) -> CValue {
        if let Self(s) = self {
            s.get(key).unwrap_or(&default.get(key).unwrap().1).clone()
        } else {
            unreachable!()
        }
    }
    pub fn to_game(self) -> crate::games::Game {
        let default = get_default_config();
        let box_art = self.get_or_default("box_art", &default).as_string();
        let path = self.get_or_default("path_to_game", &default).as_string();

        let image = image::io::Reader::open(box_art.clone())
            .unwrap()
            .decode()
            .unwrap()
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
            "wine" => Box::new(WineRunner {
                path: path.clone(),
                path_to_wine: self
                    .get_or_default("wine:path_to_wine", &default)
                    .as_string(),
                wineprefix: None,
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
            }) as Box<dyn Runner>,
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

            path_to_game: path,

            runner_id,

            runner,

            config: crate::games::Config {
                mangohud: self.get_or_default("mangohud", &default).as_bool(),
                mesa_prime: self.get_or_default("mesa_prime", &default).as_bool(),
                nv_prime: self.get_or_default("nv_prime", &default).as_bool(),
                vk_icd_loader: opt(self.get_or_default("vk_icd_loader", &default).as_string()),
            },

            bare_config: self,
        }
    }
}
