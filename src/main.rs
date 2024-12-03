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

    while let Ok((stream, _)) = listener.accept().await {
        handle_stream_result(stream).await?;
    }
    Ok(())
}

async fn handle_stream_result (mut tcp_stream: TcpStream) -> anyhow::Result<()> {
    let (_read_half, mut write_half) = tcp_stream.split();

    let mut response_bytes = BytesMut::new();
    response_bytes.put_i32(7);
    let _ = write_half.write(response_bytes.iter().as_slice()).await
        .context("could not write to response")?;
    Ok(())
}