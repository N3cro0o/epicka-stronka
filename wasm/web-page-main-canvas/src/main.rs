use web_page_main_canvas_lib as lib;
use lib::MainLayout;
use lib::js::alert;

fn main() -> iced::Result<> {
    console_error_panic_hook::set_once();
    alert("Jajo");
    let app = iced::application(MainLayout::new, MainLayout::update, MainLayout::view)
        .subscription(MainLayout::subscription);
    app.run()
}
