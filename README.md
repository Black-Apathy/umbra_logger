# 🏦 Umbra Logger

> A high-performance, memory-safe command-line interface (CLI) for advanced personal finance tracking.

Umbra Logger is engineered entirely in **Rust** to guarantee zero-cost abstractions and memory safety. It replaces slow, web-based finance trackers with a lightning-fast terminal UI, utilizing a localized SQLite database for complete data ownership and rapid I/O operations.

## ✨ Core Features

* **Interactive Terminal UI:** Built with `inquire` for sticky-state interactive menus, allowing seamless data entry without typing out rigid command flags.
* **Batch Transaction Processing:** Queue multiple transactions in memory and commit them to the database in a single batch to optimize write speeds.
* **Real-Time Analytics:** Instantly calculate dynamic account balances (Opening, Credits, Debits, Total) using optimized SQL queries.
* **100% Localized Data:** Persists data securely to a local SQLite (`.db`) file via `rusqlite`, ensuring maximum privacy.

## 🛠️ Tech Stack

* **Language:** Rust (Edition 2024)
* **Database:** SQLite3 (`rusqlite` crate)
* **UI / Prompting:** `inquire`
* **Date / Time Handling:** `chrono`
* **Environment Variables:** `dotenvy`

## 🚀 Getting Started

### Prerequisites
Ensure you have the Rust toolchain and Cargo installed on your system.
```bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh

```

### Installation & Run

1. Clone the repository:

```bash
git clone [https://github.com/Black-Apathy/umbra_logger.git](https://github.com/Black-Apathy/umbra_logger.git)
cd umbra_logger

```

2. Set up your environment file:
Create a `.env` file in the root directory and define your database path:

```env
DATABASE_URL=MyFinance.db

```

3. Build and execute the CLI:

```bash
cargo run --release

```

## 🏗️ Project Architecture

The codebase is modularized for enterprise scalability:

* `main.rs` - Handles the application loop and interactive UI rendering.
* `db.rs` - Encapsulates all SQLite connection logic, prepared statements, and data fetching.
* `models.rs` - Defines strictly typed Rust structs mapping directly to relational database schemas.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](https://www.google.com/search?q=LICENSE) file for details.
