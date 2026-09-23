use iced::{stream, Element, Length, Subscription, Pixels, Size};
use iced::widget::{stack, scrollable, space, text, container, center, row, column, mouse_area, rule};
use iced::widget::container::Container;
use iced::futures::channel::mpsc;
use iced::futures::sink::SinkExt;
use iced::futures::Stream;

use std::convert::From;
use std::sync::OnceLock;

#[cfg_attr(target_family = "wasm", path = "js.rs")]
#[cfg_attr(not(target_family = "wasm"), path = "no_js.rs")]
pub mod js;

pub const APP_SPACING: Pixels = Pixels(5.0);
pub const APP_PADDING: Pixels = Pixels(10.0);
const APP_WINDOW_TRIGGER_PERCENT: f32 = 0.3;

pub static HTML_SENDER: OnceLock<mpsc::Sender<Input>> = OnceLock::new(); 

#[derive(Debug, Clone, PartialEq)]
pub enum AppPage {
    AboutMe(u8),
    AboutPage,
    Projects,
}

impl From<i32> for AppPage {
    fn from(value: i32) -> Self {
        match value {
            0 => AppPage::AboutMe(0),
            1 => AppPage::Projects, 
            2 => AppPage::AboutPage,
            _ => {
                unsafe { js::log("Wrong AppPage id"); }
                AppPage::AboutMe(0)
            }
        }
    }
}

impl Default for AppPage {
    fn default() -> Self {
        AppPage::AboutMe(0)
    }
}

impl AppPage {
    pub fn view(&self) -> Element<'_, MainMessage> {
        match self {
            AppPage::AboutMe(val) => { Self::me_view((*val).into()) }
            _ => { Self::me_view(0) }
        }   
    }

    fn me_view(val: usize) -> Element<'static, MainMessage> {
        let c = match val {
            0 => {
                column![
                    text("On the Internet you can find me as N3cro0o. I'm aspiring student, software and game developer. Everyday I'm trying to make better and better projects and polish the old ones to perfection. And most importantly I tried to spread this mindset to others.")
                        .center()
                        .width(Length::Fill),
                    text("I think of myself as an artist, I like to draw and create new handmande assets for each of my projects. By being creative and not fearing chasing knowledge I don't fear expressing myself. You can check this for yourself by looking at my past projects :p")
                        .center()
                        .width(Length::Fill),
                    space::vertical().height(Length::Fixed(20.0)),
                    text("I am not the biggest fan nor user of Large Language Models and I think the whole Artificial Intelligence mania should stop and make space for logic. AI is amazing tool and just a tool, not universal 'get out of jail free' card when it comes to almost everything. Therefore all coding, all assets and all songs I made, make and will make are 100% flesh made.")
                        .center()
                        .width(Length::Fill),
                    ].spacing(APP_SPACING)
            }
            _ => {
                column![
                    text("I have been creating games and programs since 2022. It started with Jorris Quest, a fan game for one Rimworld content creator. After that little big project I started learning new technologies and patterns to make the best programs I can. Here is the list of technologies I am comfortable with:"),
                    rule::horizontal(5.0),
                    text("Create table here lul"),
                    ]
                    .spacing(APP_SPACING)
            }
        };
        c.into()
    }

    pub fn go_next(&mut self) {
        if let AppPage::AboutMe(val) = self {
            *val = *val + 1;
        }
    }

    pub fn go_prev(&mut self) {
        if let AppPage::AboutMe(val) = self && *val > 0 {
            *val = *val - 1;
        }
    }

}

#[derive(Debug, Clone)]
pub enum Input {
    ReturnCurrentPage,
    ChangeCurrentPage(usize),
    ChangeAppSize((usize, usize)),
}

#[derive(Debug)]
pub struct MoveWidget {
    hover: bool,
    left: bool,
}

impl MoveWidget {
    pub fn new(left: bool) -> Self {
        MoveWidget{
            hover: false,
            left,
        }
    }

    pub fn toggle_hover(&mut self, hover: bool) {
        self.hover = hover;
    }

    pub fn hover(&self) -> bool {
        self.hover
    }

    pub fn view(&self) -> Element<'_, MainMessage> {
        if self.hover {
            container(space::vertical()).style(container::primary)
                .width(Length::Fill)
                .into()
        }
        else {
            container(space::vertical()).style(container::transparent)
                .width(Length::Fill)
                .into()
        }
    }
}

#[derive(Debug)]
pub struct MainLayout {
    pub random_u32: u32,
    pub curr_page: AppPage,
    left_trigger: MoveWidget,
    right_trigger: MoveWidget,
    window_size: Size,
}

#[derive(Debug, Clone)]
pub enum MainMessage {
    Nothing,
    AllEvents(iced::Event),
    EventSyncWorkerReady(mpsc::Sender<Input>),
    EventInputHtml(Input),
    WindowSize(Size),
}

fn sync_worker() -> impl Stream<Item = MainMessage> {
    let buffer = 100;
    stream::channel(buffer, async move |mut output| {
        let (sd, mut rv) = mpsc::channel(buffer);
        output.send(MainMessage::EventSyncWorkerReady(sd)).await;
        loop {
            use iced::futures::StreamExt;
            let input = rv.select_next_some().await;
            js::log(&format!("{:?}", input));
            output.send(MainMessage::EventInputHtml(input)).await;
        }
    })
}

impl MainLayout {
    pub fn new() -> Self {
        js::log("WASM READY");
        MainLayout {
            random_u32: 0,
            curr_page: AppPage::default(),
            left_trigger: MoveWidget::new(true),
            right_trigger: MoveWidget::new(false),
            window_size: Size::new(640.0, 480.0),
        }
    }

    pub fn update(&mut self, message: MainMessage) -> iced::Task<MainMessage> {
        let interact_button = iced::mouse::Button::Left;
        match message {
            MainMessage::Nothing => {
                unsafe { js::log("Wrong AppPage id"); }
            }

            MainMessage::AllEvents(e) => {
                if let iced::Event::Mouse(mouse) = e {
                    if let iced::mouse::Event::CursorMoved {position: pos} = mouse {
                        let wind_offset = self.window_size.width * APP_WINDOW_TRIGGER_PERCENT;
                        self.left_trigger.toggle_hover(pos.x < wind_offset);
                        self.right_trigger.toggle_hover(pos.x > self.window_size.width - wind_offset);
                    }

                    if let iced::mouse::Event::ButtonPressed(bttn) = mouse && bttn == interact_button {
                        if self.left_trigger.hover() {
                            js::log("Left click");
                            self.curr_page.go_prev();
                        }
                        else if self.right_trigger.hover() {
                            js::log("Right click");
                            self.curr_page.go_next();
                        }
                    }
                }
            }
            
            MainMessage::EventSyncWorkerReady(sync) => {
                if let Err(err) = HTML_SENDER.set(sync) {
                    js::log("Sync error")
                }
                else {
                    js::log("Sync ready");
                }
                return iced::window::latest().and_then(iced::window::size).map(MainMessage::WindowSize)
            } 

            MainMessage::EventInputHtml(i) => {
                match i {
                    Input::ReturnCurrentPage => {
                        js::log("Return page");
                    }
                    Input::ChangeCurrentPage(id) => {
                        js::log(&format!("Numbah: {}", id));
                    }
                    Input::ChangeAppSize(size) => {
                        let width_f32 = size.0 as f32;
                        let height_f32 = size.1 as f32;
                        let size_f32 = Size::new(width_f32, height_f32);
                        self.window_size = size_f32;
                    }
                }
            }

            MainMessage::WindowSize(size) => { // Since this method returns... twice as small window size, we need to do something fancy
                js::log(&format!("{:?}", size)); 
                self.window_size = size;
            }
        }

        iced::Task::none()
    }

    pub fn view(&self) -> Element<'_, MainMessage> {
        let main = center(self.curr_page.view())
            .padding(APP_PADDING);
        let overlay = center(row![
                self.left_trigger.view(),
                space::horizontal(),
                self.right_trigger.view(),
            ]);
        stack![
                main,
                overlay,
            ]
            .into()
    }

    fn move_overlay(&self) -> Element<'_, MainMessage> {
        row![
            mouse_area(space::vertical()),
        ]
        .into()
    }
   
    pub fn subscription(&self) -> Subscription<MainMessage> {
        let runner = Subscription::run(sync_worker);
        Subscription::batch([iced::event::listen().map(MainMessage::AllEvents), runner])
    }
}
