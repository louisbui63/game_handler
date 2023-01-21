mod config;
mod games;

use std::borrow::Borrow;

use config::get_default_config;
use games::Game;
use iced::executor;
use iced::futures::io::Window;
use iced::theme::Text;
use iced::widget::{
    button, column, container, horizontal_rule, progress_bar, radio, row, scrollable, text,
    vertical_space, Row,
};
use iced::{Application, Command, Element, Length, Settings, Theme};

pub fn main() -> iced::Result {
    pretty_env_logger::init();
    log::info!("Starting...");
    MainGUI::run(Settings::default())
}

struct MainGUI {
    theme: Theme,
    games: Vec<Game>,
    selected: Option<usize>,
    option_toggled: bool,
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
    ToggleSettings,
    TextSetting(String, String),
}

impl Application for MainGUI {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let default_cfg = games::DefaultCfg::new("settings.toml");
        let mut games: Vec<_> = std::fs::read_dir("games")
            .unwrap()
            .map(|a| Game::from_toml(a.unwrap().path().to_str().unwrap(), &default_cfg))
            .collect();
        //alphabetic sort
        games.sort_by_key(|a| a.name.clone());

        (
            MainGUI {
                theme: Default::default(),
                games,
                selected: None,
                option_toggled: false,
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
                self.option_toggled = !self.option_toggled;
                Command::none()
            }
            Message::TextSetting(s1, s2) => {
                match &s1[..] {
                    "name" => self.games[self.selected.unwrap()].name = s2,
                    "box_art" => {
                        self.games[self.selected.unwrap()].box_art =
                            if s2.is_empty() { None } else { Some(s2) }
                    }
                    _ => eprintln!("error : tried to modify unknown setting"),
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let MainGUI { .. } = self;

        let mut game_viewer = column![iced::widget::text(if let Some(i) = self.selected {
            self.games[i].name.clone()
        } else {
            "no game selected".to_owned()
        })]
        .spacing(10);

        if let Some(i) = self.selected {
            game_viewer = game_viewer
                .push(
                    iced::widget::button(iced::widget::text("run")).on_press(Message::RunSelected),
                )
                .push(
                    iced::widget::button(iced::widget::text("toggle settings"))
                        .on_press(Message::ToggleSettings),
                );
        }

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

        let ge: Element<Message> = if !self.option_toggled {
            let image_size = 200;

            let mut grid: iced_aw::Grid<Message, _, _> =
                iced_aw::Grid::with_column_width(image_size + 20);
            for (i, g) in self.games.iter().enumerate() {
                grid.insert::<Element<Message>>(
                    iced::widget::button(iced::widget::column(vec![
                        iced::widget::image(iced::widget::image::Handle::from_pixels(
                            g.image.width(),
                            g.image.height(),
                            g.image.as_raw().clone(),
                        ))
                        .width(Length::Units(image_size))
                        .into(),
                        iced::widget::text(g.name.clone()).into(),
                    ]))
                    .on_press(Message::GameSelected(i))
                    .into(),
                );
            }
            grid.into()
        } else {
            let mut options = column![];

            let default = get_default_config();
            // options = options.push(row![
            //     iced::widget::text("name : "),
            //     iced::widget::text_input("", &self.games[self.selected.unwrap()].name[..], |a| {
            //         Message::TextSetting("name".to_owned(), a)
            //     }),
            // ]);
            // options = options.push(row![
            //     iced::widget::text("box art : "),
            //     iced::widget::text_input(
            //         "",
            //         &self.games[self.selected.unwrap()]
            //             .box_art
            //             .clone()
            //             .unwrap_or(String::new())[..],
            //         |a| { Message::TextSetting("box_art".to_owned(), a) }
            //     ),
            // ]);
            // options = options.push(row![
            //     iced::widget::text("release year : "),
            //     iced::widget::text_input(
            //         "",
            //         &self.games[self.selected.unwrap()]
            //             .release_year
            //             .map_or("".to_owned(), |a| format!("{}", a))[..],
            //         |a| { Message::TextSetting("release_year".to_owned(), a) }
            //     ),
            // ]);
            // options = options.push(row![
            //     iced::widget::text("path to the game : "),
            //     iced::widget::text_input(
            //         "",
            //         &self.games[self.selected.unwrap()].path_to_game[..],
            //         |a| { Message::TextSetting("path_to_game".to_owned(), a) }
            //     ),
            // ]);
            // options = options.push(row![
            //     iced::widget::text("runner : "),
            //     iced::widget::pick_list(
            //         &games::RUNNERS[..],
            //         {
            //             let mut found = None;
            //             for i in games::RUNNERS {
            //                 if i.to_owned() == self.games[self.selected.unwrap()].runner_id {
            //                     found = Some(i);
            //                     break;
            //                 }
            //             }
            //             found
            //         },
            //         |a| { Message::TextSetting("runner".to_owned(), a.to_owned()) }
            //     )
            // ]);
            options.into()
        };

        let content = column![
            row![choose_theme, game_viewer].spacing(10),
            horizontal_rule(20),
            iced::widget::scrollable(ge)
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
