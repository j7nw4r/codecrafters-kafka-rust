#![allow(unused_imports)]

mod message;

use std::io;
use std::io::Read;
use tokio::net::{TcpListener, TcpStream};
use anyhow::{bail, Context};

const ADDRESS: &str = "127.0.0.1:9092";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind(ADDRESS)
        .await
        .context("could not bind to address")?;

    while let Ok((stream, _)) = listener.accept().await {
        handle_stream_result(stream)?;
    }
    Ok(())
}

fn handle_stream_result (_tcp_stream: TcpStream) -> anyhow::Result<()> {
    Ok(())
}