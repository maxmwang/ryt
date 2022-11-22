use cursive::view::Resizable;
use cursive::views::{ResizedView, SelectView};

use crate::app::{api::MyVideo, mpv};

pub fn list_component(lst: Vec<MyVideo>) -> ResizedView<SelectView> {
    SelectView::new()
        .with_all(
            lst.into_iter()
                .enumerate()
                .map(|(i, item)| (format!("{: >2}: {}", i + 1, item.title), item.id))
                .collect::<Vec<(String, String)>>(),
        )
        .on_submit(|_, id| {
            mpv::play(id);
        })
        .full_screen()
}
