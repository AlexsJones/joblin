use std::process::{Command, Stdio, Child};
use std::thread::sleep;
use std::time::{Duration, Instant};
use tokio::task;

fn spawn_server() -> Child {
    Command::new("cargo")
        .args(["run", "--package", "joblinsvr", "--bin", "joblinsvr"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to start server")
}

#[tokio::test]
async fn test_server_load() {
    // Start the server
    let mut server = spawn_server();
    sleep(Duration::from_secs(2)); // Give server time to start

    let num_clients = 50;
    let start = Instant::now();

    // Spawn many clients in parallel
    let mut handles = Vec::new();
    for _ in 0..num_clients {
        handles.push(task::spawn_blocking(|| {
            let output = Command::new("cargo")
                .args([
                    "run", "--package", "joblinctl", "--bin", "joblinctl", "--",
                    "add", "--job", "sleep 1"
                ])
                .output()
                .expect("client failed");
            assert!(output.status.success());
        }));
    }

    // Wait for all clients to finish
    for handle in handles {
        handle.await.unwrap();
    }

    let elapsed = start.elapsed();

    // Kill the server
    let _ = server.kill();

    // If jobs were handled serially, this would be >= num_clients seconds.
    // If async, should be just over 1 second.
    assert!(
        elapsed < Duration::from_secs(num_clients as u64),
        "Jobs were not handled concurrently! Elapsed: {:?}",
        elapsed
    );
    println!("Handled {} jobs in {:?}", num_clients, elapsed);
} 