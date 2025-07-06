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
async fn test_server_handles_jobs_concurrently() {
    // Start the server
    let mut server = spawn_server();
    sleep(Duration::from_secs(2)); // Give server time to start

    let start = Instant::now();

    // Spawn two clients that submit jobs with different sleep durations
    let client1 = task::spawn_blocking(|| {
        Command::new("cargo")
            .args([
                "run", "--package", "joblinctl", "--bin", "joblinctl", "--",
                "add", "--job", "sleep 2"
            ])
            .output()
            .expect("client1 failed");
    });

    let client2 = task::spawn_blocking(|| {
        Command::new("cargo")
            .args([
                "run", "--package", "joblinctl", "--bin", "joblinctl", "--",
                "add", "--job", "sleep 2"
            ])
            .output()
            .expect("client2 failed");
    });

    // Wait for both clients to finish
    let _ = tokio::join!(client1, client2);

    let elapsed = start.elapsed();

    // Kill the server
    let _ = server.kill();

    // If jobs were handled serially, this would be >= 4s. If async, should be just over 2s.
    assert!(
        elapsed < Duration::from_secs(4),
        "Jobs were not handled concurrently! Elapsed: {:?}",
        elapsed
    );
} 