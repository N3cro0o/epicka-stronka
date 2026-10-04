use iced::{stream, Animation, Rectangle, Element, Length, Subscription, Pixels, Size, Task};
use iced::widget::{selector, stack, table, scrollable, space, text, container, center, row, column, mouse_area, rule};
use iced::widget::container::Container;
use iced::widget::image::Handle;
use iced::futures::channel::mpsc;
use iced::futures::sink::SinkExt;
use iced::futures::Stream;

use std::convert::From;
use std::sync::OnceLock;

#[cfg_attr(target_family = "wasm", path = "js.rs")]
#[cfg_attr(not(target_family = "wasm"), path = "no_js.rs")]
pub mod js;

pub const APP_SPACING: Pixels = Pixels(10.0);
pub const APP_PADDING: Pixels = Pixels(10.0);
const APP_FONT_SIZE: f32 = 24.0;
const APP_WINDOW_TRIGGER_PERCENT: f32 = 0.3;
const ME_SCROLL_ID: iced::widget::Id = iced::widget::Id::new("me_scroll");

pub static HTML_SENDER: OnceLock<mpsc::Sender<Input>> = OnceLock::new(); 

pub struct ProjectWidget {
    image: Handle,
    desc: String,
}

impl Default for ProjectWidget {
    fn default() -> Self {
        let bytes = include_bytes!("../../../img/star.png").to_vec();
        ProjectWidget{
            image: Handle::from_bytes(bytes),
            desc: "lorem ipsum dolor sit amet.".to_string(),
        }
    }
}

impl ProjectWidget {
    const SIZE_MULT: f32 = 10.0;

    pub fn view(&self, state: &MainLayout) -> Element<'static, MainMessage> {
        let height: f32 = f32::from(APP_SPACING) * 2.0 + APP_FONT_SIZE * Self::SIZE_MULT / state.pixel_ratio as f32;
        container(
            row![
                center(iced::widget::image(self.image.clone()))
                    .width(Length::FillPortion(2))
                    .height(Length::Fixed(APP_FONT_SIZE * Self::SIZE_MULT / state.pixel_ratio as f32)),
                rule::vertical(2.0),
                container(text(self.desc.clone()).size(APP_FONT_SIZE / state.pixel_ratio as f32))
                    .width(Length::FillPortion(7))
                    .center_y(Length::Fill),
            ]
            .align_y(iced::Center)
            .spacing(APP_SPACING)
            .height(Length::Fill)
        )
        .center_y(Length::Fixed(height))
        .padding(APP_PADDING)
        .width(Length::Fill)
        // .style(container::secondary)
        .into()
    }

    pub fn project_vec() -> Vec<Self> {
        let mut vec = vec![];
        let handle_vec = Self::image_handle_vec();
        let desc_vec = Self::desc_vec();
        for i in 0..handle_vec.len() {
            vec.push(Self{image: handle_vec[i].clone(), desc: desc_vec[i].clone()});
        }
        vec
    }

    fn image_handle_vec() -> Vec<Handle> {
        let mut vec = vec![];
        vec.push(Handle::from_bytes(include_bytes!("../../../img/projects/osrs_ge.png").to_vec()));
        vec.push(Handle::from_bytes(include_bytes!("../../../img/projects/pgne.png").to_vec()));
        vec.push(Handle::from_bytes(include_bytes!("../../../img/projects/tauri.png").to_vec()));
        vec.push(Handle::from_bytes(include_bytes!("../../../img/projects/bachelor.png").to_vec()));
        vec.push(Handle::from_bytes(include_bytes!("../../../img/projects/pincher.jpg").to_vec()));
        vec
    }

    fn desc_vec() -> Vec<String> {
        let mut vec = vec![];
        vec.push(String::from("Old School RuneScape Grand Exchange helper tool created using Rust Iced 0.14. I have made this mainly to learn Iced framework and to make a really handy tool for myself (since OSRS is one of my favourites, it was a perferct idea).\nThis app supports checking current and historic prices, compare alchemy prices and create custom recipes. To add a cherry on top, custom notification and logging systems has been prepared. There are even sounds :p."));
        vec.push(String::from("Virtual Pet game made using Godot Engine 4.0. Your main goal is to help your virtual student survive the exam session. Game made for gamedev course during Bachelor course in the span of 4 weeks. This project is one big inside joke and thus a lot of unserious elements were introduced. This game introduced localisation system with English, Polish and Silesian languages."));
        vec.push(String::from("Internal application project made using Tauri 2.0. The main goal behind this project was creation of new java launcher with update and testing functionality. This software was made during internship for European debt collection agency. This project has shown ins and outs of Java Virtual Machine, multi-environment development, automating development and why everybody hates JVM."));
        vec.push(String::from("IoT project used to calculate distance to target using two towers and triangulation. Prototype was made during Bachelor studies (and thanks to this beauty I graduated). It was made using one central computer and powet source, Raspberry Pi 5, two servos and two camera modules. User can operate thru local network and web control panel. All necessary logic was written by me using rust tide + rppal, React.js and nginx."));
        vec.push(String::from("Rimworld mod introducing new breed of dogs, Pinchers. This mod was made in one month without much help but one very handy and old guide. The main goal was to learn the ins and outs of Rimworld and to learn how to modify already existing applications. New breed after being tamed allows to detect nerby enemies and bark if they are close enough. The main inspiration behind the sprites were my dogs (and I believe they came out cute)."));
        vec
    }
}

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
    pub fn view(&self, state: &MainLayout) -> Element<'_, MainMessage> {
        match self {
            AppPage::AboutMe(val) => { Self::me_view((*val).into(), state) }
            AppPage::Projects => { Self::proj_view(state) }
            AppPage::AboutPage => { Self::page_view(state) }
        }   
    }

    fn me_view(val: usize, state: &MainLayout) -> Element<'static, MainMessage> {
        use std::iter;
        let w = state.outer_size.unwrap().width.clone() - f32::from(APP_PADDING) * 2.0;
        let h = state.outer_size.unwrap().height.clone() - f32::from(APP_PADDING) * 2.0;
        let mut v = vec![];
        v.push(
            column![
                space::vertical().height(Length::Fixed(50.0)),
                text("Hello!")
                    .center()
                    .size(APP_FONT_SIZE * 2.0 / state.pixel_ratio as f32)
                    .width(Length::Fill),
                space::vertical(),
                text("On the Internet you can find me as N3cro0o. I'm aspiring student, software and game developer. Everyday I'm trying to make better and better projects and polish the old ones to perfection. And most importantly I tried to spread this mindset to others.")
                    .center()
                    .size(APP_FONT_SIZE / state.pixel_ratio as f32)
                    .width(Length::Fill),
                text("I think of myself as an artist, I like to draw and create new handmande assets for each of my projects. By being creative and not fearing chasing knowledge I don't fear expressing myself. You can check this for yourself by looking at my past projects :p")
                    .center()
                    .size(APP_FONT_SIZE / state.pixel_ratio as f32)
                    .width(Length::Fill),
                space::vertical().height(Length::Fixed(20.0)),
                text("I am not the biggest fan nor user of Large Language Models and I think the whole Artificial Intelligence mania should stop and make space for logic. AI is amazing tool and just a tool, not universal 'get out of jail free' card when it comes to almost everything. Therefore all coding, all assets and all songs I made, make and will make are 100% flesh made.")
                    .center()
                    .size(APP_FONT_SIZE / state.pixel_ratio as f32)
                    .width(Length::Fill),
                iced::widget::image(state.hello_gif_image.clone()).height(Length::Fixed(64.0)),
                space::vertical(),
                ].spacing(APP_SPACING)
                .padding(APP_PADDING)
                .width(Length::Fixed(w))
                .height(Length::Fixed(h))
                .into()
            );
        v.push({
            let table1 = {
                let columns = [
                    table::column("Technology",
                        |data: (&str, u8)| text(data.0)
                        .center()
                        .size(APP_FONT_SIZE / state.pixel_ratio as f32)
                    ),
                    table::column("Knowledge",
                        |data: (&str, u8)| {
                            let mut img_vec = vec![];
                            for i in 0..data.1 {
                                let handle = state.star_image.clone();
                                img_vec.push(iced::widget::image(handle).width(Length::Fixed(APP_FONT_SIZE / state.pixel_ratio as f32)).into());
                            }
                            iced::widget::Row::from_vec(img_vec)
                        }   
                    ),
                ];
                let data = [
                    ("Rust", 5),
                    ("C/C++", 5),
                    ("GDScript", 5),
                    ("C# and .NET", 4),
                    ("Minecraft language", 3),
                    ("Python", 2),
                    ("JavaScript", 1),
                    ("TypeScript", 1),
                ];
                table(columns, data)
            };
            let table2 = {
                let columns = [
                    table::column("Technology",
                        |data: (&str, u8)| text(data.0)
                        .center()
                        .size(APP_FONT_SIZE / state.pixel_ratio as f32)
                    ),
                    table::column("Knowledge",
                        |data: (&str, u8)| {
                            let mut img_vec = vec![];
                            for i in 0..data.1 {
                                let handle = state.star_image.clone();
                                img_vec.push(iced::widget::image(handle).width(Length::Fixed(APP_FONT_SIZE / state.pixel_ratio as f32)).into());
                            }
                            iced::widget::Row::from_vec(img_vec)
                        }   
                    ),
                ];
                let data = [
                    ("Godot", 5),
                    ("PostgreSQL", 4),
                    ("Rust Iced", 4),
                    ("OpenGL", 3),
                    ("Unity", 3),
                    ("Bash", 3),
                    ("React.js", 2),
                    ("Curses", 1),
                ];
                table(columns, data)
            };
            let table3 = {
                let columns = [
                    table::column("Technology",
                        |data: (&str, u8)| text(data.0)
                        .center()
                        .size(APP_FONT_SIZE / state.pixel_ratio as f32)
                    ),
                    table::column("Knowledge",
                        |data: (&str, u8)| {
                            let mut img_vec = vec![];
                            for i in 0..data.1 {
                                let handle = state.star_image.clone();
                                img_vec.push(iced::widget::image(handle).width(Length::Fixed(APP_FONT_SIZE / state.pixel_ratio as f32)).into());
                            }
                            iced::widget::Row::from_vec(img_vec)
                        }   
                    ),
                ];
                let data = [
                    ("MS Paint", 5),
                    ("Aseprite", 4),
                    ("GIMP", 4),
                    ("Git", 4),
                    ("Audacity", 3),
                    ("FL Studio", 2),
                    ("Krita", 1),
                    ("Blender", 1),
                ];
                table(columns, data)
            };
            column![
                space::vertical().height(Length::Fixed(50.0)),
                text("My Skillz")
                    .center()
                    .size(APP_FONT_SIZE * 2.0 / state.pixel_ratio as f32)
                    .width(Length::Fill),
                space::vertical(),
                text("I have been creating games and programs since 2022. It started with Jorris Quest, a fan game for one Rimworld content creator. After that little big project I started learning new technologies and patterns to make the best programs I can. Here is the list of technologies I am comfortable with:")
                    .center()
                    .size(APP_FONT_SIZE / state.pixel_ratio as f32)
                    .width(Length::Fill),
                rule::horizontal(2.0),
                row![
                        space::horizontal(),
                        table1,
                        space::horizontal(),
                        table2, 
                        space::horizontal(),
                        table3,
                        space::horizontal(),
                    ]
                    .spacing(APP_SPACING),
                space::vertical(),
                ]
                .spacing(APP_SPACING)
                .padding(APP_PADDING)
                .width(Length::Fixed(w))
                .height(Length::Fixed(h))
                .into()
        });
        scrollable(iced::widget::Row::from_vec(v))
            .direction(scrollable::Direction::Horizontal(scrollable::Scrollbar::hidden()))
            .id(ME_SCROLL_ID)
            .into()
    }

    fn proj_view(state: &MainLayout) -> Element<'static, MainMessage> {
        let handle_vec = ProjectWidget::project_vec();
        let mut element_vec = vec![];
        for project in handle_vec.iter() {
            element_vec.push(project.view(state));
        }
        let scroll = scrollable(
            iced::widget::Column::from_vec(element_vec)
                .spacing(APP_SPACING)
                .width(Length::Fill)
        );
        column![
                space::vertical().height(Length::Fixed(50.0)),
                text("On this page I have descripted *almost* all projects of mine.")
                    .size(APP_FONT_SIZE * 2.0 / state.pixel_ratio as f32)
                    .width(Length::Fill)
                    .center(),
                text("Due to some reasons like losing source files, being secret or being forgotten. Happens, you know.")
                    .size(APP_FONT_SIZE * 1.2 / state.pixel_ratio as f32)
                    .width(Length::Fill)
                    .center(),
                rule::horizontal(2.0),
                scroll,
            ].spacing(APP_SPACING)
            .padding(APP_PADDING)
            .width(Length::Fill)
            .into()
    }

    fn page_view(state: &MainLayout) -> Element<'static, MainMessage> {
        let c = column![            
                space::vertical().height(Length::Fixed(50.0)),
                text("How this page was made")
                    .center()
                    .size(APP_FONT_SIZE * 2.0 / state.pixel_ratio as f32)
                    .width(Length::Fill),
                space::vertical(),
                space::vertical().height(Length::Fixed(20.0)),
                text("This page was made using mostly WASM with some basic HTML, CSS and JS wrapper. WASM elements are rendered using canvas elements. The main WASM element (this one :p) was made using rust Iced 0.14 and wasm-bindgen crates. The background was made using [insert desc here].")
                    .center()
                    .size(APP_FONT_SIZE / state.pixel_ratio as f32)
                    .width(Length::Fill),
                row![
                        space::horizontal(),
                        iced::widget::image(state.html_image.clone()).height(Length::Fixed(128.0)),
                        iced::widget::image(state.wasm_image.clone()).height(Length::Fixed(128.0)),
                        space::horizontal(),
                    ]
                    .spacing(APP_SPACING),
                text("The main goal behind this stack was to learn WASM and how to properly handle it. I am not the biggest fan of webdev and I strongly believe most of popular browser sites would be better if they were desktop apps. But these are just my delusions lol. Nowadays all developers need to know at least basics of creating web applications so to recall how things work I made this page. But I decided to only use it as a foundation, a skeleton while WASM elements are the main elements. I used HTML this way as a failsafe when WASM fails to load. Everything is better than blank gray page XD.")
                    .center()
                    .size(APP_FONT_SIZE / state.pixel_ratio as f32)
                    .width(Length::Fill),
                space::vertical(),
            ].spacing(APP_SPACING)
            .padding(APP_PADDING);
        c.into()
    }

    pub fn go_next(&mut self) {
        if let AppPage::AboutMe(val) = self && *val < 1 {
            *val = *val + 1;
        }
    }

    pub fn go_prev(&mut self) {
        if let AppPage::AboutMe(val) = self && *val > 0 {
            *val = *val - 1;
        }
    }

    pub fn get_curr(&self) -> u8 {
        match self {
            AppPage::AboutMe(val) => *val,
            _ => 0,
        }   
    }
    
    pub fn me_pages(&self) -> u8 {
        2
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
            container(space::vertical()).style(Self::style)
                .width(Length::Fill)
                .into()
        }
        else {
            container(space::vertical()).style(container::transparent)
                .width(Length::Fill)
                .into()
        }
    }

    fn style(theme: &iced::Theme) -> container::Style {
        let pal = theme.extended_palette();
        let mut c = pal.primary.base.color.clone();
        let col = c.scale_alpha(0.2);
        container::Style {
            background: Some(col.into()),
            text_color: Some(pal.primary.base.text),
            ..container::Style::default()
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
    pixel_ratio: f64,
    star_image: Handle,
    html_image: Handle,
    wasm_image: Handle,
    hello_gif_image: Handle,
    outer_size: Option<Rectangle>,
    me_animation: Animation<f32>,
}

#[cfg(target_family = "wasm")]
#[derive(Debug, Clone)]
pub enum MainMessage {
    Nothing,
    AllEvents(iced::Event),
    EventSyncWorkerReady(mpsc::Sender<Input>),
    EventInputHtml(Input),
    WindowSize(Size),
    GetContainerSize(Option<selector::Target>),
    SecTick(wasmtimer::std::Instant),
    Tick(iced::time::Instant),
}

#[cfg(not(target_family = "wasm"))]
#[derive(Debug, Clone)]
pub enum MainMessage {
    Nothing,
    AllEvents(iced::Event),
    EventSyncWorkerReady(mpsc::Sender<Input>),
    EventInputHtml(Input),
    WindowSize(Size),
    GetContainerSize(Option<selector::Target>),
    SecTick(iced::time::Instant),
    Tick(iced::time::Instant),
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
    pub fn new() -> (Self, Task<MainMessage>) {
        js::log("WASM READY");
        let pixel =  js::get_pixel_ratio().unwrap_or(1.0);
        let bytes = include_bytes!("../../../img/star.png").to_vec();
        let bytes_hullo = include_bytes!("../../../img/hello.gif").to_vec();
        let bytes_html = include_bytes!("../../../img/site/html.png").to_vec();
        let bytes_wasm = include_bytes!("../../../img/site/wasm.png").to_vec();
        let m = MainLayout {
            random_u32: 0,
            curr_page: AppPage::default(),
            left_trigger: MoveWidget::new(true),
            right_trigger: MoveWidget::new(false),
            window_size: Size::new(640.0, 480.0),
            pixel_ratio: pixel,
            star_image: Handle::from_bytes(bytes),
            html_image: Handle::from_bytes(bytes_html),
            wasm_image: Handle::from_bytes(bytes_wasm),
            hello_gif_image: Handle::from_bytes(bytes_hullo),
            outer_size: None,
            me_animation: Animation::new(0.0).duration(std::time::Duration::new(0, 250)),
        };
        (m, selector::find(iced::widget::Id::new("outer_container")).map(MainMessage::GetContainerSize))
    }

    pub fn update(&mut self, message: MainMessage) -> iced::Task<MainMessage> {
        let interact_button = iced::mouse::Button::Left;
        match message {
            MainMessage::Nothing => {
                unsafe { js::log("Wrong AppPage id"); }
            }

            MainMessage::AllEvents(e) => {
                if let iced::Event::Mouse(mouse) = e {
                    if let iced::mouse::Event::CursorMoved {position: pos} = mouse && self.curr_page != AppPage::Projects{
                        let wind_offset = self.window_size.width * APP_WINDOW_TRIGGER_PERCENT;
                        self.left_trigger.toggle_hover(pos.x < wind_offset);
                        self.right_trigger.toggle_hover(pos.x > self.window_size.width - wind_offset);
                    }

                    if let iced::mouse::Event::ButtonPressed(bttn) = mouse && bttn == interact_button {
                        if self.left_trigger.hover() {
                            js::log("Left click");
                            let point_from: f32 = self.outer_size.unwrap_or_default().width 
                                * (self.curr_page.get_curr() as f32 / (self.curr_page.me_pages() - 1) as f32);
                            self.curr_page.go_prev();
                            let point_to: f32 = self.outer_size.unwrap_or_default().width 
                                * (self.curr_page.get_curr() as f32 / (self.curr_page.me_pages() - 1) as f32);
                            let instant = iced::time::Instant::now();
                            self.me_animation = Animation::new(point_from)
                                .duration(std::time::Duration::new(1, 500))
                                .go(point_to, instant);
                            js::log(&format!("from: {}, to: {}", point_from, point_to));
                        }
                        else if self.right_trigger.hover() {
                            js::log("Right click");
                            let point_from: f32 = self.outer_size.unwrap_or_default().width 
                                * (self.curr_page.get_curr() as f32 / (self.curr_page.me_pages() - 1) as f32);
                            self.curr_page.go_next();
                            let point_to: f32 = self.outer_size.unwrap_or_default().width 
                                * (self.curr_page.get_curr() as f32 / (self.curr_page.me_pages() - 1) as f32);
                            let instant = iced::time::Instant::now();
                            self.me_animation = Animation::new(point_from)
                                .duration(std::time::Duration::new(1, 500))
                                .go(point_to, instant);
                            js::log(&format!("from: {}, to: {}", point_from, point_to));
                        }
                    }

                    if let iced::mouse::Event::CursorLeft = mouse {
                        self.left_trigger.toggle_hover(false);
                        self.right_trigger.toggle_hover(false);
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
                        let page: AppPage = (id as i32).into();
                        self.curr_page = page;
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

            MainMessage::GetContainerSize(option) => {
                js::log(&format!("{:#?}", option));
                if let Some(rect) = option {
                    self.outer_size = rect.visible_bounds();
                    if self.outer_size.is_some() {
                        return Task::none();
                    }
                }
            }

            MainMessage::SecTick(_) => {
                if self.outer_size.is_none() {
                    return selector::find(iced::widget::Id::new("outer_container")).map(MainMessage::GetContainerSize);
                }
            }

            MainMessage::Tick(instant) => {
                if self.me_animation.is_animating(instant) {
                    let v = self.me_animation.interpolate_with(|v| v, instant);
                    js::log(&format!("{:?}: {}", instant, v));
                    return iced::widget::operation::scroll_to(ME_SCROLL_ID,
                        iced::widget::operation::AbsoluteOffset {
                            x: Some(self.me_animation.interpolate_with(|v| v, instant)),
                            y: None,
                        });
                }
            }
        }

        iced::Task::none()
    }

    pub fn view(&self) -> Element<'_, MainMessage> {
        let main; 
        if self.outer_size.is_none() {
            main = center(text("Please wait..."));
        }
        else {
            main = center(self.curr_page.view(self))
                .id(iced::widget::Id::new("outer_container"))
                .padding(APP_PADDING);
        }
        let overlay = center(row![
            self.left_trigger.view(),
            space::horizontal().width(Length::FillPortion(2)),
            self.right_trigger.view(),
        ]).id(iced::widget::Id::new("outer_container"));
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
  
    #[cfg(target_family = "wasm")]
    pub fn subscription(&self) -> Subscription<MainMessage> {
        let runner = Subscription::run(sync_worker);
        let tick_sec = iced::time::every(iced::time::Duration::from_secs(1)).map(MainMessage::SecTick);
        let tick = iced::window::frames().map(MainMessage::Tick);
        Subscription::batch([iced::event::listen().map(MainMessage::AllEvents), runner, tick_sec, tick])
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn subscription(&self) -> Subscription<MainMessage> {
        let runner = Subscription::run(sync_worker);
        let tick = iced::window::frames().map(MainMessage::Tick);
        Subscription::batch([iced::event::listen().map(MainMessage::AllEvents), runner, tick])
    }
}
