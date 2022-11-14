use cursive::align::HAlign;
use cursive::view::{Nameable, Resizable};
use cursive::views::{Dialog, EditView};

pub fn input_view() -> Dialog {
    let input = EditView::new()
        .filler("█")
        .on_submit(|s, input| {
            s.pop_layer();
            s.add_layer(Dialog::info(format!("You entered {}", input)));
        })
        .with_name("input")
        .full_width();

    let input_wrapper = Dialog::around(input)
        .title("Search of [h]elp")
        .title_position(HAlign::Left);

    input_wrapper
}
