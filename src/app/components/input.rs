use cursive::align::HAlign::Left;
use cursive::view::{Nameable, Resizable};
use cursive::views::{Dialog, EditView};

use crate::app::api;
use crate::app::views::results;

pub fn input_component() -> Dialog {
    let input = EditView::new()
        .filler("█")
        .on_submit(|s, input| {
            s.pop_layer();

            let results = api::search(input).ok();

            s.add_layer(results::results_view(results.unwrap()));
        })
        .with_name("input")
        .full_width();

    Dialog::around(input)
        .title("Search of [h]elp")
        .title_position(Left)
}
