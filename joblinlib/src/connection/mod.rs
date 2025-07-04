use tokio_util::codec::Framed;
use tokio_serde::formats::SymmetricalJson;
use tokio_serde::SymmetricallyFramed;

use tokio::net::{TcpListener, TcpStream};
use futures::{SinkExt, StreamExt};

use tokio_util::codec::LengthDelimitedCodec;

use crate::types::{AddMessageRequest};

use tokio_serde::formats::*;
use tokio_util::codec::{FramedRead, };
use futures::prelude::*;



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
        self.listener = Some(TcpListener::bind(&self.path).await?);
        Ok(())
    }
    
    pub async fn send<F,FUT>(&mut self, add_message_request: AddMessageRequest, 
    response: F) -> Result<(), anyhow::Error>
    where F: Fn(String) -> FUT,
          FUT: Future<Output = ()>
    {
        let mut serialized = SymmetricallyFramed::new(
            self.length_delimited.as_mut().ok_or_else(|| anyhow::anyhow!("Not connected"))?,
            SymmetricalJson::<AddMessageRequest>::default(),
        );

        serialized
            .send(add_message_request)
            .await?;
        if let Some(message) = serialized.next().await {
            response(message?.job).await;
        }
        Ok(())
    }
    /// Accepts a new connection and sets up JSON frame deserialization
    /// # Returns
    /// * `Result<()>` - Ok if the connection was successfully accepted
    pub async fn accept_connection<F,Fut
    >(&mut self, cb: F) -> Result<(), anyhow::Error>
    where F: Fn(String) -> Fut,
          Fut: Future<Output = ()>

    {
        let (socket, _) = self.listener
            .as_mut()
            .unwrap()
            .accept()
            .await?;

        let length_delimited = FramedRead::new(socket, LengthDelimitedCodec::new());
        let mut deserialized: JsonFramedConnection = self.create_json_framed(length_delimited);

        while let Some(message) = deserialized.try_next().await? {
            cb(message.job).await;
        }
        
        Ok(())
    }
    fn create_json_framed(&self, length_delimited: FramedRead<TcpStream,
        LengthDelimitedCodec>) -> JsonFramedConnection {
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