use iced::{stream, Element, Subscription};
use iced::widget::{button, center};
use iced::futures::channel::mpsc;
use iced::futures::sink::SinkExt;
use iced::futures::Stream;

use std::convert::From;
use std::sync::OnceLock;

pub mod js;

pub static HTML_SENDER: OnceLock<mpsc::Sender<Input>> = OnceLock::new();

#[derive(Debug, Clone, PartialEq, Default)]
pub enum AppPage {
    #[default]
    AboutMe,
    AboutPage,
    Projects,
}

impl From<i32> for AppPage {
    fn from(value: i32) -> Self {
        match value {
            0 => AppPage::AboutMe,
            1 => AppPage::Projects, 
            2 => AppPage::AboutPage,
            _ => {
                unsafe { js::log("Wrong AppPage id"); }
                AppPage::AboutMe
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Input {
    ReturnCurrentPage,
    ChangeCurrentPage(usize),
}

#[derive(Debug)]
pub struct MainLayout {
    pub random_u32: u32,
    pub curr_page: AppPage,
}

#[derive(Debug, Clone)]
pub enum MainMessage {
    Nothing,
    AllEvents(iced::Event),
    EventSyncWorkerReady(mpsc::Sender<Input>),
    EventInputHtml(Input),
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
        }
    }

    pub fn update(&mut self, message: MainMessage) {
        js::log(&format!("{:?}", message));
        match message {
            MainMessage::Nothing => {
                unsafe { js::log("Wrong AppPage id"); }
            }

            MainMessage::AllEvents(_e) => {
            
            }
            
            MainMessage::EventSyncWorkerReady(sync) => {
                if let Err(err) = HTML_SENDER.set(sync) {
                    js::log("Sync error")
                }
                else {
                    js::log("Sync ready");
                }
            } 

            MainMessage::EventInputHtml(i) => {
                match i {
                    Input::ReturnCurrentPage => {
                        js::log("Return page");
                    }
                    Input::ChangeCurrentPage(id) => {
                        js::log(&format!("Numbah: {}", id));
                    }
                }
            }
        }
    }

    pub fn view(&self) -> Element<'_, MainMessage> {
        center(button("Hello world!").on_press(MainMessage::Nothing)).into()
    }

    pub fn subscription(&self) -> Subscription<MainMessage> {
        let runner = Subscription::run(sync_worker);
        Subscription::batch([iced::event::listen().map(MainMessage::AllEvents), runner])
    }
}
