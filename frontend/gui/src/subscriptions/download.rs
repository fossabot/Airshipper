use iced::{futures, Subscription};

pub struct Download {
    url: String,
}

/// Impl Recipe trait for usage in iced
impl<H, I> iced_native::subscription::Recipe<H, I> for Download
where
    H: std::hash::Hasher,
{
    type Output = Progress;

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;

        std::any::TypeId::of::<Self>().hash(state);
        self.url.hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, I>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        Box::pin(futures::stream::unfold(State::Ready(self.url), |state| async move {
            match state {
                State::Ready(url) => {
                    let response = reqwest::get(&url).await;

                    match response {
                        Ok(response) => {
                            if let Some(total) = response.content_length() {
                                Some((Progress::Started, State::Downloading {
                                    response,
                                    total,
                                    downloaded: 0,
                                }))
                            } else {
                                Some((
                                    Progress::Errored("could not calculate file size".into()),
                                    State::Finished,
                                ))
                            }
                        },
                        Err(e) => Some((Progress::Errored(format!("{:?}", e)), State::Finished)),
                    }
                },
                State::Downloading {
                    mut response,
                    total,
                    downloaded,
                } => match response.chunk().await {
                    Ok(Some(chunk)) => {
                        let downloaded = downloaded + chunk.len() as u64;
                        let percentage = downloaded * 100 / total;
                        let progress = format!("{} / {}", bytesize::ByteSize(downloaded), bytesize::ByteSize(total));

                        Some((Progress::Advanced(progress, percentage), State::Downloading {
                            response,
                            total,
                            downloaded,
                        }))
                    },
                    Ok(None) => Some((Progress::Finished, State::Finished)),
                    Err(e) => Some((Progress::Errored(format!("{:?}", e)), State::Finished)),
                },
                State::Finished => {
                    // We do not let the stream die, as it would start a
                    // new download repeatedly if the user is not careful
                    // in case of errors.
                    let _: () = iced::futures::future::pending().await;
                    None
                },
            }
        }))
    }
}

#[derive(Debug, Clone)]
pub enum Progress {
    Started,
    Advanced(String, u64),
    Finished,
    Errored(String),
}

pub enum State {
    Ready(String),
    Downloading {
        response: reqwest::Response,
        total: u64,
        downloaded: u64,
    },
    Finished,
}
