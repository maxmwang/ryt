use cursive;

mod components;
mod config;
mod views;

pub fn app() {
    let mut siv = cursive::default();

    config::load_config(&mut siv);

    siv.add_layer(views::home::home_view());

    siv.run();
}
