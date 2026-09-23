use web_page_main_canvas_lib as lib;
use lib::MainLayout;
use lib::js::alert;

fn main() -> iced::Result<> {
    console_error_panic_hook::set_once();
    let theme = iced::Theme::Dark;
    let app = iced::application(MainLayout::new, MainLayout::update, MainLayout::view)
        .theme(theme)
        .subscription(MainLayout::subscription);
    app.run()
}
