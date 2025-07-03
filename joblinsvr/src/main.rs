use tokio::io;
use tokio::net::TcpListener;
use tokio_serde::formats::*;
use tokio_util::codec::{FramedRead, LengthDelimitedCodec};
use futures::prelude::*;
use serde_json::Value;
use log::{debug, error};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Sender, Receiver};
use std::process::Command;
use shell_words::split;
use joblinlib::types::AddMessageRequest;

#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    // Start the local listener
    let listener = TcpListener::bind("127.0.0.1:2345").await?;
    let (tx, mut rx): (Sender<String>, Receiver<String>) = mpsc::channel(10);

    tokio::spawn(async move {   
        // Process jobs
        loop {
            if let Some(msg) = rx.recv().await {
                debug!("Processing job {}", msg);
                match split(msg.as_str()) {
                    Ok(parts) => {
                       println!("Command: {}", &parts[0]);
                        println!("Args: {:?}", &parts[1..]);
                        let mut shell = Command::new("sh");
                        shell.arg("-c").arg(&parts[0]).args(&parts[1..]);
                        match shell.output() {
                            Ok(s) => {
                                debug!("Command: {}:{}",s.status, String::from_utf8_lossy(&s.stdout));
                            }
                            Err(e) => {
                                error!("{}", e);
                            }
                        }
                    },
                    Err(e) => error!("{}", e),
                }
            }
        }
    });

    loop {
        let (socket,_ ) = listener.accept().await?;
        let length_delimited = FramedRead::new(socket,
                                               LengthDelimitedCodec::new());
        // Deserialize frames
        let mut deserialized = tokio_serde::SymmetricallyFramed::new(
            length_delimited,
            SymmetricalJson::<Value>::default(),
        );


        // Spawn a task that prints all received messages to STDOUT
        tokio::spawn(
            {
                let tx = tx.clone();
            async move {
                while let Some(msg) = deserialized.try_next().await.unwrap() {

                    let amr = AddMessageRequest::from_value(msg);
                    //try to deserialise
                    tx.send(amr.job).await.unwrap()
                }
            }
        });
    }
}