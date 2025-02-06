#!/usr/bin/env bash

# Navigate to your project root
cd ~/git/OptiTrade

# Create core backend services as Rust binary crates
cargo new backend --lib  # Main backend library (optional if shared utilities exist)
cargo new backend/market_data --bin
cargo new backend/storage_agent --bin
cargo new backend/backtesting --bin
cargo new backend/execution_agent --bin
cargo new backend/analytics --bin

# Create the frontend for Rust/WASM UI
cargo new frontend --bin

# Create infrastructure folder for messaging and deployment configs
mkdir -p infra/messaging
mkdir -p infra/deployment/k8s-configs
mkdir -p infra/deployment/monitoring

# Move into backend and create agent-specific submodules
cd backend

# Create additional Rust modules inside each agent (these won't conflict with cargo new)
mkdir -p market_data/src
touch market_data/src/{websocket.rs,kafka_producer.rs,data_parser.rs}

mkdir -p storage_agent/src
touch storage_agent/src/{kafka_consumer.rs,db_writer.rs}
touch storage_agent/init_db.sql  # SQL schema for TimescaleDB

mkdir -p backtesting/src
touch backtesting/src/{strategy.rs,data_loader.rs,risk_management.rs}

mkdir -p execution_agent/src
touch execution_agent/src/{kafka_consumer.rs,order_executor.rs,risk_checker.rs}

mkdir -p analytics/src
touch analytics/src/{dashboard.rs,indicators.rs}

# Move to frontend and create UI component modules
cd ../frontend
mkdir -p src/components
touch src/components/{market_view.rs,trade_panel.rs,analytics_view.rs}

# Return to project root
cd ..
