# OptiTrade - Agentic Low-Latency Trading System

![Build Status](https://github.com/williamsryan/OptiTrade/actions/workflows/rust-build.yml/badge.svg)
![Rust](https://img.shields.io/badge/language-Rust-orange)
![License](https://img.shields.io/badge/license-GPLv3-blue)

## 📌 Overview
**OptiTrade** is a **modular, distributed trading system**, built using **a full Rust stack**. The system is designed to handle **real-time market data ingestion, execution, risk management, and AI-driven trading strategies** with **low-latency distributed processing**.

## 🚀 Features
✅ **Distributed Market Data Processing**: Multiple nodes for WebSocket streaming from **Binance, Deribit, OKX**  
✅ **Ultra-Fast Execution Engine**: Sub-millisecond trading using **Axum, Tokio, and kernel-bypass optimizations**  
✅ **Agentic AI Trading Strategies**: AI-driven strategy bots that execute trades autonomously  
✅ **Risk Management & Hedging**: Automated **delta/gamma risk balancing**  
✅ **Leptos (WebAssembly) UI**: Fast, interactive trading dashboard for execution and analytics  
✅ **Scalable & Fault-Tolerant**: Built on **NATS/Kafka messaging**, ensuring resilience  
✅ **Backtesting Framework**: Run historical simulations to refine execution logic  
✅ **Cloud Deployment Ready**: Optimized for **AWS/GCP scaling**  

---

## 📂 Project Structure
```
OptiTrade/
├── backend/                        # Core backend for agents
│   ├── market_data/                # Market Data Agent (WebSocket ingestion)
│   │   ├── src/
│   │   │   ├── main.rs              # Market Data Agent entry point
│   │   │   ├── lib.rs               # Handles WebSocket & Kafka publishing
│   │   │   ├── websocket.rs         # Handles WebSocket connections
│   │   │   ├── kafka_producer.rs    # Sends messages to Kafka
│   │   │   ├── data_parser.rs       # Parses incoming market data
│   │   ├── Cargo.toml
│   ├── storage_agent/               # Stores market data into TimescaleDB
│   │   ├── src/
│   │   │   ├── main.rs              # Storage Agent entry point
│   │   │   ├── lib.rs               # Kafka consumer & database writer
│   │   │   ├── kafka_consumer.rs    # Reads market data from Kafka
│   │   │   ├── db_writer.rs         # Inserts processed data into TimescaleDB
│   │   ├── init_db.sql              # SQL schema for TimescaleDB
│   │   ├── Cargo.toml
│   ├── backtesting/                 # Runs historical strategy simulations
│   │   ├── src/
│   │   │   ├── main.rs              # Backtesting Engine entry point
│   │   │   ├── lib.rs               # Core strategy simulation logic
│   │   │   ├── strategy.rs          # Trading strategies implementation
│   │   │   ├── data_loader.rs       # Loads historical data from TimescaleDB
│   │   │   ├── risk_management.rs   # Enforces risk controls
│   │   ├── Cargo.toml
│   ├── execution_agent/             # Executes trades via Alpaca API
│   │   ├── src/
│   │   │   ├── main.rs              # Execution Agent entry point
│   │   │   ├── lib.rs               # Handles trade execution logic
│   │   │   ├── kafka_consumer.rs    # Listens for trading signals
│   │   │   ├── order_executor.rs    # Places orders via Alpaca API
│   │   │   ├── risk_checker.rs      # Ensures position & risk limits
│   │   ├── Cargo.toml
│   ├── analytics/                   # Computes & serves trading analytics
│   │   ├── src/
│   │   │   ├── main.rs              # Analytics Engine entry point
│   │   │   ├── lib.rs               # Core analytics logic
│   │   │   ├── dashboard.rs         # Serves analytics dashboard data
│   │   │   ├── indicators.rs        # Computes technical indicators
│   │   ├── Cargo.toml
├── frontend/                        # Web-based UI built with Rust/WASM
│   ├── src/
│   │   ├── main.rs                  # WebAssembly UI entry point
│   │   ├── components/               # UI components (Leptos/Yew)
│   │   │   ├── market_view.rs        # Live market data visualization
│   │   │   ├── trade_panel.rs        # Trade execution UI
│   │   │   ├── analytics_view.rs     # Trading analytics dashboard
│   ├── Cargo.toml
├── infra/                           # Infrastructure & deployment config
│   ├── messaging/
│   │   ├── docker-compose.yml       # Kafka, Zookeeper, TimescaleDB services
│   ├── deployment/
│   │   ├── k8s-configs/             # Kubernetes deployment configs
│   │   ├── monitoring/              # Grafana/Prometheus setup
├── Cargo.toml
├── README.md
---

## **🔥 Distributed System Components & Roles**

### **📡 Market Data Agent**
- **Purpose:** Connects to exchange WebSockets, processes market data (order books, trades).
- **Tech:** Rust, `tokio-tungstenite`, `serde_json`.
- **Output:** Publishes processed market data to **NATS/Kafka**.

### **🚀 Execution Agent**
- **Purpose:** Handles low-latency order execution, receives trade signals from Strategy Agent.
- **Tech:** Axum, `reqwest`, REST/WebSockets.
- **Output:** Places trades, publishes execution reports.

### **🧠 Strategy Agent (AI/Algorithmic Bots)**
- **Purpose:** Runs AI-driven and rule-based trading strategies, sends execution signals.
- **Tech:** `tch-rs` (Torch for Rust), `ndarray`, Reinforcement Learning (RL).

### **⚠️ Risk Management Agent**
- **Purpose:** Monitors trade exposure, position sizing, and enforces risk limits.
- **Tech:** `sqlx` (PostgreSQL), `rust_decimal`, real-time monitoring.

### **📊 Leptos WebAssembly UI**
- **Purpose:** Interactive UI for trade execution, monitoring, strategy configuration.
- **Tech:** Leptos, WebAssembly (Wasm), WebSockets for live updates.

---

## **🔧 Installation & Setup**
### **1. Clone the Repository**
```bash
git clone git@github.com:williamsryan/OptiTrade.git
cd OptiTrade
```

### **2. Build & Run Backend Services**
```bash
cargo build --release
cargo run --bin market_data_agent
cargo run --bin execution_agent
cargo run --bin strategy_agent
cargo run --bin risk_manager
```

### **3. Start UI Server**
```bash
cd frontend
cargo leptos serve
```

### **4. Deploy Using Docker Compose (Optional)**
```bash
docker-compose up --build
```

---

## **📜 API Endpoints**
### **Market Data Streaming**
- **Binance WebSocket API**: `wss://stream.binance.com:9443/ws`
- **Deribit WebSocket API**: `wss://www.deribit.com/ws/api/v2`

### **Order Execution API (Example Call)**
```rust
async fn place_order(exchange: &str, instrument: &str, size: f64, price: f64) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({
        "exchange": exchange,
        "instrument": instrument,
        "size": size,
        "price": price
    });
    let _response = client.post("https://api.exchange.com/order")
        .json(&payload)
        .send()
        .await?;
    Ok(())
}
```

---

## **🚀 Roadmap**
| Feature | Status |
|---------|--------|
| **Market Data Ingestion (WebSockets)** | ✅ Done |
| **Trading Execution API (REST/WebSocket)** | ✅ Done |
| **Strategy Engine (AI/Rule-based)** | 🟡 In Progress |
| **Risk Management System** | 🔜 Upcoming |
| **Backtesting & Simulation Module** | 🔜 Upcoming |
| **Cloud Deployment (AWS/GCP)** | 🚀 Future Plan |

---

## 📄 License
This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing
Contributions are welcome! Please fork this repository and submit a pull request for any improvements.

## 📫 Contact
For questions, suggestions, or collaborations, contact:
- **GitHub**: [@williamsryan](https://github.com/williamsryan)
- **Email**: ryan.williams@assertionlabs.com
