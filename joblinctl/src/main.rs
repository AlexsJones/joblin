
use futures::SinkExt;
use clap::Parser;
use joblinlib::connection::{ConnectionManager};
use joblinlib::types::{AddMessageRequest};
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
async fn main()  {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let args = Args::parse();
    
    let mut connection_manager = ConnectionManager::new("127.0.0.1:2345");
    
    connection_manager.connect().await.unwrap();
    
    match args.command {
        Command::Add { job } => {
            connection_manager.send(AddMessageRequest{
                job: job.clone()}, |x| async move{
                println!("{x:?}");
            }).await.unwrap();
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
