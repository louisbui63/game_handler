mod config;
mod games;
mod mame;
mod native;
mod rpcs3;
mod ryujinx;
mod wine;

use std::collections::HashMap;
use std::io::{BufRead, Read, Seek};

use config::{get_default_config_with_vals, CValue, Cfg};
use games::Game;
use iced::executor;
use iced::widget::{button, column, container, radio, row};
use iced::{Application, Command, Element, Length, Settings, Theme};
use iced_aw::tabs::TabBarStyles;
use iced_aw::{TabBar, TabLabel};

pub const WIDGET_HEIGHT: u16 = 30;
pub const IMAGE_WIDTH: u32 = 200;
pub const IMAGE_HEIGHT: u32 = 300;

pub static DIRS: once_cell::sync::Lazy<directories::ProjectDirs> =
    once_cell::sync::Lazy::new(|| {
        directories::ProjectDirs::from("com", "louisbui63", "game_handler")
            .expect("couldn't find home directory")
    });

fn main() -> iced::Result {
    pretty_env_logger::init();
    log::info!("Starting...");
    log::info!("checking config directory {:?}...", DIRS.config_dir());
    if let Err(e) = std::fs::create_dir_all(DIRS.config_dir().join("games")) {
        log::error!("couldn't ensure the config directory existence or integrity : {e}");
        panic!()
    }
    log::info!("checking data directory {:?}...", DIRS.data_dir());
    if let Err(e) = std::fs::create_dir_all(DIRS.data_dir().join("banners")) {
        log::error!("couldn't ensure the data directory existence or integrity : {e}");
        panic!()
    }
    MainGUI::run(Settings {
        exit_on_close_request: true,
        ..Default::default()
    })
}

fn get_widget(
    v: &config::CValue,
    label: String,
    k: String,
    uses_default: bool,
) -> iced::widget::Row<'_, Message> {
    match v {
        config::CValue::Str(s) => row![
            iced::widget::text(label).width(Length::FillPortion(3)),
            iced::widget::text_input("", s)
                .on_input({
                    let k1 = k.clone();
                    move |a| Message::SettingChanged(k1.clone(), CValue::Str(a))
                })
                .width(Length::FillPortion(3)),
            iced::widget::toggler(None, uses_default, move |a| {
                Message::SettingDefaultChanged(k.clone(), a)
            })
            .width(Length::FillPortion(1)),
        ]
        .height(Length::Fixed(WIDGET_HEIGHT as f32)),
        config::CValue::Bool(b) => row![
            iced::widget::text(label).width(Length::FillPortion(3)),
            iced::widget::toggler(None, *b, {
                let k1 = k.clone();
                move |a| Message::SettingChanged(k1.clone(), CValue::Bool(a))
            })
            .width(Length::FillPortion(3)),
            iced::widget::toggler(None, uses_default, move |a| {
                Message::SettingDefaultChanged(k.clone(), a)
            })
            .width(Length::FillPortion(1)),
        ]
        .height(Length::Fixed(WIDGET_HEIGHT as f32)),
        config::CValue::StrArr(arr) => {
            // log::error!("Feature StrArr() not yet available in config display");
            let mut col = Vec::new();
            for i in 0..(arr.len() / 2) + 1 {
                let mut row: Vec<Element<_>>/*: Vec<Element<'_, Message, _>>*/ = Vec::new();
                row.push(
                    iced::widget::text_input(
                        "",
                        if 2 * i < arr.len() {
                            &arr[2 * i][..]
                        } else {
                            ""
                        },
                    )
                    .on_input({
                        let k1 = k.clone();
                        move |a| {
                            let mut oct = arr.clone();
                            if 2 * i < oct.len() {
                                oct[2 * i] = a;
                            } else {
                                oct.push(a)
                            }
                            Message::SettingChanged(k1.clone(), CValue::StrArr(oct))
                        }
                    })
                    .width(Length::FillPortion(if 2 * i == arr.len() { 10 } else { 9 }))
                    .into(),
                );
                if 2 * i != arr.len() {
                    row.push(
                        iced::widget::button("x")
                            .on_press(Message::SettingChanged(
                                k.clone(),
                                CValue::StrArr({
                                    let mut oct = arr.clone();
                                    oct.remove(2 * i);
                                    oct
                                }),
                            ))
                            .width(Length::FillPortion(1))
                            .into(),
                    );
                }
                if 2 * i < arr.len() {
                    row.push(
                        iced::widget::text_input(
                            "",
                            if 2 * i + 1 < arr.len() {
                                &arr[2 * i + 1][..]
                            } else {
                                ""
                            },
                        )
                        .on_input({
                            let k1 = k.clone();
                            move |a| {
                                let mut oct = arr.clone();
                                if 2 * i + 1 < oct.len() {
                                    oct[2 * i + 1] = a;
                                } else {
                                    oct.push(a)
                                }
                                Message::SettingChanged(k1.clone(), CValue::StrArr(oct))
                            }
                        })
                        .width(Length::FillPortion(if 2 * i + 1 == arr.len() {
                            10
                        } else {
                            9
                        }))
                        .into(), //as Element<'_, Message, _>,
                    );
                    if 2 * i + 1 != arr.len() {
                        row.push(
                            iced::widget::button("x")
                                .on_press(Message::SettingChanged(
                                    k.clone(),
                                    CValue::StrArr({
                                        let mut oct = arr.clone();
                                        oct.remove(2 * i + 1);
                                        oct
                                    }),
                                ))
                                .width(Length::FillPortion(1))
                                .into(),
                        );
                    }
                } else {
                    row.push(iced::widget::Space::with_width(Length::FillPortion(10)).into())
                }
                col.push(
                    iced::widget::Row::with_children(row)
                        .height(Length::Fixed(WIDGET_HEIGHT as f32))
                        .into(),
                );
            }
            let col = iced::widget::Column::with_children(col);
            row![
                iced::widget::text(label).width(Length::FillPortion(3)),
                col.width(Length::FillPortion(3)),
                iced::widget::toggler(None, uses_default, move |a| {
                    Message::SettingDefaultChanged(k.clone(), a)
                })
                .width(Length::FillPortion(1)),
            ]
            .into()
        }
        config::CValue::OneOff(l, s) => row![
            iced::widget::text(label).width(Length::FillPortion(3)),
            iced::widget::pick_list(l, Some(l[*s].clone()), {
                let k1 = k.clone();
                move |a| {
                    Message::SettingChanged(
                        k1.clone(),
                        CValue::OneOff(
                            l.clone(),
                            l.iter()
                                .enumerate()
                                .find(|(_, b)| b.to_string() == a)
                                .unwrap()
                                .0,
                        ),
                    )
                }
            })
            .width(Length::FillPortion(3)),
            iced::widget::toggler(None, uses_default, move |a| {
                Message::SettingDefaultChanged(k.clone(), a)
            })
            .width(Length::FillPortion(1)),
        ]
        .height(Length::Fixed(WIDGET_HEIGHT as f32)),
        config::CValue::PickFile(s) => if k == "box_art" {
            row![
                iced::widget::text(label).width(Length::FillPortion(6)),
                iced::widget::text_input("", s)
                    .on_input({
                        let k1 = k.clone();
                        move |a| Message::SettingChanged(k1.clone(), CValue::PickFile(a))
                    })
                    .width(Length::FillPortion(4)),
                iced::widget::button(iced_aw::native::icon_text::IconText::new(
                    iced_aw::graphics::icons::Icon::Grid
                ))
                .on_press(Message::SteamGridDb)
                .width(Length::FillPortion(1)),
                iced::widget::button(iced_aw::native::icon_text::IconText::new(
                    iced_aw::graphics::icons::Icon::Folder
                ))
                .on_press(Message::FilePicker(k.clone()))
                .width(Length::FillPortion(1)),
                iced::widget::toggler(None, uses_default, move |a| {
                    Message::SettingDefaultChanged(k.clone(), a)
                })
                .width(Length::FillPortion(2)),
            ]
        } else {
            row![
                iced::widget::text(label).width(Length::FillPortion(6)),
                iced::widget::text_input("", s)
                    .on_input({
                        let k1 = k.clone();
                        move |a| Message::SettingChanged(k1.clone(), CValue::PickFile(a))
                    })
                    .width(Length::FillPortion(5)),
                iced::widget::button(iced_aw::native::icon_text::IconText::new(
                    iced_aw::graphics::icons::Icon::Folder
                ))
                .on_press(Message::FilePicker(k.clone()))
                .width(Length::FillPortion(1)),
                iced::widget::toggler(None, uses_default, move |a| {
                    Message::SettingDefaultChanged(k.clone(), a)
                })
                .width(Length::FillPortion(2)),
            ]
        }
        .height(Length::Fixed(WIDGET_HEIGHT as f32)),
        CValue::PickFolder(s) => row![
            iced::widget::text(label).width(Length::FillPortion(6)),
            iced::widget::text_input("", s)
                .on_input({
                    let k1 = k.clone();
                    move |a| Message::SettingChanged(k1.clone(), CValue::PickFolder(a))
                })
                .width(Length::FillPortion(5)),
            iced::widget::button(iced_aw::native::icon_text::IconText::new(
                iced_aw::graphics::icons::Icon::Folder
            ))
            .width(Length::FillPortion(1)),
            iced::widget::toggler(None, uses_default, move |a| {
                Message::SettingDefaultChanged(k.clone(), a)
            })
            .width(Length::FillPortion(2)),
        ]
        .height(Length::Fixed(WIDGET_HEIGHT as f32)),
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(usize)]
enum GridStatus {
    GamesGrid,
    GlobalSettings,
    AddGame,
    GamesSettings,
    Logs,
}

impl std::convert::TryFrom<usize> for GridStatus {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::GamesGrid),
            1 => Ok(Self::GlobalSettings),
            2 => Ok(Self::AddGame),
            3 => Ok(Self::GamesSettings),
            4 => Ok(Self::Logs),
            _ => Err(()),
        }
    }
}

struct MainGUI {
    theme: Theme,
    games: Vec<Game>,
    selected: Option<usize>,
    grid_status: GridStatus,
    temp_settings: Option<Cfg>,
    default_config: HashMap<String, (String, config::CValue)>,
    steam_grid_db: bool,
    sgdb_images: Vec<(
        steamgriddb_api::images::Image,
        image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    )>,
    sgdb_selected: Option<usize>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ThemeType {
    Light,
    Dark,
}

#[derive(Debug, Clone)]
enum Message {
    ThemeChanged(ThemeType),
    GameSelected(usize),
    RunSelected,
    RunSubcommandSelected(String),
    ToggleSettings,
    ToggleGlobalSettings,
    ToggleAddGame,
    SettingChanged(String, CValue),
    SettingDefaultChanged(String, bool),
    FilePicker(String),
    FolderPicker(String),
    SetGridStatus(GridStatus),
    ApplySettings,
    ApplyCloseSettings,
    MonotonicClock,
    KillSelected,
    SteamGridDb,
    SGDBThumbSelected(usize),
    ApplySGDB,
    CancelSGDB,
}

struct ButtonStyle();
impl iced::widget::button::StyleSheet for ButtonStyle {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(iced::Color::TRANSPARENT)),
            text_color: match style {
                Theme::Light => iced::Color::BLACK,
                Theme::Dark => iced::Color::WHITE,
                Theme::Custom(_) => iced::color!(255, 0, 0),
            },
            ..Default::default()
        }
    }
}

impl Application for MainGUI {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let mut games: Vec<_> = std::fs::read_dir(DIRS.config_dir().join("games"))
            .unwrap()
            .map(|a| {
                Game::from_toml(
                    a.unwrap().path().to_str().unwrap(),
                    DIRS.config_dir().join("settings.toml").to_str().unwrap(),
                )
            })
            .collect();
        //alphabetic sort
        games.sort_by_key(|a| a.name.clone());

        (
            MainGUI {
                theme: Theme::Dark,
                games,
                selected: None,
                default_config: get_default_config_with_vals(
                    DIRS.config_dir().join("settings.toml").to_str().unwrap(),
                ),
                temp_settings: None,
                grid_status: GridStatus::GamesGrid,
                steam_grid_db: false,
                sgdb_images: vec![],
                sgdb_selected: None,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Game Handler")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ThemeChanged(theme) => {
                self.theme = match theme {
                    ThemeType::Light => Theme::Light,
                    ThemeType::Dark => Theme::Dark,
                };

                Command::none()
            }
            Message::GameSelected(id) => {
                self.selected = Some(id);
                Command::none()
            }
            Message::RunSelected => {
                if let Some(i) = self.selected {
                    self.games[i].process_reader = None;
                    self.games[i].current_log.clear();
                    self.games[i].run();
                }
                Command::none()
            }
            Message::ToggleSettings => {
                if let GridStatus::GamesGrid = self.grid_status {
                    self.grid_status = GridStatus::GamesSettings;
                    self.temp_settings =
                        Some(self.games[self.selected.unwrap()].bare_config.clone())
                } else {
                    self.grid_status = GridStatus::GamesGrid;
                }
                Command::none()
            }
            Message::SettingChanged(s1, s2) => {
                self.temp_settings.as_mut().unwrap().0.insert(s1, s2);
                Command::none()
            }
            Message::SettingDefaultChanged(s, b) => {
                if b {
                    let _ = self.temp_settings.as_mut().unwrap().0.remove(&s);
                } else {
                    self.temp_settings
                        .as_mut()
                        .unwrap()
                        .0
                        .insert(s.clone(), self.default_config.get(&s).unwrap().1.clone());
                }
                Command::none()
            }
            Message::ApplySettings | Message::ApplyCloseSettings => {
                match self.grid_status {
                    GridStatus::GamesGrid => unreachable!(),
                    GridStatus::Logs => unreachable!(),
                    GridStatus::GamesSettings => {
                        let path = self.games[self.selected.unwrap()].path_to_toml.clone();
                        self.games[self.selected.unwrap()] =
                            self.temp_settings.as_ref().unwrap().clone().to_game(
                                DIRS.config_dir().join("settings.toml").to_str().unwrap(),
                                path.clone(),
                            );

                        let to_write = self.temp_settings.as_ref().unwrap().to_toml();
                        use std::io::prelude::*;
                        std::fs::File::create(path)
                            .unwrap()
                            .write_all(to_write.as_bytes())
                            .unwrap();
                    }
                    GridStatus::GlobalSettings => {
                        let path = DIRS
                            .config_dir()
                            .join("settings.toml")
                            .to_str()
                            .unwrap()
                            .to_owned();
                        let to_write = self.temp_settings.as_ref().unwrap().to_toml();
                        use std::io::prelude::*;
                        std::fs::File::create(path)
                            .unwrap()
                            .write_all(to_write.as_bytes())
                            .unwrap();

                        self.default_config = config::get_default_config_with_vals(
                            DIRS.config_dir().join("settings.toml").to_str().unwrap(),
                        );

                        for i in 0..self.games.len() {
                            self.games[i] = self.games[i].bare_config.clone().to_game(
                                DIRS.config_dir().join("settings.toml").to_str().unwrap(),
                                self.games[i].path_to_toml.clone(),
                            );
                        }
                    }
                    GridStatus::AddGame => {
                        let random_id: u16 = rand::random();
                        let name = make_path_proof(
                            self.temp_settings
                                .as_ref()
                                .unwrap()
                                .0
                                .get("name")
                                .map(|a| a.as_string())
                                .unwrap_or(String::new()),
                        );
                        let path = DIRS
                            .config_dir()
                            .join("games")
                            .join(random_id.to_string() + &name[..] + ".toml")
                            .to_str()
                            .unwrap()
                            .to_owned();
                        let cfg = self.temp_settings.as_mut().unwrap();
                        self.games.push(cfg.clone().to_game(
                            DIRS.config_dir().join("settings.toml").to_str().unwrap(),
                            path.clone(),
                        ));

                        let to_write = self.temp_settings.as_ref().unwrap().to_toml();
                        use std::io::prelude::*;
                        std::fs::File::create(path)
                            .unwrap()
                            .write_all(to_write.as_bytes())
                            .unwrap();
                    }
                }
                if let Message::ApplyCloseSettings = message {
                    self.grid_status = GridStatus::GamesGrid;
                }
                Command::none()
            }
            Message::ToggleGlobalSettings => {
                if let GridStatus::GamesGrid = self.grid_status {
                    self.temp_settings = Some(config::Cfg::from_toml(
                        DIRS.config_dir().join("settings.toml").to_str().unwrap(),
                    ));
                    self.grid_status = GridStatus::GlobalSettings;
                } else {
                    self.grid_status = GridStatus::GamesGrid;
                }
                Command::none()
            }
            Message::ToggleAddGame => {
                if let GridStatus::GamesGrid = self.grid_status {
                    self.temp_settings = Some(config::Cfg::minimal());
                    self.grid_status = GridStatus::AddGame;
                } else {
                    self.grid_status = GridStatus::GamesGrid;
                }
                Command::none()
            }
            Message::SetGridStatus(status) => {
                if self.grid_status != status {
                    match status {
                        GridStatus::GamesSettings => {
                            self.temp_settings =
                                Some(self.games[self.selected.unwrap()].bare_config.clone())
                        }
                        GridStatus::GlobalSettings => {
                            self.temp_settings = Some(config::Cfg::from_toml(
                                DIRS.config_dir().join("settings.toml").to_str().unwrap(),
                            ))
                        }
                        GridStatus::AddGame => self.temp_settings = Some(config::Cfg::minimal()),
                        _ => {}
                    }
                    self.grid_status = status;
                };
                Command::none()
            }
            Message::FilePicker(s) => {
                let res = rfd::FileDialog::new().pick_file();
                if let Some(p) = res {
                    self.temp_settings
                        .as_mut()
                        .unwrap()
                        .0
                        .insert(s, config::CValue::PickFile(p.to_str().unwrap().to_owned()));
                }

                Command::none()
            }
            Message::FolderPicker(s) => {
                let res = rfd::FileDialog::new().pick_folder();
                if let Some(p) = res {
                    self.temp_settings.as_mut().unwrap().0.insert(
                        s,
                        config::CValue::PickFolder(p.to_str().unwrap().to_owned()),
                    );
                }

                Command::none()
            }
            Message::RunSubcommandSelected(s) => {
                if let Some(i) = self.selected {
                    self.games[i].process_reader = None;
                    self.games[i].current_log.clear();
                    self.games[i].run_subcommand(s);
                }
                Command::none()
            }
            Message::MonotonicClock => {
                for g in &mut self.games {
                    if let Some(ph) = &mut g.process_handle {
                        if let Some(out) = ph.stdout.take() {
                            g.process_reader = Some(std::io::BufReader::new(
                                timeout_readwrite::TimeoutReader::new(
                                    out,
                                    std::time::Duration::from_millis(1),
                                ),
                            ));
                        }
                        if let Some(reader) = &mut g.process_reader {
                            for _ in 0..100 {
                                let mut buf = Vec::with_capacity(512);
                                if let Err(_) = reader.read_until(0x0A, &mut buf) {
                                    break;
                                }
                                g.current_log
                                    .push_str(&String::from_utf8_lossy(buf.as_slice())[..]);
                            }
                            if let Some(_) = ph.poll() {
                                let mut buf = Vec::with_capacity(2048);
                                if let Ok(_) = reader.read_to_end(&mut buf) {
                                    g.current_log
                                        .push_str(&String::from_utf8_lossy(buf.as_slice())[..]);
                                }
                                g.process_handle = None;
                                g.process_reader = None;
                            }
                        }
                    }
                }
                Command::none()
            }
            Message::KillSelected => {
                if let Some(i) = self.selected {
                    if let Some(ph) = &mut self.games[i].process_handle {
                        ph.kill().unwrap();
                    }
                    if let Some(reader) = &mut self.games[i].process_reader {
                        let mut buf = Vec::with_capacity(2048);
                        if let Ok(_) = reader.read_to_end(&mut buf) {
                            self.games[i]
                                .current_log
                                .push_str(&String::from_utf8_lossy(buf.as_slice())[..]);
                        }
                    }
                    self.games[i].process_handle = None;
                    self.games[i].process_reader = None;
                }
                Command::none()
            }
            Message::SteamGridDb => {
                self.steam_grid_db = true;
                let title = self
                    .temp_settings
                    .as_ref()
                    .unwrap()
                    .0
                    .get("name")
                    .unwrap_or(&CValue::Str("".to_owned()))
                    .as_string();
                use steamgriddb_api::query_parameters::QueryType::Grid;
                use steamgriddb_api::Client;
                let client = Client::new("2b8131fea6e42e6b8fa178b9a86e6499");
                let games = tokio::runtime::Handle::current()
                    .block_on(client.search(&title[..]))
                    .unwrap();
                let first_game = games.iter().next().ok_or("No games found").unwrap();
                let images = tokio::runtime::Handle::current()
                    .block_on(client.get_images_for_id(
                        first_game.id,
                        &Grid(Some(
                            steamgriddb_api::query_parameters::GridQueryParameters {
                                dimentions: Some(&[
                                    steamgriddb_api::query_parameters::GridDimentions::D600x900,
                                ]),
                                ..std::default::Default::default()
                            },
                        )),
                    ))
                    .unwrap();

                let mut out = vec![];
                for i in images {
                    if let Ok(resp) = reqwest::blocking::get(i.thumb.clone()) {
                        if resp.status() == reqwest::StatusCode::OK {
                            out.push((
                                i,
                                image::load_from_memory(&resp.bytes().unwrap())
                                    .unwrap()
                                    .to_rgba8(),
                            ))
                        }
                    }
                }
                self.sgdb_images = out;
                Command::none()
            }
            Message::SGDBThumbSelected(i) => {
                self.sgdb_selected = Some(i);
                Command::none()
            }
            Message::ApplySGDB => {
                let url = &self.sgdb_images[self.sgdb_selected.unwrap()].0.url;
                let name = self.sgdb_images[self.sgdb_selected.unwrap()]
                    .0
                    .id
                    .to_string()
                    + match &self.sgdb_images[self.sgdb_selected.unwrap()].0.mime {
                        steamgriddb_api::images::MimeTypes::Default(tp) => match tp {
                            steamgriddb_api::query_parameters::MimeType::Png => ".png",
                            steamgriddb_api::query_parameters::MimeType::Jpeg => ".jpeg",
                            steamgriddb_api::query_parameters::MimeType::Webp => ".webp",
                        },
                        _ => unreachable!(),
                    };

                if let Ok(resp) = reqwest::blocking::get(url) {
                    if resp.status() == reqwest::StatusCode::OK {
                        use std::io::prelude::*;
                        let path = DIRS.data_dir().join("banners").join(name);
                        std::fs::File::create(path.clone())
                            .unwrap()
                            .write_all(&resp.bytes().unwrap())
                            .unwrap();
                        self.temp_settings.as_mut().unwrap().0.insert(
                            "box_art".to_owned(),
                            CValue::PickFile(path.to_str().unwrap().to_owned()),
                        );
                    }
                }

                self.sgdb_selected = None;
                self.steam_grid_db = false;
                self.sgdb_images.clear();
                Command::none()
            }
            Message::CancelSGDB => {
                self.sgdb_selected = None;
                self.steam_grid_db = false;
                self.sgdb_images.clear();
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let MainGUI { .. } = self;

        let mut top_bar = TabBar::new(self.grid_status as usize, |a| {
            Message::SetGridStatus(a.try_into().unwrap())
        })
        .style(TabBarStyles::Blue)
        .push(TabLabel::Text("Grid View".to_owned()))
        .push(TabLabel::Text("Global Settings".to_owned()))
        .push(TabLabel::Text("Add Game".to_owned()))
        .width(Length::FillPortion(3));

        let mut resizer = 0;
        if let Some(_) = self.selected {
            top_bar = top_bar
                .push(TabLabel::Text("Game Settings".to_owned()))
                .push(TabLabel::Text("Logs".to_owned()))
                .width(Length::FillPortion(5));
            resizer = 2;
        }

        let run_module: iced::Element<_> = if let Some(g) = self.selected {
            row![
                if let None = self.games[g].process_handle {
                    iced::widget::button(iced::widget::text("run")).on_press(Message::RunSelected)
                } else {
                    iced::widget::button(iced::widget::text("kill")).on_press(Message::KillSelected)
                },
                iced::widget::pick_list(self.games[g].get_subcommands(), None, |i| {
                    Message::RunSubcommandSelected(i)
                }) // .width(Length::Units(30))
            ]
            .align_items(iced::Alignment::End)
            .width(Length::FillPortion(2))
            .into()
        } else {
            iced::widget::Space::with_width(Length::FillPortion(2)).into()
        };

        let top_bar = row![
            top_bar,
            iced::widget::Space::with_width(if resizer == 2 {
                Length::Fixed(0.)
            } else {
                Length::FillPortion(2 - resizer)
            }),
            run_module,
        ]
        .align_items(iced::Alignment::End);

        let choose_theme = [ThemeType::Light, ThemeType::Dark].iter().fold(
            column!["Choose a theme:"].spacing(10),
            |column, option| {
                column.push(radio(
                    format!("{:?}", option),
                    *option,
                    Some(*option),
                    Message::ThemeChanged,
                ))
            },
        );

        let ge: Element<Message> = match self.grid_status {
            GridStatus::GamesGrid => {
                let image_size = IMAGE_WIDTH as u16;

                let mut grid: iced_aw::Grid<Message, _, _> =
                    iced_aw::Grid::with_column_width(image_size as f32 + 20.);
                for (i, g) in self.games.iter().enumerate() {
                    grid.insert::<Element<Message>>(
                        iced::widget::button(
                            iced::widget::column(vec![
                                iced::widget::image(iced::widget::image::Handle::from_pixels(
                                    g.image.width(),
                                    g.image.height(),
                                    g.image.as_raw().clone(),
                                ))
                                .width(Length::Fixed(image_size as f32))
                                .into(),
                                iced::widget::text(g.name.clone()).into(),
                            ])
                            .align_items(iced::Alignment::Center),
                        )
                        .on_press(Message::GameSelected(i))
                        .style(if Some(i) != self.selected {
                            iced::theme::Button::Custom(Box::new(ButtonStyle()))
                        } else {
                            iced::theme::Button::default()
                        })
                        .into(),
                    );
                }
                grid.into()
            }
            GridStatus::GamesSettings => {
                let mut options = column![];

                let selected = &self.games[self.selected.unwrap()];

                let runner = selected.runner_id.clone();

                for (t, cat) in crate::config::get_config_order() {
                    let s = t.split(':').collect::<Vec<_>>();
                    if s.len() == 1 {
                        options = options.push(iced::widget::text(t).size(30));
                    } else if s[0] == runner {
                        options = options.push(iced::widget::text(s[1]).size(30));
                    }
                    for k in cat {
                        let i = self.default_config.get(&k).unwrap(); //expect(&format!("{k}")[..]);
                        let s = k.split(':').collect::<Vec<_>>();
                        if s.len() == 1 || s[0] == runner {
                            let (v, uses_default) =
                                if let Some(v) = self.temp_settings.as_ref().unwrap().0.get(&k) {
                                    (v, false)
                                } else {
                                    (&i.1, true)
                                };
                            let label = i.0.clone() + " : ";
                            options = options.push(get_widget(v, label, k, uses_default))
                        }
                    }
                }

                options = options.push(
                    row![
                        iced::widget::button(iced::widget::text("Apply"))
                            .on_press(Message::ApplySettings),
                        iced::widget::button(iced::widget::text("Cancel"))
                            .on_press(Message::ToggleSettings),
                        iced::widget::button(iced::widget::text("Ok"))
                            .on_press(Message::ApplyCloseSettings),
                    ]
                    .align_items(iced::Alignment::End),
                );

                options.into()
            }
            GridStatus::GlobalSettings => {
                let mut options = column![choose_theme];

                for (t, cat) in crate::config::get_config_order() {
                    let s = t.split(':').collect::<Vec<_>>();
                    if s.len() == 1 {
                        options = options.push(iced::widget::text(t).size(30));
                    } else {
                        options = options.push(iced::widget::text(s[1]).size(30));
                    }
                    for k in cat {
                        let i = self.default_config.get(&k).unwrap(); //expect(&format!("{k}")[..]);
                                                                      // let s = k.split(':').collect::<Vec<_>>();
                        let (v, uses_default) =
                            if let Some(v) = self.temp_settings.as_ref().unwrap().0.get(&k) {
                                (v, false)
                            } else {
                                (&i.1, true)
                            };
                        let label = i.0.clone() + " : ";
                        options = options.push(get_widget(v, label, k, uses_default))
                    }
                }

                options = options.push(
                    row![
                        iced::widget::button(iced::widget::text("Apply"))
                            .on_press(Message::ApplySettings),
                        iced::widget::button(iced::widget::text("Cancel"))
                            .on_press(Message::ToggleGlobalSettings),
                        iced::widget::button(iced::widget::text("Ok"))
                            .on_press(Message::ApplyCloseSettings),
                    ]
                    .align_items(iced::Alignment::End),
                );
                options.into()
            }
            GridStatus::AddGame => {
                let mut options = column![];

                let runner = self
                    .temp_settings
                    .as_ref()
                    .unwrap()
                    .0
                    .get("runner")
                    .unwrap()
                    .as_string();

                for (t, cat) in crate::config::get_config_order() {
                    let s = t.split(':').collect::<Vec<_>>();
                    if s.len() == 1 {
                        options = options.push(iced::widget::text(t).size(30));
                    } else if s[0] == runner {
                        options = options.push(iced::widget::text(s[1]).size(30));
                    }
                    for k in cat {
                        let i = self.default_config.get(&k).unwrap(); //expect(&format!("{k}")[..]);
                        let s = k.split(':').collect::<Vec<_>>();
                        if s.len() == 1 || s[0] == runner {
                            let (v, uses_default) =
                                if let Some(v) = self.temp_settings.as_ref().unwrap().0.get(&k) {
                                    (v, false)
                                } else {
                                    (&i.1, true)
                                };
                            let label = i.0.clone() + " : ";
                            options = options.push(get_widget(v, label, k, uses_default))
                        }
                    }
                }

                options = options.push(
                    row![
                        iced::widget::button(iced::widget::text("Cancel"))
                            .on_press(Message::ToggleSettings),
                        iced::widget::button(iced::widget::text("Ok"))
                            .on_press(Message::ApplyCloseSettings),
                    ]
                    .align_items(iced::Alignment::End),
                );

                options.into()
            }
            GridStatus::Logs => {
                iced::widget::scrollable(iced::widget::text(if let Some(g) = self.selected {
                    self.games[g].current_log.clone()
                } else {
                    String::new()
                }))
                .width(Length::Fill)
                .into()
            }
        };

        let content = column![
            // row![game_viewer, global_settings]
            //     .spacing(10)
            top_bar.height(Length::Fixed(30 as f32)),
            iced::widget::scrollable(ge).height(Length::Fill),
        ]
        .spacing(20)
        .padding(20)
        .width(Length::Fill);

        let content: Element<_> = if self.steam_grid_db {
            iced_aw::modal::Modal::new(true, content, || {
                column![
                    iced::widget::Space::with_height(Length::FillPortion(1)),
                    iced_aw::Card::new(iced::widget::text("Choose a banner"), {
                        let mut grid: iced_aw::Grid<Message, _, _> =
                            iced_aw::Grid::with_column_width(IMAGE_WIDTH as f32 + 20.);
                        for (i, im) in self.sgdb_images.iter().enumerate() {
                            grid.insert::<Element<Message>>(
                                iced::widget::button(
                                    iced::widget::column(vec![iced::widget::image(
                                        iced::widget::image::Handle::from_pixels(
                                            im.1.width(),
                                            im.1.height(),
                                            im.1.as_raw().clone(),
                                        ),
                                    )
                                    .width(Length::Fixed(IMAGE_WIDTH as f32))
                                    .into()])
                                    .align_items(iced::Alignment::Center),
                                )
                                .on_press(Message::SGDBThumbSelected(i))
                                .style(if Some(i) != self.sgdb_selected {
                                    iced::theme::Button::Custom(Box::new(ButtonStyle()))
                                } else {
                                    iced::theme::Button::default()
                                })
                                .into(),
                            );
                        }
                        iced::widget::scrollable(column![
                            grid,
                            row![
                                iced::widget::button(iced::widget::text("Cancel"))
                                    .on_press(Message::CancelSGDB),
                                iced::widget::button(iced::widget::text("Ok"))
                                    .on_press(Message::ApplySGDB),
                            ]
                        ])
                    })
                    .height(Length::FillPortion(10)),
                    iced::widget::Space::with_height(Length::FillPortion(1)),
                ]
                .into()
            })
            .into()
        } else {
            content.into()
        };

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::time::every(iced::time::Duration::from_millis(1000)).map(|_| Message::MonotonicClock)
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

fn make_path_proof(s: String) -> String {
    let mut out = String::with_capacity(s.len());
    for i in s.chars() {
        if i.is_ascii() && i.is_alphanumeric() {
            out.push(i.to_ascii_lowercase())
        } else if i.is_whitespace() {
            out.push('_')
        }
    }
    out
}
