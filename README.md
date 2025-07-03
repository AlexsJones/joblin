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
