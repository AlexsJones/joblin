use tokio_util::codec::Framed;

use tokio::net::{TcpListener, TcpStream};
use futures::{SinkExt, StreamExt};

use tokio_util::codec::LengthDelimitedCodec;

use crate::types::{AddMessageRequest, AddMessageResponse};

use tokio_serde::formats::*;
use futures::prelude::*;
use log::error;

#[derive(Debug)]
pub struct ConnectionManager {
    path: String,
    length_delimited: Option<Framed<TcpStream, LengthDelimitedCodec>>,
    listener: Option<TcpListener>
}

type InConnection = tokio_serde::Framed<
    Framed<tokio::net::TcpStream, LengthDelimitedCodec>,
    AddMessageRequest,  // Changed from Value
    AddMessageResponse,  // Changed from Value
    Json<AddMessageRequest, AddMessageResponse>
>;
type OutConnection = tokio_serde::Framed<
    Framed<tokio::net::TcpStream, LengthDelimitedCodec>,
    AddMessageResponse,  // Changed from Value
    AddMessageRequest,  // Changed from Value
    Json<AddMessageResponse, AddMessageRequest>
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
    where F: Fn(AddMessageResponse) -> FUT,
          FUT: Future<Output = ()>
    {
        let Some(stream) = self.length_delimited.take() else {
            return Err(anyhow::anyhow!("Not connected"));
        };
        let mut framed: OutConnection = self.create_outgoing_connection(stream);
        framed
            .send(add_message_request)
            .await?;
        if let Some(Ok(message)) = framed.next().await {
            response(message).await;
        }
        // Optionally, save the connection for reuse:
        self.length_delimited = Some(framed.into_inner());
        Ok(())
    }
    /// Accepts a new connection and sets up JSON frame deserialization
    /// # Returns
    /// * `Result<()>` - Ok if the connection was successfully accepted
    pub async fn accept_connection<F,Fut
    >(&mut self, cb: F) -> Result<(), anyhow::Error>
    where F: Fn(String) -> Fut + Send + 'static,
          Fut: Future<Output = ()> + Send + 'static,

    {
        let (socket, _) = self.listener
            .as_mut()
            .unwrap()
            .accept()
            .await?;

        // spawn a thread

        tokio::spawn(
            async move {
                let length_delimited = Framed::new(socket, LengthDelimitedCodec::new());
                let mut framed: InConnection = ConnectionManager::create_incoming_connection(length_delimited);
                while let Some(message) = framed.next().await {
                    match message {
                        Ok(message) => {
                            cb(message.job).await;
                            framed.send(AddMessageResponse {
                                // Callback from the server to the client
                                message: "OK".to_string()
                            }).await.expect("something went wrong!");
                        }
                        Err(e) => {
                            error!("{e}");
                        }
                    }
                }
            });
        Ok(())
    }
    // Static method as this gets used inside a spawned thread
    fn create_incoming_connection(
       
        length_delimited: Framed<TcpStream, LengthDelimitedCodec>,
    ) -> InConnection {
        tokio_serde::Framed::new(
            length_delimited,
            Json::<AddMessageRequest, AddMessageResponse>::default(),
        )
    }
    fn create_outgoing_connection(
        &self,
        length_delimited: Framed<TcpStream, LengthDelimitedCodec>,
    ) -> OutConnection {
        tokio_serde::Framed::new(
            length_delimited,
            Json::<AddMessageResponse, AddMessageRequest>::default(),
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