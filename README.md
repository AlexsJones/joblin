# 🦀 Joblin

**Joblin** is a minimal distributed job queue system, written in Rust, split into:

- `joblinctl`: A command-line client to submit and monitor jobs
- `joblinsvr`: A background server that receives, queues, and executes jobs

This project is part of my Rust mastery journey — designed to practice real-world async concurrency, trait design, CLI structure, and message passing.

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
