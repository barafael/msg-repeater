use std::{net::SocketAddr, path::PathBuf, time::Duration};

use anyhow::Context;
use clap::{arg, Parser, Subcommand};

static DEFAULT_INTERVAL: Duration = Duration::from_secs(1);

fn parse_hex_digit(s: &str) -> anyhow::Result<u8> {
    u8::from_str_radix(s, 16).context("Failed to parse hex byte")
}

#[derive(Parser, Debug)]
#[command(author, version)]
pub struct Arguments {
    /// Socket to bind on
    #[arg(short, long, default_value = "127.0.0.1:8080")]
    pub socket: SocketAddr,

    #[arg(long, short, default_value_t = DEFAULT_INTERVAL.into())]
    pub interval: humantime::Duration,

    #[command(subcommand)]
    pub action: Action,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    /// Send raw bytes
    Raw {
        /// Bytes to send (hexadecimal)
        #[arg(num_args = 1.., value_delimiter = ' ', value_parser = parse_hex_digit)]
        bytes: Vec<u8>,
    },

    /// Send a file
    File {
        #[arg(short, long, value_parser, default_value = "msg.bin")]
        path: PathBuf,
    },
}
