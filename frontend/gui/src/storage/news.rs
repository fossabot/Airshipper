use crate::{config, network, Message, Result};
use iced::{scrollable, Element};
use rss::Channel;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub(crate) struct News {
    posts: Vec<Post>,
    pub version: String,

    #[serde(skip)]
    news_scrollable_state: scrollable::State,
}

impl News {
    /// Tries to fetch the News
    pub(crate) async fn fetch() -> Result<Self> {
        use std::io::BufReader;

        let news = network::query(config::NEWS_URL).await?;
        let version = network::get_etag(&news);
        let feed = Channel::read_from(BufReader::new(&news.bytes().await?[..]))?;

        Ok(News {
            posts: feed.items().iter().take(15).map(Post::from).collect(),
            version,
            ..Default::default()
        })
    }

    /// Returns new News incase remote one is newer
    pub(crate) async fn update(version: String) -> Result<Option<Self>> {
        let remote_version = network::query_etag(config::NEWS_URL).await?;
        if version != remote_version {
            return Ok(Some(Self::fetch().await?));
        }
        Ok(None)
    }

    pub(crate) fn view(&mut self) -> Element<Message> {
        use crate::style;
        use iced::{Container, Length, Scrollable};

        let mut news = Scrollable::new(&mut self.news_scrollable_state).spacing(20).padding(25);

        for post in &mut self.posts {
            news = news.push(post.view());
        }

        Container::new(news)
            .width(Length::FillPortion(2))
            .height(Length::Fill)
            .style(style::News)
            .into()
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Post {
    pub title: String,
    pub description: String,
    pub button_url: String,

    #[serde(skip)]
    pub btn_state: iced::button::State,
}

impl Post {
    pub(crate) fn view(&mut self) -> Element<Message> {
        use crate::{style, Interaction};
        use iced::{Button, Column, HorizontalAlignment, Length, Text, VerticalAlignment};

        let read_more_btn: Element<Interaction> = Button::new(
            &mut self.btn_state,
            Text::new("Read More")
                .size(14)
                .horizontal_alignment(HorizontalAlignment::Center)
                .vertical_alignment(VerticalAlignment::Center),
        )
        .on_press(Interaction::ReadMore(self.button_url.clone()))
        .width(Length::Units(80))
        .height(Length::Units(25))
        .padding(2)
        .style(style::ReadMoreButton)
        .into();

        Column::new()
            .push(Text::new(&self.title).size(20))
            .push(Text::new(&self.description).size(16))
            .push(read_more_btn.map(Message::Interaction))
            .spacing(8)
            .into()
    }

    fn process_description(desc: Option<&str>) -> String {
        match desc {
            Some(desc) => {
                let stripped_html = html2text::from_read(desc.as_bytes(), 400)
                    .lines()
                    .take(3)
                    .filter(|x| !x.contains("[banner]"))
                    .map(|x| format!("{}\n", x))
                    .collect::<String>();
                let stripped_markdown = strip_markdown::strip_markdown(&stripped_html);
                stripped_markdown
            },
            None => "No description found.".into(),
        }
    }
}

impl From<&rss::Item> for Post {
    fn from(post: &rss::Item) -> Self {
        Post {
            title: post.title().unwrap_or("Missing title").into(),
            description: Self::process_description(post.description()),
            button_url: post.link().unwrap_or("https://www.veloren.net").into(),

            btn_state: Default::default(),
        }
    }
}
