use anyhow::Result;
use arguments::{Action, Arguments};
use clap::Parser;

mod arguments;

#[cfg(not(feature = "async-spawning"))]
fn main() -> Result<()> {
    use std::{io::Write, thread};
    let args = Arguments::parse();

    let message = match args.action {
        Action::Raw { bytes } => bytes,
        Action::File { path } => std::fs::read(path)?,
    };

    let listener = std::net::TcpListener::bind(args.socket).expect("Failed to bind");

    let mut incoming = listener.incoming();
    while let Some(Ok(mut stream)) = incoming.next() {
        let message = message.clone();
        let delay = args.interval.into();
        std::thread::spawn(move || {
            println!("Client connected");
            while let Ok(()) = stream.write_all(&message) {
                thread::sleep(delay);
            }
            println!("Client disconnected");
        });
    }
    Ok(())
}

#[cfg(feature = "async-spawning")]
#[tokio::main(flavor = "current_thread")] // could also set `multi_thread` to enable work-stealing threads
async fn main() -> Result<()> {
    use tokio::io::AsyncWriteExt;

    let args = Arguments::parse();

    let message = match args.action {
        Action::Raw { bytes } => bytes,
        Action::File { path } => tokio::fs::read(path).await?,
    };

    let listener = tokio::net::TcpListener::bind(args.socket)
        .await
        .expect("Failed to bind");

    loop {
        let (mut stream, socket_addr) = listener.accept().await?;
        let message = message.clone();
        let delay = args.interval.into();
        tokio::task::spawn(async move {
            println!("Client {socket_addr} connected");
            // TODO this loop is not time-accurate. Use a .tick().await instead of a sleep.
            while let Ok(()) = stream.write_all(&message).await {
                tokio::time::sleep(delay).await;
            }
            println!("Client {socket_addr} disconnected");
        });
    }
}
