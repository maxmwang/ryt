use cursive::view::Resizable;
use cursive::views::{ResizedView, TextView};

pub fn title_view() -> ResizedView<TextView> {
    let title_text = (vec![
        r"         _    _        _        _       ",
        r"        /\ \ /\ \     /\_\     /\ \     ",
        r"       /  \ \\ \ \   / / /     \_\ \    ",
        r"      / /\ \ \\ \ \_/ / /      /\__ \   ",
        r"     / / /\ \_\\ \___/ /      / /_ \ \  ",
        r"    / / /_/ / / \ \ \_/      / / /\ \ \ ",
        r"   / / /__\/ /   \ \ \      / / /  \/_/ ",
        r"  / / /_____/     \ \ \    / / /        ",
        r" / / /\ \ \        \ \ \  / / /         ",
        r"/ / /  \ \ \        \ \_\/_/ /          ",
        r"\/_/    \_\/         \/_/\_\/           ",
    ])
    .join("\n");

    let title = TextView::new(title_text).center().full_screen();

    title
}
