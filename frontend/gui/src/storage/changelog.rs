use crate::{config, network, Message, Result};
use iced::{scrollable, Element};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Changelog {
    pub text: String,
    pub version: String,

    #[serde(skip)]
    changelog_scrollable_state: scrollable::State,
}

impl Changelog {
    /// Tries to fetch the Changelog
    pub(crate) async fn fetch() -> Result<Self> {
        let changelog = network::query(config::CHANGELOG_URL).await?;

        Ok(Changelog {
            version: network::get_etag(&changelog),
            text: changelog
                .text()
                .await?
                .lines()
                .skip_while(|x| !x.contains(&"## [Unreleased]"))
                .skip(2)
                .take_while(|x| !x.contains(&"## [0.1.0]"))
                .map(|x| format!("{}\n", x))
                .collect(),
            ..Default::default()
        })
    }

    /// Returns new Changelog incase remote one is newer
    pub(crate) async fn update(version: String) -> Result<Option<Self>> {
        let remote_version = network::query_etag(config::CHANGELOG_URL).await?;
        if version != remote_version {
            return Ok(Some(Self::fetch().await?));
        }
        Ok(None)
    }

    pub(crate) fn view(&mut self) -> Element<Message> {
        use iced::{Length, Scrollable, Text};

        Scrollable::new(&mut self.changelog_scrollable_state)
            .height(Length::Fill)
            .padding(15)
            .spacing(20)
            .push(Text::new(self.text.clone()).size(18))
            .into()
    }
}
