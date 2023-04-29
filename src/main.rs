mod config;
mod games;
mod mame;
mod native;
mod rpcs3;
mod ryujinx;
mod wine;

use std::collections::HashMap;

use config::{get_default_config_with_vals, CValue, Cfg};
use games::Game;
use iced::executor;
use iced::widget::{button, column, container, horizontal_rule, radio, row};
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

pub fn main() -> iced::Result {
    pretty_env_logger::init();
    log::info!("Starting...");
    log::info!("checking config directory {:?}...", DIRS.config_dir());
    if let Err(e) = std::fs::create_dir_all(DIRS.config_dir().join("games")) {
        log::error!("couldn't ensure the config directory existence or integrity : {e}");
        panic!()
    }
    MainGUI::run(Settings {
        // antialiasing: true,
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
        config::CValue::PickFile(s) => row![
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
            .on_press(Message::FolderPicker(k.clone()))
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
}

impl std::convert::TryFrom<usize> for GridStatus {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::GamesGrid),
            1 => Ok(Self::GlobalSettings),
            2 => Ok(Self::AddGame),
            3 => Ok(Self::GamesSettings),
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
                    self.games[i].run_subcommand(s);
                }
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
                .width(Length::FillPortion(4));
            resizer = 1;
        }

        let run_module: iced::Element<_> = if let Some(g) = self.selected {
            row![
                iced::widget::button(iced::widget::text("run")).on_press(Message::RunSelected),
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
            iced::widget::Space::with_width(Length::FillPortion(2 - resizer)),
            run_module,
        ]
        .align_items(iced::Alignment::End);

        // let mut game_viewer = row![column![
        //     iced::widget::text(if let Some(i) = self.selected {
        //         self.games[i].name.clone()
        //     } else {
        //         "no game selected".to_owned()
        //     }),
        //     {
        //         let mut c = iced::widget::Column::new();
        //         if let Some(i) = self.selected {
        //             for i in self.games[i].get_subcommands() {
        //                 c = c.push(
        //                     iced::widget::button(iced::widget::text(i.clone()))
        //                         .on_press(Message::RunSubcommandSelected(i)),
        //                 );
        //             }
        //         }
        //         c
        //     }
        // ]
        // .spacing(10)];

        // if let Some(_) = self.selected {
        //     let mut column = column![];
        //     column = column.push(
        //         iced::widget::button(iced::widget::text("run")).on_press(Message::RunSelected),
        //     );
        //     if let GridStatus::GamesGrid = self.grid_status {
        //         column = column.push(
        //             iced::widget::button(iced::widget::text("game settings"))
        //                 .on_press(Message::ToggleSettings),
        //         );
        //     }
        //     game_viewer = game_viewer.push(column);
        // }

        // let global_settings = column![
        //     iced::widget::button(iced::widget::text("global settings"))
        //         .on_press(Message::ToggleGlobalSettings),
        //     iced::widget::button(iced::widget::text("add game")).on_press(Message::ToggleAddGame),
        // ];

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
                        let s = k.split(':').collect::<Vec<_>>();
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
        };

        let content = column![
            // row![game_viewer, global_settings]
            //     .spacing(10)
            top_bar.height(Length::Fixed(50 as f32)),
            iced::widget::scrollable(ge).height(Length::Fill),
        ]
        .spacing(20)
        .padding(20)
        .width(Length::Fill);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
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
