use cursive::{
    align::HAlign::Left,
    view::Resizable,
    views::{Dialog, EditView},
    Cursive,
};

use crate::app::{
    api,
    views::{help, results},
};

pub fn input_component() -> Dialog {
    let input = EditView::new()
        .filler("█")
        .on_submit(handle_submit)
        .full_width();

    Dialog::around(input)
        .title("Search or [/h]elp")
        .title_position(Left)
}

fn handle_submit(siv: &mut Cursive, input: &str) {
    siv.pop_layer();

    siv.add_layer(results::results_view(api::search(input).unwrap()));

    match input {
        "/h" => siv.add_layer(help::help_view()),
        _ => siv.add_layer(results::results_view(api::search(input).unwrap())),
    }
}
