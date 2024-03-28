
mod client;
mod server;

use clap::{Args, Parser, Subcommand};
use std::{fs, error::Error};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long)]
    cert: String,

    #[arg(short, long)]
    key: String,
}

#[derive(Subcommand)]
enum Commands {
    Server(ServerArgs),
    Client(ClientArgs),
}

#[derive(Args)]
struct ServerArgs {
    address: Option<String>,
    port: Option<u16>,
}

#[derive(Args)]
struct ClientArgs {
    address: Option<String>,
    port: Option<u16>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let cert_pem = fs::read_to_string(cli.cert)
        .expect("Failed to read cert");
    let key_pem = fs::read_to_string(cli.key)
        .expect("Failed to read key");

    match &cli.command {
        Commands::Server(args) => {
            let addr = args.address
                .as_ref()
                .expect("Invalid address")
                .to_string();
            let port = args.port
                .expect("Invalid port");

            server::_server(cert_pem.as_str(), key_pem.as_str(), addr.as_str(), port)
                .await
                .expect("Unable to start server");
        }
        Commands::Client(args) => {
            let addr = args.address
                .as_ref()
                .expect("Invalid address")
                .to_string();
            let port = args.port
                .expect("Invalid port");
            client::_client(cert_pem.as_str(), addr.as_str(), port)
                .await
                .expect("Unable to start client");
        }
    }
    Ok(())
}

