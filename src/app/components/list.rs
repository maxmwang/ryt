use cursive::view::Resizable;
use cursive::views::{ResizedView, SelectView};

use crate::app::api::MyVideo;
use crate::app::command;

pub fn list_component(lst: Vec<MyVideo>) -> ResizedView<SelectView> {
    let mut list = SelectView::new().on_submit(|_, id: &String| {
        command::play(id);
    });

    list.add_all(
        lst.into_iter()
            .enumerate()
            .map(|(i, item)| (format!("{: >2}: {}", i, item.title), item.id))
            .collect::<Vec<(String, String)>>(),
    );

    list.full_screen()
}
