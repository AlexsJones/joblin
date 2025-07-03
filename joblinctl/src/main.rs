use futures::prelude::*;
use serde_json::json;
use tokio::net::TcpStream;
use tokio_serde::formats::*;
use tokio_util::codec::{FramedWrite, LengthDelimitedCodec};
use clap::{Parser};

#[derive(Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser, Debug, Clone)]
enum Command {
    Add {
        #[clap(short, long)]
        job: String,
    },
    List {

    }
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let args = Args::parse();
    let socket = TcpStream::connect("127.0.0.1:2345").await.unwrap();
    let length_delimited = FramedWrite::new(socket, LengthDelimitedCodec::new());
    let mut serialized =
        tokio_serde::SymmetricallyFramed::new(length_delimited, SymmetricalJson::default());

    match args.command {
        Command::Add { job } => {

            let add_message_request = joblinlib::types::AddMessageRequest {
                job
            };

            serialized
                .send(add_message_request)
                .await
                .unwrap()
        },
        Command::List { } => {

        }
    }



}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use joblinlib::types::AddMessageRequest;

    #[test]
    fn test_command_add_parsing() {
        let args = vec!["joblinctl", "add", "--job", "echo test"];
        let args = Args::parse_from(args);
        match args.command {
            Command::Add { job } => assert_eq!(job, "echo test"),
            _ => panic!("Expected Add command"),
        }
    }

    #[test]
    fn test_add_message_request_construction() {
        let req = AddMessageRequest { job: "ls -l".to_string() };
        assert_eq!(req.job, "ls -l");
    }
}
