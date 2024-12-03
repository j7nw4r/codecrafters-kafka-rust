#![allow(unused_imports)]

mod message;

use std::io;
use std::io::Read;
use tokio::net::{TcpListener, TcpStream};
use anyhow::{bail, Context};
use bytes::{BufMut, BytesMut};
use tokio::io::AsyncWriteExt;

const ADDRESS: &str = "127.0.0.1:9092";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind(ADDRESS)
        .await
        .context("could not bind to address")?;

    let Ok((stream, _)) = listener.accept().await else {
        bail!("could not accept connection");
    };
    if let Err(e) = handle_stream(stream).await {
        println!("{}", e);
    }

    Ok(())
}

async fn handle_stream(mut tcp_stream: TcpStream) -> anyhow::Result<()> {
    let (_read_half, mut write_half) = tcp_stream.split();

    let mut response_bytes = BytesMut::new();
    response_bytes.put_i32(0);
    response_bytes.put_i32(7);
    let _ = write_half.write(&response_bytes).await
        .context("could not write to response")?;
    write_half.flush().await.context("could not flush to response")?;
    Ok(())
}