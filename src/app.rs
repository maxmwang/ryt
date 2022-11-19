mod api;
mod components;
mod config;
mod mpv;
mod views;

pub fn app() {
    let mut siv = cursive::crossterm();

    config::load_config(&mut siv);

    siv.add_layer(views::home::home_view());

    siv.run();
}
