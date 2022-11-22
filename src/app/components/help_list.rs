use cursive::{
    view::Resizable,
    views::{ListView, ResizedView, TextView},
};

pub fn help_list_component() -> ResizedView<ListView> {
    ListView::new()
        .child("q", TextView::new("Quit"))
        .child("/h", TextView::new("Help Page"))
        .child(
            "Long Command",
            TextView::new("Very long Command Description"),
        )
        .full_screen()
}
