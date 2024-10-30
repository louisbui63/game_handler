mod citra;
mod config;
mod date;
mod duckstation;
mod games;
mod grid_widget;
mod mame;
mod native;
mod pcsx2;
mod process_subscription;
mod rpcs3;
mod ryujinx;
mod sort;
mod steam;
mod theme;
mod ui;
#[cfg(unix)]
mod umu;
mod vita3k;
#[cfg(unix)]
mod wine;
mod yuzu;

use std::collections::HashMap;

use config::{get_default_config_with_vals, CValue, Cfg};
use games::Game;
use iced::color;
use iced::futures::channel::mpsc::Sender;
use iced::theme::Palette;
use iced::Task as Command;

pub const WIDGET_HEIGHT: u16 = 31;
pub const IMAGE_WIDTH: u32 = 200;
pub const IMAGE_HEIGHT: u32 = 300;

pub static DIRS: once_cell::sync::Lazy<directories::ProjectDirs> =
    once_cell::sync::Lazy::new(|| {
        directories::ProjectDirs::from("com", "louisbui63", "game_handler")
            .expect("couldn't find home directory")
    });

fn main() -> iced::Result {
    // File::create()
    flexi_logger::Logger::try_with_env_or_str("info")
        .unwrap()
        .log_to_file(
            flexi_logger::FileSpec::try_from(DIRS.data_dir().join("log")).unwrap_or(
                flexi_logger::FileSpec::default()
                    .directory(DIRS.data_dir().join("logs"))
                    .basename("log"),
            ),
        )
        .duplicate_to_stdout(flexi_logger::Duplicate::Error)
        .start()
        .unwrap();

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
    if let Err(e) = std::fs::create_dir_all(DIRS.data_dir().join("mame")) {
        log::error!("couldn't ensure the data directory existence or integrity : {e}");
        panic!()
    }

    // log::info!("Loading theme");
    // let file = if let Ok(ct) = std::fs::read_to_string(DIRS.data_dir().join("theme.toml")) {
    //     ct
    // } else {
    //     String::new()
    // };

    // if std::env::args().nth(1).unwrap_or(String::new()) == "--dump-default-theme".to_string() {
    //     println!(
    //         "{}",
    //         toml::to_string_pretty(&crate::theme::CurrentTheme::default()).unwrap()
    //     );
    //     panic!("Error: Success");
    // }

    // *crate::theme::CURRENT_THEME.lock().unwrap() = toml::from_str(&file[..])
    //     .unwrap_or_else(|_| {
    //         log::error!("Theme file \"%data_dir%/theme.toml\" not found or wrongly formatted : defaulting to embedded theme.");
    //         toml::from_str(theme::EMBEDDED_THEME).unwrap()});

    log::info!("UI starting");
    // MainGUI::run(Settings {
    //     ..Default::default()
    // })
    iced::application("game_handler", MainGUI::update, MainGUI::view)
        .subscription(MainGUI::subscription)
        .theme(MainGUI::theme)
        .run_with(MainGUI::new)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum GridStatus {
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

pub struct MainGUI {
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
    sgdb_query: String,
    sgdb_other_possibilities: Vec<steamgriddb_api::search::SearchResult>,
    sgdb_async_status: SGDBAsyncStatus,
    time_played_db: HashMap<String, std::time::Duration>,
    time_played_ty_db: HashMap<String, std::time::Duration>,
    sort_alg: sort::Sorts,
    log: iced::widget::text_editor::Content,
}

impl MainGUI {
    fn update_db(&self) {
        if let Ok(mut file) = std::fs::File::create(DIRS.data_dir().join("times.toml")) {
            let mut db = HashMap::new();
            self.time_played_db
                .iter()
                .map(|(k, v)| db.insert(k, v.as_secs() as i64))
                .last();
            let _ = std::io::Write::write_fmt(
                &mut file,
                format_args!("{}", toml::to_string(&db).unwrap_or("42".to_owned())),
            );
        }
        if let Ok(mut file) = std::fs::File::create(DIRS.data_dir().join("times_ty.toml")) {
            let mut db = HashMap::new();
            self.time_played_ty_db
                .iter()
                .map(|(k, v)| db.insert(k, v.as_secs() as i64))
                .last();
            let _ = std::io::Write::write_fmt(
                &mut file,
                format_args!("{}", toml::to_string(&db).unwrap_or("42".to_owned())),
            );
        }
    }

    fn update_log(&mut self) {
        if let Some(g) = self.selected {
            // let sel = self.log.selection();
            // let pos = self.log.cursor_position();
            self.log = iced::widget::text_editor::Content::with_text(&self.games[g].current_log);
        } else {
            self.log = iced::widget::text_editor::Content::new();
        }
    }
}

enum SGDBAsyncStatus {
    Nothing,
    SearchQuery(String),
    ImageQuery(usize),
    ImageDownload(Vec<steamgriddb_api::images::Image>),
    FinalImageDownload(steamgriddb_api::images::Image),
    NoImage,
}
impl std::default::Default for SGDBAsyncStatus {
    fn default() -> Self {
        Self::Nothing
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ThemeType {
    Light,
    Dark,
}

#[derive(Debug, Clone)]
pub enum Message {
    GameSelected(usize),
    RunSelected,
    RunSubcommandSelected(String),
    ToggleSettings,
    ToggleGlobalSettings,
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
    SGDBChangeQuery(String),
    SGDBSelectGame(usize),
    SGDBAsyncSearchQueryDone(Vec<steamgriddb_api::search::SearchResult>),
    SGDBAsyncImageQueryStart(Vec<steamgriddb_api::images::Image>),
    SGDBAsyncImageDownloadProgress(
        Option<(
            steamgriddb_api::images::Image,
            image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        )>,
    ),
    SGDBAsyncFinalImageDownloadDone(Option<String>),
    DoNothing,
    AddLogs(usize, String),
    AddSender(usize, Sender<process_subscription::PSubInput>),
    ProcessDied(usize),
    ProcessWatcherClock,
    LogAction(iced::widget::text_editor::Action),
    SGDBAsyncNoImage,
}

impl MainGUI {
    // type Message = Message;
    // type Theme = crate::theme::Theme;
    // type Executor = executor::Default;
    // type Flags = ();

    fn new() -> (Self, Command<Message>) {
        let mut time_played_db = HashMap::new();
        let mut time_played_ty_db = HashMap::new();
        std::fs::read_to_string(DIRS.data_dir().join("times.toml"))
            .unwrap_or_else(|e| {
                log::error!("couldn't read playtime database : {e}");
                String::new()
            })
            .parse::<toml::Table>()
            .unwrap_or_else(|e| {
                log::error!("couldn't read playtime database : {e}");
                toml::Table::new()
            })
            .try_into::<HashMap<String, i64>>()
            .unwrap_or_else(|e| {
                log::error!("couldn't convert playtime database : {e}");
                HashMap::new()
            })
            .drain()
            .map(|(k, i)| {
                time_played_db.insert(k, std::time::Duration::from_secs(i.try_into().unwrap_or(0)))
            })
            .last(); // here we just consume the iter such that our map does something
        std::fs::read_to_string(DIRS.data_dir().join("times_ty.toml"))
            .unwrap_or_else(|e| {
                log::error!("couldn't read playtime database : {e}");
                String::new()
            })
            .parse::<toml::Table>()
            .unwrap_or_else(|e| {
                log::error!("couldn't read playtime database : {e}");
                toml::Table::new()
            })
            .try_into::<HashMap<String, i64>>()
            .unwrap_or_else(|e| {
                log::error!("couldn't convert playtime database : {e}");
                HashMap::new()
            })
            .drain()
            .map(|(k, i)| {
                time_played_ty_db
                    .insert(k, std::time::Duration::from_secs(i.try_into().unwrap_or(0)))
            })
            .last(); // here we just consume the iter such that our map does something
        let mut games: Vec<_> = std::fs::read_dir(DIRS.config_dir().join("games"))
            .unwrap()
            .map(|a| {
                Game::from_toml(
                    &a.unwrap().path(),
                    &DIRS.config_dir().join("settings.toml"),
                    &time_played_db,
                    &time_played_ty_db,
                )
            })
            .collect();
        //alphabetic sort
        // games.sort_unstable_by_key(|a| a.name.clone());
        sort::sort_none_selected(&mut games, sort::Sorts::Name.get_fn());

        (
            MainGUI {
                games,
                selected: None,
                default_config: get_default_config_with_vals(
                    &DIRS.config_dir().join("settings.toml"),
                ),
                temp_settings: None,
                grid_status: GridStatus::GamesGrid,
                steam_grid_db: false,
                sgdb_images: vec![],
                sgdb_selected: None,
                sgdb_query: String::new(),
                sgdb_other_possibilities: vec![],
                sgdb_async_status: SGDBAsyncStatus::default(),
                time_played_db,
                time_played_ty_db,
                sort_alg: sort::Sorts::Name,
                log: iced::widget::text_editor::Content::new(),
            },
            iced::font::load(iced_fonts::NERD_FONT_BYTES).map(|_| Message::DoNothing),
            // Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Game Handler")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::GameSelected(id) => {
                self.selected = Some(id);
                self.update_log();
                Command::none()
            }
            Message::RunSelected => {
                if let Some(i) = self.selected {
                    self.games[i].current_log.clear();
                    self.games[i].time_started = Some(std::time::SystemTime::now());
                    self.games[i].run();
                    if self.games[i].config.no_sleep_enabled {
                        match nosleep::NoSleep::new() {
                            Ok(mut ns) => {
                                ns.start(nosleep::NoSleepType::PreventUserIdleDisplaySleep)
                                    .unwrap_or_else(|e| {
                                        log::error!("couldn't initiate sleep blocker : {e}")
                                    });
                                self.games[i].no_sleep = Some(ns);
                            }
                            Err(e) => {
                                log::error!("couldn't initiate sleep blocker : {e}")
                            }
                        }
                    }
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
                            self.temp_settings.as_ref().unwrap().clone().into_game(
                                &DIRS.config_dir().join("settings.toml"),
                                path.clone(),
                                &self.time_played_db,
                                &self.time_played_ty_db,
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
                            &DIRS.config_dir().join("settings.toml"),
                        );

                        for i in 0..self.games.len() {
                            self.games[i] = self.games[i].bare_config.clone().into_game(
                                &DIRS.config_dir().join("settings.toml"),
                                self.games[i].path_to_toml.clone(),
                                &self.time_played_db,
                                &self.time_played_ty_db,
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
                                .unwrap_or_default(),
                        );
                        let path = DIRS
                            .config_dir()
                            .join("games")
                            .join(random_id.to_string() + &name[..] + ".toml");
                        let cfg = self.temp_settings.as_mut().unwrap();
                        self.games.push(cfg.clone().into_game(
                            &DIRS.config_dir().join("settings.toml"),
                            path.clone(),
                            &self.time_played_db,
                            &self.time_played_ty_db,
                        ));

                        let to_write = self.temp_settings.as_ref().unwrap().to_toml();
                        use std::io::prelude::*;
                        std::fs::File::create(path)
                            .unwrap()
                            .write_all(to_write.as_bytes())
                            .unwrap();

                        self.sort(self.sort_alg.get_fn());
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
                        &DIRS.config_dir().join("settings.toml"),
                    ));
                    self.grid_status = GridStatus::GlobalSettings;
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
                                &DIRS.config_dir().join("settings.toml"),
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
                    self.games[i].current_log.clear();
                    self.games[i].run_subcommand(s);
                }
                Command::none()
            }
            Message::MonotonicClock => {
                for g in &mut self.games {
                    if g.is_running {
                        if let Some(s) = &mut g.psub_sender {
                            if let Err(e) = s.try_send(process_subscription::PSubInput::ReadInput) {
                                log::error!("Couldn't send log request : {e}");
                            }
                        }
                    }
                }
                Command::none()
            }
            Message::ProcessWatcherClock => Command::none(),
            Message::KillSelected => {
                if let Some(i) = self.selected {
                    if let Some(sender) = &mut self.games[i].psub_sender {
                        if let Err(e) = sender.try_send(process_subscription::PSubInput::Terminate)
                        {
                            log::error!("Unable to send signal to process subscription : {e}");
                        }
                    } else {
                    }
                }
                Command::none()
            }
            Message::SteamGridDb => {
                let title = self
                    .temp_settings
                    .as_ref()
                    .unwrap()
                    .0
                    .get("name")
                    .unwrap_or(&CValue::Str("".to_owned()))
                    .as_string();
                self.sgdb_async_status = SGDBAsyncStatus::SearchQuery(title.clone());
                self.steam_grid_db = true;
                self.sgdb_query = title;
                Command::none()
            }
            Message::SGDBChangeQuery(q) => {
                self.sgdb_query = q.clone();
                self.sgdb_async_status = SGDBAsyncStatus::SearchQuery(q);
                Command::none()
            }
            Message::SGDBThumbSelected(i) => {
                self.sgdb_selected = Some(i);
                Command::none()
            }
            Message::ApplySGDB => {
                if let Some(sel) = self.sgdb_selected {
                    self.sgdb_async_status =
                        SGDBAsyncStatus::FinalImageDownload(self.sgdb_images[sel].0.clone())
                }
                Command::none()
            }
            Message::CancelSGDB => {
                self.sgdb_selected = None;
                self.steam_grid_db = false;
                self.sgdb_async_status = SGDBAsyncStatus::Nothing;
                self.sgdb_images.clear();
                Command::none()
            }
            Message::SGDBSelectGame(id) => {
                self.sgdb_async_status = SGDBAsyncStatus::ImageQuery(id);
                Command::none()
            }
            Message::SGDBAsyncSearchQueryDone(games) => {
                self.sgdb_other_possibilities = games;
                self.sgdb_async_status = SGDBAsyncStatus::Nothing;
                Command::none()
            }
            Message::SGDBAsyncImageQueryStart(images) => {
                self.sgdb_async_status =
                    SGDBAsyncStatus::ImageDownload(images.iter().rev().cloned().collect());
                self.sgdb_images.clear();
                Command::none()
            }
            Message::SGDBAsyncImageDownloadProgress(image) => {
                if let Some(im) = image {
                    self.sgdb_images.push(im);
                }
                if let SGDBAsyncStatus::ImageDownload(images) = &mut self.sgdb_async_status {
                    let _ = images.pop();
                    if images.is_empty() {
                        self.sgdb_async_status = SGDBAsyncStatus::Nothing;
                    }
                }

                Command::none()
            }
            Message::SGDBAsyncFinalImageDownloadDone(image) => {
                self.sgdb_async_status = SGDBAsyncStatus::Nothing;
                if let Some(im) = image {
                    self.temp_settings
                        .as_mut()
                        .unwrap()
                        .0
                        .insert("box_art".to_owned(), CValue::PickFile(im));
                    self.sgdb_selected = None;
                    self.steam_grid_db = false;
                    self.sgdb_images.clear();
                }
                Command::none()
            }
            Message::SGDBAsyncNoImage => {
                self.sgdb_async_status = SGDBAsyncStatus::NoImage;
                Command::none()
            }
            Message::DoNothing => {
                //yep, you guessed it, we do nothing here
                Command::none()
            }
            Message::AddLogs(i, logs) => {
                self.games[i].current_log += &logs[..];
                if i == self.selected.unwrap_or(usize::MAX) {
                    use iced::widget::text_editor::{Action, Edit, Motion};
                    self.log.perform(Action::Move(Motion::DocumentEnd));
                    self.log
                        .perform(Action::Edit(Edit::Paste(std::sync::Arc::new(logs))));
                }

                // self.update_log();
                Command::none()
            }
            Message::AddSender(i, sender) => {
                self.games[i].psub_sender = Some(sender);
                Command::none()
            }
            Message::ProcessDied(i) => {
                if let Some(when) = self.games[i].time_started {
                    log::info!(
                        "process {i} died: previous playtime: {:?}",
                        self.games[i].time_played
                    );
                    self.games[i].time_played += std::time::SystemTime::now()
                        .duration_since(when)
                        .unwrap_or(std::time::Duration::new(0, 0));
                    log::info!(
                        "process {i} died: new playtime: {:?}",
                        self.games[i].time_played
                    );
                    self.games[i].time_played_this_year += std::time::SystemTime::now()
                        .duration_since(when)
                        .unwrap_or(std::time::Duration::new(0, 0));
                    self.time_played_db.insert(
                        self.games[i]
                            .path_to_toml
                            .file_name()
                            .unwrap_or_default()
                            .to_str()
                            .unwrap_or_default()
                            .to_owned(),
                        self.games[i].time_played,
                    );
                    self.games[i].time_started = None;
                    self.update_db();
                } else {
                    log::info!("process {i} died: no playtime added")
                }
                self.games[i].no_sleep = None;
                self.games[i].psub_sender = None;
                self.games[i].is_running = false;
                Command::none()
            }
            Message::LogAction(a) => {
                match a {
                    iced::widget::text_editor::Action::Edit(_) => {}
                    a => self.log.perform(a),
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        let MainGUI { .. } = self;

        ui::get_view_widget(self)
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        let sgdb_async = match &self.sgdb_async_status {
            SGDBAsyncStatus::Nothing | SGDBAsyncStatus::NoImage => iced::Subscription::none(),
            SGDBAsyncStatus::SearchQuery(q) => {
                let api_key = self
                    .default_config
                    .get("launcher:sgdb_api_key")
                    .unwrap()
                    .1
                    .as_string();
                iced::Subscription::run_with_id(
                    q.clone(),
                    iced::futures::stream::unfold(q.clone(), move |q| {
                        let api_key = api_key.clone();
                        async move {
                            use steamgriddb_api::Client;
                            let client = Client::new(api_key);
                            let out = client.search(&q[..]).await;
                            Some((
                                Message::SGDBAsyncSearchQueryDone(out.unwrap_or_else(|e| {
                                    log::error!("error in SGDB search query : {e}");
                                    Vec::new()
                                })),
                                q.clone(),
                            ))
                        }
                    }),
                )
            }
            SGDBAsyncStatus::ImageQuery(id) => {
                let api_key = self
                    .default_config
                    .get("launcher:sgdb_api_key")
                    .unwrap()
                    .1
                    .as_string();
                iced::Subscription::run_with_id(
                    *id,
                    iced::futures::stream::unfold(*id, move |id| {
                        let api_key = api_key.clone();
                        async move {
                            use steamgriddb_api::Client;
                            let client = Client::new(api_key);
                            let images = client.get_images_for_id(
                                id,
                                &steamgriddb_api::QueryType::Grid(Some(
                                    steamgriddb_api::query_parameters::GridQueryParameters {
                                        dimentions: Some(&[
                                            steamgriddb_api::query_parameters::GridDimentions::D600x900,
                                        ]),
                                        ..std::default::Default::default()
                                    },
                                )),
                            ).await
                            .unwrap_or_else(|_| vec![]);

                            if images.is_empty() {
                                Some((Message::SGDBAsyncNoImage, id))
                            } else {
                                Some((Message::SGDBAsyncImageQueryStart(images), id))
                            }
                        }
                    }),
                )
            }
            SGDBAsyncStatus::ImageDownload(images) => {
                if let Some(image) = images.last() {
                    iced::Subscription::run_with_id(
                        image.id,
                        iced::futures::stream::unfold(image.clone(), move |image| async move {
                            let im = if let Ok(resp) = reqwest::get(image.thumb.clone()).await {
                                if resp.status() == reqwest::StatusCode::OK {
                                    if let Ok(u) = resp.bytes().await {
                                        Some((
                                            image.clone(),
                                            image::load_from_memory(&u).unwrap().to_rgba8(),
                                        ))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            } else {
                                None
                            };
                            Some((Message::SGDBAsyncImageDownloadProgress(im), image))
                        }),
                    )
                } else {
                    iced::Subscription::none()
                }
            }
            SGDBAsyncStatus::FinalImageDownload(image) => iced::Subscription::run_with_id(
                image.id,
                iced::futures::stream::unfold(image.clone(), move |image| async move {
                    let url = image.clone().url;
                    let name = image.clone().id.to_string()
                        + match image.clone().mime {
                            steamgriddb_api::images::MimeTypes::Default(tp) => match tp {
                                steamgriddb_api::query_parameters::MimeType::Png => ".png",
                                steamgriddb_api::query_parameters::MimeType::Jpeg => ".jpeg",
                                steamgriddb_api::query_parameters::MimeType::Webp => ".webp",
                            },
                            _ => unreachable!(),
                        };

                    let path = if let Ok(resp) = reqwest::get(url).await {
                        if resp.status() == reqwest::StatusCode::OK {
                            use std::io::prelude::*;
                            let path = DIRS.data_dir().join("banners").join(name);
                            std::fs::File::create(path.clone())
                                .unwrap()
                                .write_all(&resp.bytes().await.unwrap())
                                .unwrap();
                            path.to_str().map(|a| a.to_owned())
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    Some((Message::SGDBAsyncFinalImageDownloadDone(path), image))
                }),
            ),
        };
        let mono_clock = iced::time::every(iced::time::Duration::from_millis(
            if let GridStatus::Logs = self.grid_status {
                17
            } else {
                1000
            },
        ))
        .map(|_| Message::MonotonicClock);
        let mut running_processes = Vec::new();
        for (i, g) in self.games.iter().enumerate() {
            if g.is_running {
                running_processes.push(process_subscription::get_psub(i, g.cmd_to_run.clone()).map(
                    |input| match input {
                        process_subscription::Event::Ready(i, sender) => {
                            Message::AddSender(i, sender)
                        }
                        process_subscription::Event::GotLogs(i, logs) => Message::AddLogs(i, logs),
                        process_subscription::Event::ProcessEnded(i) => Message::ProcessDied(i),
                    },
                ))
            }
        }
        running_processes.push(mono_clock);
        running_processes.push(sgdb_async);
        iced::Subscription::batch(running_processes)
    }

    fn theme(&self) -> iced::Theme {
        iced::theme::Theme::custom_with_fn(
            // So here we should really invert primary strong and primary base (somehow)
            "that cute pink theme".to_owned(),
            Palette {
                background: color!(0x303446), // Base
                text: color!(0xc6d0f5),       // Text
                primary: color!(0xFFABFF),    // Not Blue
                success: color!(0xa6d189),    // Green
                danger: color!(0xe78284),     // Red
            },
            |pal| {
                let mut out = iced::theme::palette::Extended::generate(pal);
                out.primary.strong.color = out.primary.base.color;
                out.primary.base.color = mul_color(out.primary.base.color, 4. / 5.);

                out
            },
        )
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

fn duration_to_string(dur: std::time::Duration) -> String {
    let mins = dur.as_secs() / 60;
    let hours = mins / 60;
    format!("{} hours {} minutes", hours, mins - hours * 60)
}

fn mul_color(c: iced::Color, mul: f32) -> iced::Color {
    iced::Color {
        r: c.r * mul,
        g: c.g * mul,
        b: c.b * mul,
        a: c.a,
    }
}
