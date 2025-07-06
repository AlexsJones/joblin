# 🦀 Joblin

---

## 📊 Honest Score Card

**Strengths:**
- Demonstrates real async client/server architecture in Rust using Tokio
- Uses modern Rust features: async/await, channels, serde, error handling
- Good separation of concerns (client, server, shared lib)
- Includes both unit and integration tests (async and load)
- Clear, idiomatic code for message framing and protocol
- README and code are well-documented

**Weaknesses:**
- Integration tests require spawning binaries, which is slow and can be brittle
- No HTTP/REST API (yet)
- Protocol is not easily extensible to new message types without refactor
- Error handling is basic in some places (e.g., unwraps in server)
- No persistent job storage or advanced job management
- No authentication or security
- No web UI or advanced monitoring

**Grade:**

> **B+**
>
> A solid, idiomatic async Rust project that demonstrates real-world concurrency and protocol design. With a few more features (REST API, extensible protocol, persistent storage, better error handling), it could be an A.

---

**Joblin** is a minimal distributed job queue system, written in Rust, split into:

- `joblinctl`: A command-line client to submit and monitor jobs
- `joblinsvr`: A background server that receives, queues, and executes jobs

This project is part of my Rust mastery journey — designed to practice real-world async concurrency, trait design, CLI structure, and message passing.

_I have been writing this exclusively without AI, which feels like a novelty. 
But given how quickly I am forgetting rust these days... it is a necessity_

---

## 🎯 Goals & Learning Objectives

This project is helping me develop the following **advanced Rust skills**:

| Skill                | What I'm Practicing                                                   |
|----------------------|------------------------------------------------------------------------|
| **Concurrency**      | Using `tokio`, `spawn`, `await`, task scheduling                       |
| **Async I/O**        | Interacting with sockets / local communication                         |
| **Trait-based design** | Modeling extensible job behavior using `trait` and `dyn`            |
| **Workspaces**       | Managing a multi-binary project with shared logic                      |
| **Error Handling**   | Using `thiserror`, `anyhow`, `Result`, and propagation idioms          |
| **Ownership & Borrowing** | Clean data flow between threads & components                    |
| **Serialization**    | Persisting job state using `serde` and `serde_json`                    |
| **CLIs**             | Using `clap` to build ergonomic multi-command tools                    |
| **Testing**          | Writing unit + integration tests for modular logic                     |

---

## 🧱 Project Structure

```
joblin/
├── Cargo.toml          # Workspace root
├── joblinctl/          # CLI client
│   └── src/main.rs
├── joblinsvr/          # Job queue server
│   └── src/main.rs
```

---

## 🚀 Usage

From the root of the workspace:

### Build the project

```bash
make build
```

### Run the server

```bash
make run-svr
```

### Submit a job (e.g., sleep 5 seconds)

```bash
make run-ctl ARGS='add --job "sleep 5"'
```

### List jobs

```bash
make run-ctl ARGS='list'
```

---

## 🔧 Development Tools

- `make` – common workflow automation
- `cargo fmt` – formatting
- `cargo check` – static analysis
- `cargo test` – testing

---

## 🧪 Next Stretch Goals

- Add REST or local socket communication between `ctl` and `svr`
- Support different job types (trait objects)
- Add status polling and live updates
- Optional Web UI with `axum` or `warp`

---

## 🦀 Why Rust?

This project is intentionally designed to challenge my understanding of:
- Rust’s ownership model in multi-threaded code
- Async primitives
- Structuring larger, real-world Rust projects

---

## 📜 License

MIT OR Apache-2.0 — choose what suits you.

---

## 🛠️ Technology: Tokio Async Client/Server & Callbacks

This project is built on top of [Tokio](https://tokio.rs/), Rust's leading asynchronous runtime. It demonstrates:

- **Async TCP Networking:** Both the client (`joblinctl`) and server (`joblinsvr`) use Tokio's async TCP primitives to communicate efficiently without blocking threads.
- **Framed, Length-Delimited Protocol:** Messages are sent as length-delimited frames, using `tokio_util::codec::LengthDelimitedCodec` and `tokio_serde` for seamless JSON serialization/deserialization.
- **Client/Server Model:**
    - The server listens for incoming connections and processes job requests concurrently, spawning tasks for each connection.
    - The client connects to the server, sends job requests, and awaits responses.
- **Callback-based Response Handling:**
    - The client uses a callback (an async closure) to handle each response from the server, demonstrating how to pass and execute async callbacks in Rust.
- **Concurrency:**
    - The server can process multiple jobs in parallel, leveraging Tokio's task scheduling and message passing (via channels).
- **Error Handling:**
    - Uses `anyhow` and idiomatic Rust error propagation for robust async error handling.

This architecture is a practical example of how to build robust, scalable, and highly concurrent networked applications in Rust using Tokio.

---

## 🧪 Integration & Load Testing

This project includes integration tests to prove the server's asynchronous and concurrent behavior:

- **Async Concurrency Test (`async_server.rs`)**: 
    - Launches the server and submits multiple jobs in parallel from separate clients.
    - Verifies that jobs are processed concurrently (not serially) by measuring total execution time. If the server were serial, the test would take much longer.

- **Load Test (`load_test.rs`)**:
    - Spawns a large number of clients (e.g., 50), each submitting a job simultaneously.
    - Asserts that all jobs complete in much less time than if they were processed one after another, demonstrating the server's ability to handle high load and many concurrent connections.
    - Fails if the server cannot keep up or processes jobs serially.

These tests demonstrate the robustness and scalability of the async server implementation, and can be run with:

```sh
cargo test --test async_server
cargo test --test load_test
```
