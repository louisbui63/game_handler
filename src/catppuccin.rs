pub fn frappe() -> iced::theme::Custom {
    iced::theme::Custom::new(iced::theme::Palette {
        background: iced::Color::from_rgb(48. / 255., 52. / 255., 70. / 255.),
        text: iced::Color::from_rgb(198. / 255., 208. / 255., 245. / 255.),
        primary: iced::Color::from_rgb(98. / 255., 104. / 255., 128. / 255.),
        success: iced::Color::from_rgb(166. / 255., 209. / 255., 137. / 255.),
        danger: iced::Color::from_rgb(242. / 255., 213. / 255., 207. / 255.),
    })
}
