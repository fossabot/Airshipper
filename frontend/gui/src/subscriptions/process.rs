use client::process::ProcessUpdate;
use iced::futures;
use iced_native::subscription::Recipe;
use tokio::process::Command;

pub(crate) struct Process(Command);

impl<H, I> Recipe<H, I> for Process
where
    H: std::hash::Hasher,
{
    type Output = Result<ProcessUpdate, std::io::Error>;

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;

        std::any::TypeId::of::<Self>().hash(state);
        // TODO: is exploiting the Debug impl for hashing a good idea?
        format!("{:?}", self.0).hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, I>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        Box::pin(client::process::stream_process(self.0))
    }
}
