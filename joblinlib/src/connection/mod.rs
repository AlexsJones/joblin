use tokio_util::codec::Framed;
use tokio_serde::formats::SymmetricalJson;
use tokio_serde::{SymmetricallyFramed, Framed as TokioSerdeFramed};

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncRead, AsyncWrite};
use futures::{SinkExt, StreamExt};

use tokio_util::codec::LengthDelimitedCodec;

use crate::types::{AddMessageRequest};
use tokio::io;

use tokio_serde::formats::*;
use tokio_util::codec::{FramedRead, };
use futures::prelude::*;
use serde_json::Value;

use tokio::sync::mpsc;
use tokio::sync::mpsc::{Sender, Receiver};
use std::process::Command;


#[derive(Debug)]
pub struct ConnectionManager {
    path: String,
    length_delimited: Option<Framed<TcpStream, LengthDelimitedCodec>>,
    listener: Option<TcpListener>
}

type JsonFramedConnection = tokio_serde::Framed<
    FramedRead<tokio::net::TcpStream, LengthDelimitedCodec>,
    AddMessageRequest,  // Changed from Value
    AddMessageRequest,  // Changed from Value
    Json<AddMessageRequest, AddMessageRequest>
>;


impl ConnectionManager {
    pub async fn connect(&mut self) -> Result<(), anyhow::Error>{
        self.length_delimited = 
            Some(Framed::new(TcpStream::connect(&self.path).await?,
                        LengthDelimitedCodec::new()));
        Ok(())
    }

    pub async fn listen(&mut self) -> Result<(), anyhow::Error>{
        self.listener = Some(TcpListener::bind("127.0.0.1:2345").await?);
        Ok(())
    }
    
    pub async fn send(&mut self, add_message_request: AddMessageRequest, 
    response: fn(&str)) -> Result<(), anyhow::Error> {
        let mut serialized = SymmetricallyFramed::new(
            self.length_delimited.as_mut().ok_or_else(|| anyhow::anyhow!("Not connected"))?,
            SymmetricalJson::<AddMessageRequest>::default(),
        );

        serialized
            .send(add_message_request)
            .await?;
        if let Some(message) = serialized.next().await {
            response(message?.job.as_str())
        }
        Ok(())
    }
    /// Accepts a new connection and sets up JSON frame deserialization
    /// # Returns
    /// * `Result<()>` - Ok if the connection was successfully accepted
    pub async fn accept_connection(&mut self) -> Result<(), anyhow::Error> {
        let (socket, _) = self.listener
            .as_mut()
            .unwrap()
            .accept()
            .await?;

        let length_delimited = FramedRead::new(socket, LengthDelimitedCodec::new());
        let mut deserialized: JsonFramedConnection = self.create_json_framed(length_delimited);

        while let Some(message) = deserialized.try_next().await? {
            println!("{:?}", message.job);
        }


        Ok(())
    }
    fn create_json_framed(&self, length_delimited: FramedRead<TcpStream, LengthDelimitedCodec>) -> JsonFramedConnection {
        tokio_serde::SymmetricallyFramed::new(
            length_delimited,
            SymmetricalJson::<AddMessageRequest>::default(),
        )
    }

    pub fn new(conn: &str) -> Self {
        Self {
            path: conn.to_string(),
            length_delimited: None,
            listener:None
        }
    }

}