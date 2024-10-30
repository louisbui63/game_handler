use crate::config;
// use crate::theme::widget::Element;
use crate::Message;
use crate::IMAGE_HEIGHT;
use crate::{IMAGE_WIDTH, WIDGET_HEIGHT};
use config::CValue;
use iced::alignment::Horizontal;
use iced::widget::image::FilterMethod;
use iced::widget::text;
use iced::widget::{button, column, container, row};
use iced::Alignment;
use iced::{Length, Theme};
use iced_aw::{TabBar, TabLabel};
use iced_fonts::NERD_FONT;

// struct ButtonStyle();
// impl iced::widget::button::StyleSheet for ButtonStyle {
//     type Style = Theme;
//
//     fn active(&self, style: &Self::Style) -> button::Appearance {
//         button::Appearance {
//             background: Some(iced::Background::Color(iced::Color::TRANSPARENT)),
//             text_color: match style {
//                 Theme::Light => iced::Color::BLACK,
//                 Theme::Dark => iced::Color::WHITE,
//                 Theme::Custom(_) => iced::color!(255, 0, 0),
//                 Theme::Dracula => todo!(),
//                 Theme::Nord => todo!(),
//                 Theme::SolarizedLight => todo!(),
//                 Theme::SolarizedDark => todo!(),
//                 Theme::GruvboxLight => todo!(),
//                 Theme::GruvboxDark => todo!(),
//                 Theme::CatppuccinLatte => todo!(),
//                 Theme::CatppuccinFrappe => todo!(),
//                 Theme::CatppuccinMacchiato => todo!(),
//                 Theme::CatppuccinMocha => todo!(),
//                 Theme::TokyoNight => todo!(),
//                 Theme::TokyoNightStorm => todo!(),
//                 Theme::TokyoNightLight => todo!(),
//                 Theme::KanagawaWave => todo!(),
//                 Theme::KanagawaDragon => todo!(),
//                 Theme::KanagawaLotus => todo!(),
//                 Theme::Moonfly => todo!(),
//                 Theme::Nightfly => todo!(),
//                 Theme::Oxocarbon => todo!(),
//             },
//             ..Default::default()
//         }
//     }
// }

pub fn get_widget(
    v: &config::CValue,
    label: String,
    k: String,
    uses_default: bool,
) -> iced::widget::Row<'_, Message> {
    match v {
        config::CValue::Str(s) => row![
            iced::widget::text(label)
                .width(Length::FillPortion(3)),
                // .into(),
            iced::widget::text_input("", s)
                .on_input({
                    let k1 = k.clone();
                    move |a| Message::SettingChanged(k1.clone(), CValue::Str(a))
                })
                .width(Length::FillPortion(3)),
                // .into(),
            iced::widget::toggler(uses_default).on_toggle(move |a| {
                Message::SettingDefaultChanged(k.clone(), a)
            })
            .width(Length::FillPortion(1)),
            // .into(),
        ]
        .height(Length::Fixed(WIDGET_HEIGHT as f32)),

        config::CValue::Bool(b) => row![
            iced::widget::text(label)
                .width(Length::FillPortion(3)),
                // .into(),
            iced::widget::Container::new(
                iced::widget::toggler(*b).on_toggle( {
                    let k1 = k.clone();
                    move |a| Message::SettingChanged(k1.clone(), CValue::Bool(a))
                }),
                // .into()
            )
            .width(Length::FillPortion(1)),
            iced::widget::Space::with_width(Length::FillPortion(2)),//.into(),
            iced::widget::toggler(uses_default).on_toggle( move |a| {
                Message::SettingDefaultChanged(k.clone(), a)
            })
            .width(Length::FillPortion(1)),
            // .into(),
        ]
        .height(Length::Fixed(WIDGET_HEIGHT as f32)),
        config::CValue::StrArr(arr) => {
            // log::error!("Feature StrArr() not yet available in config display");
            let mut col = Vec::new();
            for i in 0..(arr.len() / 2) + 1 {
                let mut row: Vec<iced::Element<'_, Message>>/*: Vec<Element<'_, Message, _>>*/ = Vec::new();
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
                // .into(),
                col.width(Length::FillPortion(3)), //.into(),
                iced::widget::toggler(uses_default).on_toggle(move |a| {
                    Message::SettingDefaultChanged(k.clone(), a)
                })
                .width(Length::FillPortion(1)),
                // .into(),
            ]
        }
        config::CValue::OneOff(l, s) => row![
            iced::widget::text(label).width(Length::FillPortion(3)),
            iced::widget::pick_list(l.clone(), Some(l[*s].clone()), {
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
            iced::widget::toggler(uses_default).on_toggle(move |a| {
                Message::SettingDefaultChanged(k.clone(), a)
            })
            .width(Length::FillPortion(1)),
        ]
        .height(Length::Fixed(WIDGET_HEIGHT as f32)),
        config::CValue::PickFile(s) => if k == "box_art" {
            row![
                iced::widget::text(label).width(Length::FillPortion(6)),
                // .into(),
                iced::widget::text_input("", s)
                    .on_input({
                        let k1 = k.clone();
                        move |a| Message::SettingChanged(k1.clone(), CValue::PickFile(a))
                    })
                    .width(Length::FillPortion(4)),
                // .into(),
                iced::widget::button(
                    text("󰕰")
                        .font(NERD_FONT)
                        .align_x(Horizontal::Center)
                )
                .on_press(Message::SteamGridDb)
                .width(Length::FillPortion(1)),
                // .into(),
                iced::widget::button(
                    text("󰉋")
                        .font(NERD_FONT)
                        .align_x(Horizontal::Center)
                )
                .on_press(Message::FilePicker(k.clone()))
                .width(Length::FillPortion(1)),
                // .into(),
                iced::widget::toggler(uses_default).on_toggle(move |a| {
                    Message::SettingDefaultChanged(k.clone(), a)
                })
                .width(Length::FillPortion(2)),
                // .into(),
            ]
        } else {
            row![
                iced::widget::text(label).width(Length::FillPortion(6)),
                // .into(),
                iced::widget::text_input("", s)
                    .on_input({
                        let k1 = k.clone();
                        move |a| Message::SettingChanged(k1.clone(), CValue::PickFile(a))
                    })
                    .width(Length::FillPortion(5)),
                // .into(),
                iced::widget::button(
                    text("󰉋")
                        .font(NERD_FONT)
                        .align_x(Horizontal::Center)
                )
                .on_press(Message::FilePicker(k.clone()))
                .width(Length::FillPortion(1)),
                // .into(),
                iced::widget::toggler(uses_default).on_toggle(move |a| {
                    Message::SettingDefaultChanged(k.clone(), a)
                })
                .width(Length::FillPortion(2)),
                // .into(),
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
            iced::widget::button(
                text("󰉋")
                    .font(NERD_FONT)
                    .align_x(Horizontal::Center)
            )
            .on_press(Message::FolderPicker(k.clone()))
            .width(Length::FillPortion(1)),
            iced::widget::toggler(uses_default).on_toggle(move |a| {
                Message::SettingDefaultChanged(k.clone(), a)
            })
            .width(Length::FillPortion(2)),
        ],
    }
    .align_y(Alignment::Center)
}

pub fn get_view_widget(mg: &crate::MainGUI) -> iced::Element<'_, Message> {
    let mut top_bar: iced_aw::widgets::TabBar<Message, crate::GridStatus> =
        TabBar::new(Message::SetGridStatus)
            // .style(TabBarStyles::Blue)
            .push(
                crate::GridStatus::GamesGrid,
                TabLabel::Text("Grid View".to_owned()),
            )
            .push(
                crate::GridStatus::GlobalSettings,
                TabLabel::Text("Global Settings".to_owned()),
            )
            .push(
                crate::GridStatus::AddGame,
                TabLabel::Text("Add Game".to_owned()),
            )
            .set_active_tab(&mg.grid_status)
            .width(Length::FillPortion(3))
            .style(|theme: &Theme, status| {
                let pal = theme.extended_palette();
                match status {
                    iced_aw::card::Status::Active => iced_aw::style::tab_bar::Style {
                        text_color: pal.primary.strong.text,
                        tab_label_background: iced::Background::Color(pal.primary.strong.color),
                        tab_label_border_width: 0.,
                        ..iced_aw::style::tab_bar::Style::default()
                    },
                    iced_aw::card::Status::Hovered => iced_aw::style::tab_bar::Style {
                        text_color: pal.primary.base.text,
                        tab_label_background: iced::Background::Color(pal.primary.base.color),
                        tab_label_border_width: 0.,
                        ..iced_aw::style::tab_bar::Style::default()
                    },
                    iced_aw::card::Status::Disabled => iced_aw::style::tab_bar::Style {
                        text_color: iced::Color::WHITE,
                        tab_label_background: iced::Background::Color(pal.background.strong.color),
                        tab_label_border_width: 0.,
                        ..iced_aw::style::tab_bar::Style::default()
                    },
                    _ => iced_aw::style::tab_bar::Style::default(),
                }
            });

    if mg.selected.is_some() {
        top_bar = top_bar
            .push(
                crate::GridStatus::GamesSettings,
                TabLabel::Text("Game Settings".to_owned()),
            )
            .push(crate::GridStatus::Logs, TabLabel::Text("Logs".to_owned()))
            .set_active_tab(&mg.grid_status)
            .width(Length::FillPortion(5));
    }

    let run_module: iced::Element<'_, Message> = if let Some(g) = mg.selected {
        /*crate::theme::widget*/
        iced::widget::Row::with_children(vec![
            iced::widget::Space::with_width(Length::Fill).into(),
            if !mg.games[g].is_running {
                iced::widget::button(iced::widget::text("run"))
                    .on_press(Message::RunSelected)
                    .into()
            } else {
                iced::widget::button(iced::widget::text("kill"))
                    .on_press(Message::KillSelected)
                    .into()
            },
            iced::widget::pick_list::<'_, _, _, String, _, _, _>(
                mg.games[g].get_subcommands(),
                None,
                Message::RunSubcommandSelected,
            )
            .into(), // .width(Length::Units(30))
        ])
        .align_y(iced::Alignment::End)
        .width(Length::FillPortion(2))
        .into()
    } else {
        iced::widget::Space::with_width(Length::FillPortion(2)).into()
    };

    let top_bar = iced::widget::Row::with_children(vec![
        top_bar.into(),
        if let Some(i) = mg.selected {
            text(crate::duration_to_string(mg.games[i].time_played)).into()
        } else {
            iced::widget::Space::with_width(Length::FillPortion(2)).into()
        },
        run_module,
    ])
    .width(Length::Fill);
    // .align_items(iced::Alignment::End);

    // let choose_theme = [crate::ThemeType::Light, crate::ThemeType::Dark]
    //     .iter()
    //     .fold(column!["Choose a theme:"].spacing(10), |column, option| {
    //         column.push(radio(
    //             format!("{:?}", option),
    //             *option,
    //             Some(*option),
    //             Message::ThemeChanged,
    //         ))
    //     });

    let ge: iced::Element<'_, Message> = match mg.grid_status {
        crate::GridStatus::GamesGrid => {
            let image_size = IMAGE_WIDTH as u16;

            let mut grid: iced_aw::OldGrid<Message, _> =
                iced_aw::OldGrid::with_column_width(image_size as f32 + 20.);
            for (i, g) in mg.games.iter().enumerate() {
                grid.insert::<iced::Element<'_, Message>>(
                    iced::widget::Container::new(
                        iced::widget::button(
                            iced::widget::column(vec![
                                iced::widget::image(iced::widget::image::Handle::from_rgba(
                                    g.image.width(),
                                    g.image.height(),
                                    g.image.as_raw().clone(),
                                ))
                                .filter_method(FilterMethod::Nearest)
                                // .width(Length::Fixed(image_size as f32))
                                .height(Length::Fixed(IMAGE_HEIGHT as f32))
                                .into(),
                                iced::widget::text(g.name.clone()).into(),
                            ])
                            // .padding(iced::Padding::from(0.5))
                            .align_x(iced::Alignment::Center),
                        )
                        .on_press(Message::GameSelected(i))
                        .style(move |theme: &Theme, status| {
                            let palette = theme.extended_palette();
                            if Some(i) != mg.selected {
                                if let iced::widget::button::Status::Hovered = status {
                                    let mut out = button::Style::default()
                                        .with_background(palette.background.strong.color);
                                    out.text_color = iced::Color::WHITE;
                                    out
                                } else {
                                    let mut out = button::Style::default()
                                        .with_background(iced::Color::TRANSPARENT);
                                    out.text_color = iced::Color::WHITE;
                                    out
                                }
                            } else {
                                let mut out = button::Style::default()
                                    .with_background(palette.primary.strong.color);
                                out.text_color = iced::Color::BLACK;
                                out
                            }
                        }), // .height(Length::Fixed(600.))
                    )
                    .padding(iced::Padding::from(0))
                    // .height(Length::Shrink)
                    .into(), // .into(),
                );
            }
            grid.into()
        }
        crate::GridStatus::GamesSettings => {
            let mut options = column![];

            let selected = &mg.games[mg.selected.unwrap()];

            let runner = selected.runner_id.clone();

            for (t, cat) in crate::config::CONFIG_ORDER.clone() {
                let s = t.rsplit(':').collect::<Vec<_>>();
                if s.len() == 1 || s[1] == runner {
                    options = options.push(iced::widget::value(s[0]).size(30));
                    // options = options.push(iced::widget::text(t).size(30));
                    for k in cat {
                        let i = mg.default_config.get(&k).unwrap(); //expect(&format!("{k}")[..]);
                        let s = k.split(':').collect::<Vec<_>>();
                        if s.len() == 1 || s[0] == runner {
                            let (v, uses_default) =
                                if let Some(v) = mg.temp_settings.as_ref().unwrap().0.get(&k) {
                                    (v, false)
                                } else {
                                    (&i.1, true)
                                };
                            let label = i.0.clone() + " : ";
                            options = options.push(get_widget(v, label, k, uses_default))
                        }
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
                .align_y(iced::Alignment::End),
            );

            options.into()
        }
        crate::GridStatus::GlobalSettings => {
            let mut options = column![];

            for (t, cat) in crate::config::CONFIG_ORDER.clone() {
                options = options.push(iced::widget::value(t.rsplit(':').next().unwrap()).size(30));
                // options = options.push(iced::widget::text(t).size(30));
                for k in cat {
                    let i = mg.default_config.get(&k).unwrap(); //expect(&format!("{k}")[..]);
                                                                // let s = k.split(':').collect::<Vec<_>>();
                    let (v, uses_default) =
                        if let Some(v) = mg.temp_settings.as_ref().unwrap().0.get(&k) {
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
                .align_y(iced::Alignment::End),
            );
            options.into()
        }
        crate::GridStatus::AddGame => {
            let mut options = column![];

            let runner = mg
                .temp_settings
                .as_ref()
                .unwrap()
                .0
                .get("runner")
                .unwrap()
                .as_string();

            for (t, cat) in crate::config::CONFIG_ORDER.clone() {
                let s = t.rsplit(':').collect::<Vec<_>>();
                if s.len() == 1 || s[1] == runner {
                    options = options.push(iced::widget::value(s[0]).size(30));
                    // options = options.push(iced::widget::text(t).size(30));
                    for k in cat {
                        let i = mg.default_config.get(&k).unwrap(); //expect(&format!("{k}")[..]);
                        let s = k.split(':').collect::<Vec<_>>();
                        if s.len() == 1 || s[0] == runner {
                            let (v, uses_default) =
                                if let Some(v) = mg.temp_settings.as_ref().unwrap().0.get(&k) {
                                    (v, false)
                                } else {
                                    (&i.1, true)
                                };
                            let label = i.0.clone() + " : ";
                            options = options.push(get_widget(v, label, k, uses_default))
                        }
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
                .align_y(iced::Alignment::End),
            );

            options.into()
        }
        crate::GridStatus::Logs => {
            /* crate::theme::widget::Scrollable::new( */
            iced::widget::Container::new(
                iced::widget::TextEditor::new(
                    {
                        //
                        &mg.log
                    }, //         if let Some(g) = mg.selected {
                       //     &iced::widget::text_editor::Content::with_text(&mg.games[g].current_log)
                       // } else {
                       //     &iced::widget::text_editor::Content::new()
                       // }
                )
                .on_action(Message::LogAction)
                .height(Length::Fill),
            )
            .height(Length::Fill)
            .into()
        }
    };

    let content = column![
        // row![game_viewer, global_settings]
        //     .spacing(10)
        top_bar.height(Length::Fixed(43.)), /* .into() */
        if let crate::GridStatus::Logs = mg.grid_status {
            ge
        } else {
            iced::widget::Container::new(iced::widget::scrollable(ge))
                .height(Length::Fill)
                .into()
            //.height(Length::Fill), /* .into() */
        }
    ]
    .spacing(20)
    .padding(20)
    .width(Length::Fill);

    let content: iced::Element<'_, Message> = if mg.steam_grid_db {
        iced::widget::stack(vec![
            content.into(),
            column![
                iced::widget::Space::with_height(Length::FillPortion(1)),
                iced_aw::Card::new(iced::widget::text("Choose a banner"), {
                    let mut grid: iced_aw::OldGrid<Message, _> =
                        iced_aw::OldGrid::with_column_width(IMAGE_WIDTH as f32 + 20.);
                    for (i, im) in mg.sgdb_images.iter().enumerate() {
                        grid.insert::<iced::Element<'_, Message>>(
                            iced::widget::button(
                                iced::widget::column(vec![iced::widget::image(
                                    iced::widget::image::Handle::from_rgba(
                                        im.1.width(),
                                        im.1.height(),
                                        im.1.as_raw().clone(),
                                    ),
                                )
                                .width(Length::Fixed(IMAGE_WIDTH as f32))
                                .into()])
                                .align_x(iced::Alignment::Center),
                            )
                            .on_press(Message::SGDBThumbSelected(i))
                            .style(move |theme: &Theme, status| {
                                let palette = theme.extended_palette();
                                if Some(i) != mg.sgdb_selected {
                                    if let iced::widget::button::Status::Hovered = status {
                                        let mut out = button::Style::default()
                                            .with_background(palette.background.strong.color);
                                        out.text_color = iced::Color::WHITE;
                                        out
                                    } else {
                                        let mut out = button::Style::default()
                                            .with_background(iced::Color::TRANSPARENT);
                                        out.text_color = iced::Color::WHITE;
                                        out
                                    }
                                } else {
                                    let mut out = button::Style::default()
                                        .with_background(palette.primary.strong.color);
                                    out.text_color = iced::Color::BLACK;
                                    out
                                }
                            })
                            .into(),
                        );
                    }

                    let mut query = vec![iced::widget::text_input("", &mg.sgdb_query[..])
                        .on_input(Message::SGDBChangeQuery)
                        .into()];
                    for g in mg.sgdb_other_possibilities.iter() {
                        query.push(
                            iced::widget::button(iced::widget::text(
                                if let Some(d) = g.release_date {
                                    format!(
                                        "{} ({}) - {}",
                                        g.name,
                                        (chrono::DateTime::UNIX_EPOCH
                                            + std::time::Duration::from_secs(d as u64))
                                        .format("%Y"),
                                        g.id,
                                    )
                                } else {
                                    format!("{} - {}", g.name, g.id,)
                                },
                            ))
                            .on_press(Message::SGDBSelectGame(g.id))
                            .into(),
                        )
                    }
                    let grid: iced::Element<_> = if let crate::SGDBAsyncStatus::NoImage =
                        mg.sgdb_async_status
                    {
                        iced::widget::text("There is no available grid for this game.").into()
                    } else if let crate::SGDBAsyncStatus::ImageDownload(_) = mg.sgdb_async_status {
                        iced::widget::column![iced::widget::text("Loading images..."), grid].into()
                    } else {
                        grid.into()
                    };

                    iced::widget::column![
                        iced::widget::scrollable(column![
                            iced::widget::Column::with_children(query),
                            grid,
                        ])
                        .height(Length::FillPortion(16)),
                        row![
                            iced::widget::button(iced::widget::text("Cancel"))
                                .on_press(Message::CancelSGDB),
                            iced::widget::button(iced::widget::text("Ok"))
                                .on_press(Message::ApplySGDB),
                        ]
                        .height(Length::FillPortion(1))
                    ]
                })
                .style(|theme: &Theme, _status| {
                    let palette = theme.extended_palette();
                    let color = palette.primary.strong.color;
                    let text_color = palette.primary.strong.text;
                    let foreground = theme.palette();

                    iced_aw::widget::card::Style {
                        border_color: color,
                        head_background: color.into(),
                        head_text_color: text_color,
                        close_color: text_color,
                        background: palette.background.base.color.into(),
                        body_text_color: foreground.text,
                        foot_text_color: foreground.text,
                        ..iced_aw::widget::card::Style::default()
                    }
                })
                .height(Length::FillPortion(10)),
                iced::widget::Space::with_height(Length::FillPortion(1)),
            ]
            .into(),
        ])
        .into()
    } else {
        content.into()
    };
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center(iced::Length::Fill)
        .into()
}
