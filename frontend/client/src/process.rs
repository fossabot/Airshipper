use std::process::{ExitStatus, Stdio};
use tokio::{
    io::BufReader,
    prelude::*,
    process::Command,
    stream::{Stream, StreamExt},
};

/// Returns a stream of stdout/stderr lines of the Process
/// TODO: Handle the case if process couldn't be spawned
pub fn stream_process(mut cmd: Command) -> impl Stream<Item = Result<ProcessUpdate, std::io::Error>> {
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    let mut child = cmd.spawn().expect("Failed to spawn process.");

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let reader = BufReader::new(stdout).lines().merge(BufReader::new(stderr).lines());

    // Ensure the child process is spawned in the runtime so it can
    // make progress on its own while we await for any output.
    let exit_status = tokio::spawn(async { child.await });
    reader
        .map(|x| Ok(ProcessUpdate::Line(x?)))
        .chain(futures::stream::once(async {
            Ok(ProcessUpdate::Exit(exit_status.await??))
        }))
}

#[derive(Debug)]
pub enum ProcessUpdate {
    Line(String),
    Exit(ExitStatus),
}
