pub mod widget {
    use iced::advanced::text::highlighter::PlainText;

    pub type Renderer = iced::Renderer;
    pub type Element<'a, Message> = iced::Element<'a, Message, Renderer>;
    pub type Row<'a, Message> = iced::widget::Row<'a, Message, Renderer>;
    pub type TextEditor<'a, Message> = iced::widget::TextEditor<'a, PlainText, Message, Renderer>;
}
#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct CurrentTheme {
    app: application::Appearance,
    text: text::Appearance,
    container: container::Appearance,
    button: button::Appearance,
    grid_unselected: button::Appearance,
    radio_active_selected: radio::Appearance,
    radio_active_unselected: radio::Appearance,
    radio_hovered_selected: radio::Appearance,
    radio_hovered_unselected: radio::Appearance,
    tab_bar_active_active: tab_bar::Appearance,
    tab_bar_active_inactive: tab_bar::Appearance,
    tab_bar_hovered_active: tab_bar::Appearance,
    tab_bar_hovered_inactive: tab_bar::Appearance,
    text_input_active: text_input::Appearance,
    text_input_focused: text_input::Appearance,
    text_input_placeholder_color: iced::Color,
    text_input_value_color: iced::Color,
    text_input_disabled_color: iced::Color,
    text_input_selection_color: iced::Color,
    text_input_disabled: text_input::Appearance,
    card_active: card::Appearance,
    modal_active: iced_aw::style::modal::Appearance,
    toggler_active_active: toggler::Appearance,
    toggler_active_inactive: toggler::Appearance,
    toggler_hovered_active: toggler::Appearance,
    toggler_hovered_inactive: toggler::Appearance,
    scrollable_active: scrollable::Appearance,
    scrollable_hovered_mouse_over: scrollable::Appearance,
    scrollable_hovered_no_mouse_over: scrollable::Appearance,
    pick_list_active: pick_list::Appearance,
    pick_list_hovered: pick_list::Appearance,
    overlay_menu: overlay::menu::Appearance,
    text_editor_active: text_editor::Appearance,
    text_editor_focused: text_editor::Appearance,
    text_editor_disabled: text_editor::Appearance,
    text_editor_placeholder_color: iced::Color,
    text_editor_value_color: iced::Color,
    text_editor_disabled_color: iced::Color,
    text_editor_selection_color: iced::Color,
}

use std::sync::Mutex;

lazy_static::lazy_static! {
    pub static ref CURRENT_THEME: Mutex<CurrentTheme> = Mutex::new(CurrentTheme::default());
}

#[derive(Default)]
pub struct Theme;

use iced::{
    application, overlay,
    widget::{
        button, container, pick_list, radio, scrollable, text, text_editor, text_input, toggler,
    },
};
use iced_aw::{modal, style::card, tab_bar};

impl text_editor::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> text_editor::Appearance {
        CURRENT_THEME.lock().unwrap().text_editor_active
    }

    fn focused(&self, _style: &Self::Style) -> text_editor::Appearance {
        CURRENT_THEME.lock().unwrap().text_editor_focused
    }

    fn placeholder_color(&self, _style: &Self::Style) -> iced::Color {
        CURRENT_THEME.lock().unwrap().text_editor_placeholder_color
    }

    fn value_color(&self, _style: &Self::Style) -> iced::Color {
        CURRENT_THEME.lock().unwrap().text_editor_value_color
    }

    fn disabled_color(&self, _style: &Self::Style) -> iced::Color {
        CURRENT_THEME.lock().unwrap().text_editor_disabled_color
    }

    fn selection_color(&self, _style: &Self::Style) -> iced::Color {
        CURRENT_THEME.lock().unwrap().text_editor_selection_color
    }

    fn disabled(&self, _style: &Self::Style) -> text_editor::Appearance {
        CURRENT_THEME.lock().unwrap().text_editor_disabled
    }
}

impl application::StyleSheet for Theme {
    type Style = ();
    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        CURRENT_THEME.lock().unwrap().app
    }
}

impl text::StyleSheet for Theme {
    type Style = ();
    fn appearance(&self, _style: Self::Style) -> iced::widget::text::Appearance {
        CURRENT_THEME.lock().unwrap().text
    }
}

impl container::StyleSheet for Theme {
    type Style = ();
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        CURRENT_THEME.lock().unwrap().container
    }
}

#[derive(Default)]
pub enum ButtonStyle {
    #[default]
    Normal,
    GridUnselected,
    GridSelected,
}

impl button::StyleSheet for Theme {
    type Style = ButtonStyle;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        match style {
            ButtonStyle::Normal | ButtonStyle::GridSelected => CURRENT_THEME.lock().unwrap().button,
            ButtonStyle::GridUnselected => CURRENT_THEME.lock().unwrap().grid_unselected,
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        button::Appearance {
            shadow_offset: active.shadow_offset + iced::Vector::new(0.0, 0.0),
            background: if let ButtonStyle::Normal = style {
                active.background.map(|background| match background {
                    iced::Background::Color(color) => iced::Background::Color(iced::Color {
                        a: color.a * 0.5,
                        ..color
                    }),
                    iced::Background::Gradient(gradient) => {
                        iced::Background::Gradient(gradient.mul_alpha(0.5))
                    }
                })
            } else if let ButtonStyle::GridUnselected = style {
                active.background.map(|background| match background {
                    iced::Background::Color(color) => iced::Background::Color(iced::Color {
                        r: color.r * 0.8,
                        g: color.g * 0.8,
                        b: color.b * 0.8,
                        ..color
                    }),
                    iced::Background::Gradient(gradient) => {
                        iced::Background::Gradient(gradient.mul_alpha(0.5))
                    }
                })
            } else {
                active.background
            },
            ..active
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        button::Appearance {
            shadow_offset: iced::Vector::default(),
            ..active
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        button::Appearance {
            shadow_offset: iced::Vector::default(),
            background: active.background.map(|background| match background {
                iced::Background::Color(color) => iced::Background::Color(iced::Color {
                    a: color.a * 0.5,
                    ..color
                }),
                iced::Background::Gradient(gradient) => {
                    iced::Background::Gradient(gradient.mul_alpha(0.5))
                }
            }),
            text_color: iced::Color {
                a: active.text_color.a * 0.5,
                ..active.text_color
            },
            ..active
        }
    }
}

impl radio::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style, is_selected: bool) -> radio::Appearance {
        if is_selected {
            CURRENT_THEME.lock().unwrap().radio_active_selected
        } else {
            CURRENT_THEME.lock().unwrap().radio_active_unselected
        }
    }

    fn hovered(&self, _style: &Self::Style, is_selected: bool) -> radio::Appearance {
        if is_selected {
            CURRENT_THEME.lock().unwrap().radio_hovered_selected
        } else {
            CURRENT_THEME.lock().unwrap().radio_hovered_unselected
        }
    }
}

impl tab_bar::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style, is_active: bool) -> tab_bar::Appearance {
        if is_active {
            CURRENT_THEME.lock().unwrap().tab_bar_active_active
        } else {
            CURRENT_THEME.lock().unwrap().tab_bar_active_inactive
        }
    }

    fn hovered(&self, _style: &Self::Style, is_active: bool) -> tab_bar::Appearance {
        if is_active {
            CURRENT_THEME.lock().unwrap().tab_bar_hovered_active
        } else {
            CURRENT_THEME.lock().unwrap().tab_bar_hovered_inactive
        }
    }
}

impl text_input::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        CURRENT_THEME.lock().unwrap().text_input_active
    }

    fn focused(&self, _style: &Self::Style) -> text_input::Appearance {
        CURRENT_THEME.lock().unwrap().text_input_focused
    }

    fn placeholder_color(&self, _style: &Self::Style) -> iced::Color {
        CURRENT_THEME.lock().unwrap().text_input_placeholder_color
    }

    fn value_color(&self, _style: &Self::Style) -> iced::Color {
        CURRENT_THEME.lock().unwrap().text_input_value_color
    }

    fn disabled_color(&self, _style: &Self::Style) -> iced::Color {
        CURRENT_THEME.lock().unwrap().text_input_disabled_color
    }

    fn selection_color(&self, _style: &Self::Style) -> iced::Color {
        CURRENT_THEME.lock().unwrap().text_input_selection_color
    }

    fn disabled(&self, _style: &Self::Style) -> text_input::Appearance {
        CURRENT_THEME.lock().unwrap().text_input_disabled
    }
}

impl card::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> iced_aw::card::Appearance {
        CURRENT_THEME.lock().unwrap().card_active
    }
}

impl modal::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> iced_aw::style::modal::Appearance {
        CURRENT_THEME.lock().unwrap().modal_active
    }
}

impl toggler::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style, is_active: bool) -> toggler::Appearance {
        if is_active {
            CURRENT_THEME.lock().unwrap().toggler_active_active
        } else {
            CURRENT_THEME.lock().unwrap().toggler_active_inactive
        }
    }

    fn hovered(&self, _style: &Self::Style, is_active: bool) -> toggler::Appearance {
        if is_active {
            CURRENT_THEME.lock().unwrap().toggler_hovered_active
        } else {
            CURRENT_THEME.lock().unwrap().toggler_hovered_inactive
        }
    }
}

impl scrollable::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> scrollable::Appearance {
        CURRENT_THEME.lock().unwrap().scrollable_active
    }

    fn hovered(
        &self,
        _style: &Self::Style,
        is_mouse_over_scrollbar: bool,
    ) -> scrollable::Appearance {
        if is_mouse_over_scrollbar {
            CURRENT_THEME.lock().unwrap().scrollable_hovered_mouse_over
        } else {
            CURRENT_THEME
                .lock()
                .unwrap()
                .scrollable_hovered_no_mouse_over
        }
    }
}

impl pick_list::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &<Self as pick_list::StyleSheet>::Style) -> pick_list::Appearance {
        CURRENT_THEME.lock().unwrap().pick_list_active
    }

    fn hovered(&self, _style: &<Self as pick_list::StyleSheet>::Style) -> pick_list::Appearance {
        CURRENT_THEME.lock().unwrap().pick_list_hovered
    }
}

impl overlay::menu::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> overlay::menu::Appearance {
        CURRENT_THEME.lock().unwrap().overlay_menu
    }
}

pub static EMBEDDED_THEME: &'static str = r#"
[app.background_color]
r = 0.19
g = 0.20
b = 0.27
a = 1.0

[app.text_color]
r = 0.73
g = 0.76
b = 0.90
a = 1.0

[text]

[container.border]
width = 0.0
radius = [
    0.0,
    0.0,
    0.0,
    0.0,
]

[container.border.color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[container.shadow]
blur_radius = 0.0

[container.shadow.color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[container.shadow.offset]
x = 0.0
y = 0.0

[button.shadow_offset]
x = 0.0
y = 0.0

[button.text_color]
r = 0.19
g = 0.20
b = 0.27
a = 1.0

[button.background.Color]
r = 1.0
g = 0.67
b = 1.0
a = 1.0

[button.border]
width = 0.0
radius = [
    0.0,
    0.0,
    0.0,
    0.0,
]

[button.border.color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[button.shadow]
blur_radius = 0.0

[button.shadow.color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[button.shadow.offset]
x = 0.0
y = 0.0

[grid_unselected.shadow_offset]
x = 0.0
y = 0.0

[grid_unselected.text_color]
r = 0.73
g = 0.76
b = 0.90
a = 1.0

[grid_unselected.background.Color]
r = 0.19
g = 0.20
b = 0.27
a = 1.0

[grid_unselected.border]
width = 0.0
radius = [
    0.0,
    0.0,
    0.0,
    0.0,
]

[grid_unselected.border.color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[grid_unselected.shadow]
blur_radius = 0.0

[grid_unselected.shadow.color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[grid_unselected.shadow.offset]
x = 0.0
y = 0.0

[radio_active_selected]
border_width = 0.0

[radio_active_selected.background.Color]
r = 1.0
g = 0.67
b = 1.0
a = 1.0

[radio_active_selected.dot_color]
r = 0.73
g = 0.76
b = 0.90
a = 1.0

[radio_active_selected.border_color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[radio_active_unselected]
border_width = 0.0

[radio_active_unselected.background.Color]
r = 0.76
g = 0.60
b = 0.76
a = 1.0

[radio_active_unselected.dot_color]
r = 0.73
g = 0.76
b = 0.90
a = 1.0

[radio_active_unselected.border_color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[radio_hovered_selected]
border_width = 0.0

[radio_hovered_selected.background.Color]
r = 1.0
g = 0.67
b = 1.0
a = 1.0

[radio_hovered_selected.dot_color]
r = 0.73
g = 0.76
b = 0.90
a = 1.0

[radio_hovered_selected.border_color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[radio_hovered_unselected]
border_width = 0.0

[radio_hovered_unselected.background.Color]
r = 1.0
g = 0.67
b = 1.0
a = 1.0

[radio_hovered_unselected.dot_color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[radio_hovered_unselected.border_color]
r = 0.73
g = 0.76
b = 0.90
a = 1.0

[tab_bar_active_active]
border_width = 0.0
tab_label_border_width = 0.0
icon_border_radius = [
    4.0,
    4.0,
    4.0,
    4.0,
]

[tab_bar_active_active.tab_label_background.Color]
r = 1.0
g = 0.67
b = 1.0
a = 1.0

[tab_bar_active_active.tab_label_border_color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[tab_bar_active_active.icon_color]
r = 0.0
g = 0.0
b = 0.0
a = 1.0

[tab_bar_active_active.icon_background.Color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[tab_bar_active_active.text_color]
r = 0.19
g = 0.20
b = 0.27
a = 1.0

[tab_bar_active_inactive]
border_width = 0.0
tab_label_border_width = 0.0
icon_border_radius = [
    4.0,
    4.0,
    4.0,
    4.0,
]

[tab_bar_active_inactive.tab_label_background.Color]
r = 0.74
g = 0.68
b = 0.88
a = 1.0

[tab_bar_active_inactive.tab_label_border_color]
r = 0.699999988079071
g = 0.699999988079071
b = 0.699999988079071
a = 1.0

[tab_bar_active_inactive.icon_color]
r = 0.0
g = 0.0
b = 0.0
a = 1.0

[tab_bar_active_inactive.icon_background.Color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[tab_bar_active_inactive.text_color]
r = 0.19
g = 0.20
b = 0.27
a = 1.0

[tab_bar_hovered_active]
border_width = 0.0
tab_label_border_width = 0.0
icon_border_radius = [
    4.0,
    4.0,
    4.0,
    4.0,
]

[tab_bar_hovered_active.tab_label_background.Color]
r = 0.91
g = 0.61
b = 0.91
a = 1.0

[tab_bar_hovered_active.tab_label_border_color]
r = 0.699999988079071
g = 0.699999988079071
b = 0.699999988079071
a = 1.0

[tab_bar_hovered_active.icon_color]
r = 0.0
g = 0.0
b = 0.0
a = 1.0

[tab_bar_hovered_active.icon_background.Color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[tab_bar_hovered_active.text_color]
r = 0.19
g = 0.20
b = 0.27
a = 1.0

[tab_bar_hovered_inactive]
border_width = 0.0
tab_label_border_width = 0.0
icon_border_radius = [
    4.0,
    4.0,
    4.0,
    4.0,
]

[tab_bar_hovered_inactive.tab_label_background.Color]
r = 0.61
g = 0.56
b = 0.73
a = 1.0

[tab_bar_hovered_inactive.tab_label_border_color]
r = 0.699999988079071
g = 0.699999988079071
b = 0.699999988079071
a = 1.0

[tab_bar_hovered_inactive.icon_color]
r = 0.0
g = 0.0
b = 0.0
a = 1.0

[tab_bar_hovered_inactive.icon_background.Color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[tab_bar_hovered_inactive.text_color]
r = 0.19
g = 0.20
b = 0.27
a = 1.0

[text_input_active.background.Color]
r = 0.19
g = 0.20
b = 0.27
a = 1.0

[text_input_active.border]
width = 1.0
radius = [
    0.0,
    0.0,
    0.0,
    0.0,
]

[text_input_active.border.color]
r = 0.73
g = 0.76
b = 0.90
a = 1.0

[text_input_active.icon_color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[text_input_focused.background.Color]
r = 0.19
g = 0.20
b = 0.27
a = 1.0

[text_input_focused.border]
width = 1.0
radius = [
    0.0,
    0.0,
    0.0,
    0.0,
]

[text_input_focused.border.color]
r = 0.73
g = 0.76
b = 0.90
a = 1.0

[text_input_focused.icon_color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[text_input_placeholder_color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[text_input_value_color]
r = 0.73
g = 0.76
b = 0.90
a = 1.0

[text_input_disabled_color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[text_input_selection_color]
r = 0.73
g = 0.76
b = 0.90
a = 1.0

[text_input_disabled.background.Color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[text_input_disabled.border]
width = 0.0
radius = [
    0.0,
    0.0,
    0.0,
    0.0,
]

[text_input_disabled.border.color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[text_input_disabled.icon_color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[card_active]
border_radius = 10.0
border_width = 1.0

[card_active.background.Color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[card_active.border_color]
r = 1.0
g = 0.67
b = 1.0
a = 1.0

[card_active.head_background.Color]
r = 1.0
g = 0.67
b = 1.0
a = 1.0

[card_active.head_text_color]
r = 0.19
g = 0.20
b = 0.27
a = 1.0

[card_active.body_background.Color]
r = 0.19
g = 0.20
b = 0.27
a = 1.0

[card_active.body_text_color]
r = 0.73
g = 0.76
b = 0.90
a = 1.0

[card_active.foot_background.Color]
r = 0.0
g = 0.0
b = 0.0
a = 1.0

[card_active.foot_text_color]
r = 0.0
g = 0.0
b = 0.0
a = 1.0

[card_active.close_color]
r = 0.0
g = 0.0
b = 0.0
a = 1.0

[modal_active.background.Color]
r = 1.0
g = 1.0
b = 1.0
a = 0.1

[toggler_active_active]
background_border_width = 0.0
foreground_border_width = 0.0

[toggler_active_active.background]
r = 1.0
g = 0.67
b = 1.0
a = 1.0

[toggler_active_active.background_border_color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[toggler_active_active.foreground]
r = 0.19
g = 0.20
b = 0.27
a = 1.0

[toggler_active_active.foreground_border_color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[toggler_active_inactive]
background_border_width = 0.0
foreground_border_width = 0.0

[toggler_active_inactive.background]
r = 0.74
g = 0.68
b = 0.88
a = 1.0

[toggler_active_inactive.background_border_color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[toggler_active_inactive.foreground]
r = 0.19
g = 0.20
b = 0.27
a = 1.0

[toggler_active_inactive.foreground_border_color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[toggler_hovered_active]
background_border_width = 0.0
foreground_border_width = 0.0

[toggler_hovered_active.background]
r = 1.0
g = 0.67
b = 1.0
a = 1.0

[toggler_hovered_active.background_border_color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[toggler_hovered_active.foreground]
r = 0.19
g = 0.20
b = 0.27
a = 1.0

[toggler_hovered_active.foreground_border_color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[toggler_hovered_inactive]
background_border_width = 0.0
foreground_border_width = 0.0

[toggler_hovered_inactive.background]
r = 0.74
g = 0.68
b = 0.88
a = 1.0

[toggler_hovered_inactive.background_border_color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[toggler_hovered_inactive.foreground]
r = 0.19
g = 0.20
b = 0.27
a = 1.0

[toggler_hovered_inactive.foreground_border_color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[scrollable_active.container.border]
width = 0.0
radius = [
    0.0,
    0.0,
    0.0,
    0.0,
]

[scrollable_active.container.border.color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[scrollable_active.container.shadow]
blur_radius = 0.0

[scrollable_active.container.shadow.color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[scrollable_active.container.shadow.offset]
x = 0.0
y = 0.0

[scrollable_active.scrollbar.border]
width = 0.0
radius = [
    0.0,
    0.0,
    0.0,
    0.0,
]

[scrollable_active.scrollbar.border.color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[scrollable_active.scrollbar.scroller.color]
r = 0.73
g = 0.76
b = 0.90
a = 1.0

[scrollable_active.scrollbar.scroller.border]
width = 0.0
radius = [
    0.0,
    0.0,
    0.0,
    0.0,
]

[scrollable_active.scrollbar.scroller.border.color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[scrollable_hovered_mouse_over.container.border]
width = 0.0
radius = [
    0.0,
    0.0,
    0.0,
    0.0,
]

[scrollable_hovered_mouse_over.container.border.color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[scrollable_hovered_mouse_over.container.shadow]
blur_radius = 0.0

[scrollable_hovered_mouse_over.container.shadow.color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[scrollable_hovered_mouse_over.container.shadow.offset]
x = 0.0
y = 0.0

[scrollable_hovered_mouse_over.scrollbar.border]
width = 0.0
radius = [
    0.0,
    0.0,
    0.0,
    0.0,
]

[scrollable_hovered_mouse_over.scrollbar.border.color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[scrollable_hovered_mouse_over.scrollbar.scroller.color]
r = 1.0
g = 0.67
b = 1.0
a = 1.0

[scrollable_hovered_mouse_over.scrollbar.scroller.border]
width = 0.0
radius = [
    0.0,
    0.0,
    0.0,
    0.0,
]

[scrollable_hovered_mouse_over.scrollbar.scroller.border.color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[scrollable_hovered_no_mouse_over.container.border]
width = 0.0
radius = [
    0.0,
    0.0,
    0.0,
    0.0,
]

[scrollable_hovered_no_mouse_over.container.border.color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[scrollable_hovered_no_mouse_over.container.shadow]
blur_radius = 0.0

[scrollable_hovered_no_mouse_over.container.shadow.color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[scrollable_hovered_no_mouse_over.container.shadow.offset]
x = 0.0
y = 0.0

[scrollable_hovered_no_mouse_over.scrollbar.border]
width = 0.0
radius = [
    0.0,
    0.0,
    0.0,
    0.0,
]

[scrollable_hovered_no_mouse_over.scrollbar.border.color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[scrollable_hovered_no_mouse_over.scrollbar.scroller.color]
r = 0.73
g = 0.76
b = 0.90
a = 1.0

[scrollable_hovered_no_mouse_over.scrollbar.scroller.border]
width = 0.0
radius = [
    0.0,
    0.0,
    0.0,
    0.0,
]

[scrollable_hovered_no_mouse_over.scrollbar.scroller.border.color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[pick_list_active.text_color]
r = 0.73
g = 0.76
b = 0.90
a = 1.0

[pick_list_active.placeholder_color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[pick_list_active.handle_color]
r = 1.0
g = 0.67
b = 1.0
a = 1.0

[pick_list_active.background.Color]
r = 0.19
g = 0.20
b = 0.27
a = 1.0

[pick_list_active.border]
width = 1.0
radius = [
    0.0,
    0.0,
    0.0,
    0.0,
]

[pick_list_active.border.color]
r = 0.73
g = 0.76
b = 0.90
a = 1.0

[pick_list_hovered.text_color]
r = 0.73
g = 0.76
b = 0.90
a = 1.0

[pick_list_hovered.placeholder_color]
r = 0.0
g = 0.0
b = 0.0
a = 0.0

[pick_list_hovered.handle_color]
r = 1.0
g = 0.67
b = 1.0
a = 1.0

[pick_list_hovered.background.Color]
r = 0.19
g = 0.20
b = 0.27
a = 1.0

[pick_list_hovered.border]
width = 1.0
radius = [
    0.0,
    0.0,
    0.0,
    0.0,
]

[pick_list_hovered.border.color]
r = 0.73
g = 0.76
b = 0.90
a = 1.0

[overlay_menu.text_color]
r = 0.73
g = 0.76
b = 0.90
a = 1.0

[overlay_menu.background.Color]
r = 0.19
g = 0.20
b = 0.27
a = 1.0

[overlay_menu.border]
width = 1.0
radius = [
    0.0,
    0.0,
    0.0,
    0.0,
]

[overlay_menu.border.color]
r = 0.73
g = 0.76
b = 0.90
a = 1.0

[overlay_menu.selected_text_color]
r = 0.19
g = 0.20
b = 0.27
a = 1.0

[overlay_menu.selected_background.Color]
r = 1.0
g = 0.67
b = 1.0
a = 1.0

[text_editor_active.background.Color]
r = 0.19
g = 0.20
b = 0.27
a = 1.0

[text_editor_active.border]
width = 1.0
radius = [
    0.0,
    0.0,
    0.0,
    0.0,
]

[text_editor_active.border.color]
r = 0.73
g = 0.76
b = 0.90
a = 1.0

[text_editor_focused.background.Color]
r = 0.19
g = 0.20
b = 0.27
a = 1.0

[text_editor_focused.border]
width = 1.0
radius = [
    0.0,
    0.0,
    0.0,
    0.0,
]

[text_editor_focused.border.color]
r = 0.73
g = 0.76
b = 0.90
a = 1.0

[text_editor_disabled.background.Color]
r = 0.19
g = 0.20
b = 0.27
a = 1.0

[text_editor_disabled.border]
width = 1.0
radius = [
    0.0,
    0.0,
    0.0,
    0.0,
]

[text_editor_disabled.border.color]
r = 0.73
g = 0.76
b = 0.90
a = 1.0

[text_editor_placeholder_color]
r = 0.73
g = 0.76
b = 0.90
a = 1.0

[text_editor_value_color]
r = 0.73
g = 0.76
b = 0.90
a = 1.0

[text_editor_disabled_color]
r = 0.73
g = 0.76
b = 0.90
a = 1.0

[text_editor_selection_color]
r = 0.76
g = 0.60
b = 0.76
a = 1.0
"#;
