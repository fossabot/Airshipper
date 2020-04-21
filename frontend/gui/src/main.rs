use client::{filesystem, profile::Profiles};
use iced::{
    button, scrollable, window::Settings as Window, Align, Application, Button, Column, Command, Container, Element,
    HorizontalAlignment, Image, Length, ProgressBar, Row, Settings, Space, Subscription, Text, VerticalAlignment,
};
use storage::{Changelog, News};

mod config;
mod error;
mod network;
mod storage;
mod style;
mod subscriptions;
//mod updater;

pub(crate) type Result<T> = std::result::Result<T, error::ClientError>;

pub fn main() {
    // TODO: If cli enabled parse arguments and pass it into the settings: https://github.com/hecrj/iced/pull/246
    Airshipper::run(settings())
}

#[derive(Debug)]
pub(crate) enum Message {
    Interaction(Interaction),
    Loaded(Result<Airshipper>),
    ChangelogUpdate(Result<Option<Changelog>>),
    NewsUpdate(Result<Option<News>>),
}

/// Used to describe User Actions like pressing a specific Button.
#[derive(Debug, Clone)]
pub(crate) enum Interaction {
    PlayPressed,
    ReadMore(String),
    // Interaction won't do anything
    Disabled,
}

#[derive(Debug)]
pub(crate) enum LauncherState {
    LoadingSave,
    QueryingForUpdates,
    UpdateAvailable,
    ReadyToPlay,
    Downloading,
    Installing,
    Playing,

    Error(crate::error::ClientError),
}

impl Default for LauncherState {
    fn default() -> Self {
        LauncherState::LoadingSave
    }
}

#[derive(Debug, Default)]
struct Airshipper {
    /// Current state the GUI is in (e.g. Loading up the save file, updating veloren, ...)
    pub state: LauncherState,
    /// Any part of the GUI which will need to be saved
    pub changelog: Changelog,
    pub news: News,
    profiles: Profiles,
    /// Other unrelated state
    pub changelog_scrollable_state: scrollable::State,
    pub news_scrollable_state: scrollable::State,
    pub play_button_state: button::State,
}

impl Application for Airshipper {
    type Executor = iced_futures::executor::Tokio;
    type Message = Message;

    fn new() -> (Airshipper, Command<Message>) {
        (
            Airshipper::default(),
            Command::perform(Airshipper::load(), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        format!("Airshipper v{}", env!("CARGO_PKG_VERSION"))
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Loaded(state) => {
                *self = state.unwrap();
                self.state = LauncherState::QueryingForUpdates;
                Command::batch(vec![
                    Command::perform(
                        Changelog::update(self.changelog.version.clone()),
                        Message::ChangelogUpdate,
                    ),
                    Command::perform(News::update(self.news.version.clone()), Message::NewsUpdate),
                    /* TODO: Command::perform(Profiles::update(self.profiles.latest().version.clone()),
                     * Message::ProfileUpdate), */
                ])
            },
            Message::ChangelogUpdate(update) => {
                self.changelog = update.unwrap().unwrap();
                Command::none()
            },
            Message::NewsUpdate(update) => {
                self.news = update.unwrap().unwrap();
                Command::none()
            },
            _ => Command::none(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    fn view(&mut self) -> Element<Message> {
        let title =
            Container::new(Image::new(filesystem::get_assets_path("veloren-logo.png"))).width(Length::FillPortion(10));

        let icons = Row::new()
            .width(Length::Fill)
            .height(Length::Units(90))
            .align_items(Align::Center)
            .spacing(10)
            .padding(15)
            .push(title)
            .push(Space::with_width(Length::FillPortion(5)));

        let changelog = self.changelog.view();

        // Contains title, changelog
        let left = Column::new()
            .width(Length::FillPortion(3))
            .height(Length::Fill)
            .padding(15)
            .push(icons)
            .push(changelog);

        let news = self.news.view();

        // Contains logo, changelog and news
        let middle = Row::new().padding(2).push(left).push(news);
        let middle_container = Container::new(middle)
            .height(Length::FillPortion(6))
            .style(style::Middle);

        let download_text = match &self.state {
            LauncherState::Downloading => format!("Downloading... {}/sec", 100),
            LauncherState::Installing => "Installing...".into(),
            LauncherState::LoadingSave => "Loading...".into(),
            LauncherState::QueryingForUpdates => "Checking for updates...".into(),
            LauncherState::ReadyToPlay => "Ready to play...".into(),
            LauncherState::UpdateAvailable => "Update available!".into(),
            LauncherState::Playing => "Much fun playing!".into(),
            LauncherState::Error(e) => format!("{}", e),
        };
        let download_progress = match &self.state {
            LauncherState::Downloading => 50.0,
            _ => 0.0,
        };

        let download_speed = Text::new(&download_text).size(16);
        let download_progressbar = ProgressBar::new(0.0..=100.0, download_progress).style(style::Progress);
        let download = Column::new()
            .width(Length::FillPortion(4))
            .spacing(5)
            .push(download_speed)
            .push(download_progressbar);

        let play_button_text = match &self.state {
            LauncherState::Downloading => format!("Downloading"),
            LauncherState::Installing => "Installing".into(),
            LauncherState::LoadingSave => "Loading".into(),
            LauncherState::QueryingForUpdates => "Loading".into(),
            LauncherState::ReadyToPlay => "Play".into(),
            LauncherState::UpdateAvailable => "Update".into(),
            LauncherState::Playing => "Playing".into(),
            LauncherState::Error(_) => "ERROR".into(),
        };
        let mut play = Button::new(
            &mut self.play_button_state,
            Text::new(&play_button_text)
                .size(30)
                .height(Length::Fill)
                .horizontal_alignment(HorizontalAlignment::Center)
                .vertical_alignment(VerticalAlignment::Center),
        )
        .on_press(Interaction::PlayPressed)
        .width(Length::FillPortion(1))
        .height(Length::Units(60))
        .style(style::PlayButton)
        .padding(2);

        // Disable button if loading, playing or downloading the game.
        match self.state {
            LauncherState::UpdateAvailable | LauncherState::ReadyToPlay => {},
            _ => {
                play = play.style(style::PlayButtonDisabled);
                play = play.on_press(Interaction::Disabled);
            },
        }
        let play: Element<Interaction> = play.into();

        let bottom = Row::new()
            .align_items(Align::End)
            .spacing(20)
            .padding(10)
            .push(download)
            .push(play.map(Message::Interaction));
        let bottom_container = Container::new(bottom).style(style::Bottom);

        // Contains everything
        let content = Column::new()
            .padding(2)
            .width(Length::Fill)
            .height(Length::Fill)
            .push(middle_container)
            .push(bottom_container);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style::Content)
            .into()
    }
}

impl Airshipper {
    pub(crate) async fn load() -> Result<Self> {
        // TODO
        Ok(Airshipper::default())
    }

    //pub(crate) async fn save(&self) -> Result<()> {
    //    Ok(())
    //}
}

fn settings() -> Settings {
    Settings {
        window: Window {
            size: (1050, 620),
            resizable: true,
            decorations: true,
        },
        default_font: Some(include_bytes!("../assets/haxrcorp_4089_cyrillic_altgr_extended.ttf")),
        // Enforce the usage of dedicated gpu if available
        antialiasing: true,
    }
}
