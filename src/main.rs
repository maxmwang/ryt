use cursive;
use cursive::traits::With;
use cursive::views::TextView;

const TITLE: &str = "
      ___
     /  /\\          ___         ___
    /  /::\\        /__/|       /  /\\
   /  /:/\\:\\      |  |:|      /  /:/
  /  /:/~/:/      |  |:|     /  /:/
 /__/:/ /:/___  __|__|:|    /  /::\\
 \\  \\:\\/:::::/ /__/::::\\   /__/:/\\:\\
  \\  \\::/~~~~     ~\\~~\\:\\  \\__\\/  \\:\\
   \\  \\:\\           \\  \\:\\      \\  \\:\\
    \\  \\:\\           \\__\\/       \\__\\/
     \\__\\/
";

fn main() {
    let mut siv = cursive::default();

    load_theme(&mut siv);
    load_global_listeners(&mut siv);

    let title = TextView::new(TITLE);

    siv.add_layer(title);

    siv.run();
}

fn load_theme(siv: &mut cursive::Cursive) {
    use cursive::theme;

    siv.set_theme(theme::Theme {
        shadow: false,
        borders: theme::BorderStyle::Simple,
        palette: theme::Palette::default().with(|palette| {
            use cursive::theme::BaseColor::*;
            use cursive::theme::Color::*;
            use cursive::theme::PaletteColor::*;

            palette[Background] = TerminalDefault;
            palette[View] = TerminalDefault;
            palette[Primary] = White.dark();
            palette[Highlight] = TerminalDefault;
        }),
    });
}

fn load_global_listeners(siv: &mut cursive::Cursive) {
    siv.add_global_callback('q', |s| s.quit());
}
