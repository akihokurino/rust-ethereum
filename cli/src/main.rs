extern crate core;

use clap::{arg, Parser, ValueEnum};
use dotenv::dotenv;
use prelude::*;
use std::str::FromStr;

#[derive(ValueEnum, Clone, Debug)]
enum Command {
    Balance,
    SendEth,
    CreateMetadata,
    Mint,
    Transfer,
    Info,
    Deploy,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Package {
    RustWeb3,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Contract {
    Nft721,
    Nft1155,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    #[clap(value_enum)]
    command: Command,

    #[arg(long, default_value = "nft")]
    name: String,

    #[arg(long, default_value = "nft market sample")]
    description: String,

    #[arg(long, default_value = "sample.png")]
    image_filename: String,

    #[arg(
        long,
        default_value = "https://placehold.jp/3d4070/ffffff/500x500.png?text=Reveal"
    )]
    image_url: String,

    #[arg(long, default_value_t = 10)]
    amount: u128,

    #[arg(long, default_value = "QmPDE4pXnFvNtqJ2889HgEQUEft8KCdyMaKKt5zzw3NuMS")]
    content_hash: String,

    #[arg(long, default_value = "rust-web3")]
    #[clap(value_enum)]
    package: Package,

    #[arg(long, default_value = "Polygon")]
    network: String,

    #[arg(long, default_value = "nft721")]
    #[clap(value_enum)]
    contract: Contract,

    #[arg(long, default_value_t = 0.1)]
    ether: f64,

    #[arg(long, default_value = "0x0E91D6613a84d7C8b72a289D8b275AF7717C3d2E")]
    to_address: String,

    #[arg(long, default_value_t = 1)]
    token_id: u128,
}

#[tokio::main]
pub async fn main() {
    dotenv().ok();

    if let Err(e) = execute(Args::parse()).await {
        println!("error: {:?}", e);
        return;
    }
}

async fn execute(args: Args) -> CliResult<()> {
    let network = Network::from_str(&args.network).unwrap();

    match args.command {
        Command::Balance => match args.package {
            Package::RustWeb3 => impl_rust_web3::get_balance(network)
                .await
                .map_err(Error::from),
        },
        Command::SendEth => match args.package {
            Package::RustWeb3 => impl_rust_web3::send_eth(network, args.ether, args.to_address)
                .await
                .map_err(Error::from),
        },
        Command::CreateMetadata => {
            if !args.image_url.is_empty() {
                ipfs::create_metadata_from_url(args.name, args.description, args.image_url)
                    .await
                    .map_err(Error::from)
            } else {
                ipfs::create_metadata_from_file(args.name, args.description, args.image_filename)
                    .await
                    .map_err(Error::from)
            }
        }
        Command::Mint => match args.package {
            Package::RustWeb3 => match args.contract {
                Contract::Nft721 => {
                    let cli = impl_rust_web3::nft_721::client::Client::new(network);
                    cli.mint(args.content_hash.clone())
                        .await
                        .map_err(Error::from)
                }
                Contract::Nft1155 => {
                    let cli = impl_rust_web3::nft_1155::client::Client::new(network);
                    cli.mint(args.content_hash.clone(), args.amount)
                        .await
                        .map_err(Error::from)
                }
            },
        },
        Command::Transfer => match args.package {
            Package::RustWeb3 => match args.contract {
                Contract::Nft721 => {
                    let cli = impl_rust_web3::nft_721::client::Client::new(network);
                    cli.transfer(
                        impl_rust_web3::parse_address(args.to_address).unwrap(),
                        args.token_id,
                    )
                    .await
                    .map_err(Error::from)
                }
                Contract::Nft1155 => {
                    let cli = impl_rust_web3::nft_1155::client::Client::new(network);
                    cli.transfer(
                        impl_rust_web3::parse_address(args.to_address).unwrap(),
                        args.token_id,
                    )
                    .await
                    .map_err(Error::from)
                }
            },
        },
        Command::Info => match args.package {
            Package::RustWeb3 => match args.contract {
                Contract::Nft721 => {
                    let cli = impl_rust_web3::nft_721::client::Client::new(network);
                    println!("------------------------------------------------------------");
                    println!("Nft721 info: {}", network.nft_721_address());
                    println!("name = {}", cli.name().await?);
                    println!("latestTokenId = {}", cli.latest_token_id().await?);
                    println!("totalSupply = {:?}", cli.total_supply().await?);
                    println!("totalOwned = {:?}", cli.total_owned().await?);
                    println!("------------------------------------------------------------");
                    Ok(())
                }
                Contract::Nft1155 => {
                    let cli = impl_rust_web3::nft_1155::client::Client::new(network);
                    println!("------------------------------------------------------------");
                    println!("Nft1155 info: {}", network.nft_1155_address());
                    println!("name = {}", cli.name().await?);
                    println!("latestTokenId = {}", cli.latest_token_id().await?);
                    println!("totalSupply = {:?}", cli.total_supply().await?);
                    println!("totalOwned = {:?}", cli.total_owned().await?);
                    println!("------------------------------------------------------------");
                    Ok(())
                }
            },
        },
        Command::Deploy => match args.package {
            Package::RustWeb3 => match args.contract {
                Contract::Nft721 => {
                    let cli = impl_rust_web3::nft_721::client::Client::new(network);
                    cli.deploy().await.map_err(Error::from)
                }
                Contract::Nft1155 => {
                    let cli = impl_rust_web3::nft_1155::client::Client::new(network);
                    cli.deploy().await.map_err(Error::from)
                }
            },
        },
    }
}

pub type CliResult<T> = Result<T, Error>;

#[derive(thiserror::Error, Debug, PartialOrd, PartialEq, Clone)]
pub enum Error {
    #[error("internal error: {0}")]
    Internal(String),
}

impl From<impl_rust_web3::Error> for Error {
    fn from(e: impl_rust_web3::Error) -> Self {
        let msg = format!("rust-web3 error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<ipfs::Error> for Error {
    fn from(e: ipfs::Error) -> Self {
        let msg = format!("ipfs error: {:?}", e);
        Self::Internal(msg)
    }
}
