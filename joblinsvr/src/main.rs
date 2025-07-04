use tokio::io;
use futures::prelude::*;
use log::{debug, error};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Sender, Receiver};
use std::process::Command;
use shell_words::split;
use joblinlib::connection::ConnectionManager;

#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    // Start the local listener

    let mut connection_manager = ConnectionManager::new("127.0.0.1:2345");
    
    let (tx, mut rx): (Sender<String>, Receiver<String>) = mpsc::channel(10);

    tokio::spawn(async move {
        // Process jobs
        loop {
            if let Some(msg) = rx.recv().await {
                debug!("Processing job {msg}");
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
                                error!("{e}");
                            }
                        }
                    },
                    Err(e) => error!("{e}"),
                }
            }
        }
    });

    connection_manager.listen().await.unwrap();
    
    loop {
        let tx = tx.clone();
        connection_manager.accept_connection(|x| {
            let tx = tx.clone();
            async move {
           tx.send(x).await.unwrap();
        }}).await.unwrap();

    }
}

#[cfg(test)]
mod tests {
    
    use joblinlib::types::AddMessageRequest;
    

    #[test]
    fn test_add_message_request_from_value() {
        let value = serde_json::json!({"job": "echo test"});
        let req = AddMessageRequest::from_value(value);
        assert_eq!(req.job, "echo test");
    }

    #[test]
    fn test_split_command() {
        let cmd = "ls -l /";
        let parts = shell_words::split(cmd).unwrap();
        assert_eq!(parts[0], "ls");
        assert_eq!(parts[1], "-l");
        assert_eq!(parts[2], "/");
    }
}