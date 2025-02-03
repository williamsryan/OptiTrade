# OptiTrade - Ultra-Low-Latency Crypto Options Trading System

![Build Status](https://github.com/williamsryan/OptiTrade/actions/workflows/cmake-single-platform.yml/badge.svg)
![C++](https://img.shields.io/badge/language-C++23-blue)
![Rust](https://img.shields.io/badge/migration-Rust-orange)
![License](https://img.shields.io/badge/license-GPLv3-blue)

## 📌 Overview
**OptiTrade** is an **ultra-low-latency automated trading system** for **crypto options**, designed for real-time **market data ingestion, options pricing, execution, and risk management**. It is built for **high-performance algorithmic trading** with an initial focus on **C++23**, with **gradual Rust migration** to improve memory safety and execution efficiency.

## 🚀 Features
✅ **Real-time Market Data Feeds**: WebSocket-based integration with **Deribit, Binance, OKX**  
✅ **Ultra-Fast Execution Engine**: Sub-millisecond order placement using **kernel-bypass networking**  
✅ **Advanced Options Pricing**: Black-Scholes, Heston, and SABR models for **IV estimation**  
✅ **Algorithmic Strategy Execution**: Supports **IV arbitrage, gamma scalping, and market-making**  
✅ **Risk Management & Hedging**: Auto-adjusts **delta/gamma exposure** dynamically  
✅ **Optimized for Performance**: Uses **lock-free queues, NUMA-aware memory allocation, and SIMD processing**  
✅ **Backtesting Framework**: Simulate historical data to refine strategy performance  
✅ **AWS/GCP Deployment Ready**: Optimized for cloud-based trading execution  

## 📂 Project Structure
```
OptiTrade/
│── src/                     # Source code
│   ├── main.cpp              # Main entry point
│   ├── market_data/          # Market data ingestion & processing
│   ├── execution/            # Order execution & routing
│   ├── strategies/           # Trading strategy logic
│   ├── risk_management/      # Position sizing, exposure limits
│   ├── backtesting/          # Historical simulation
│── include/                  # Header files for all components
│── build/                    # Compiled binaries
│── tests/                    # Unit tests
│── logs/                     # Trading logs, execution reports
│── data/                     # Market data storage
│── scripts/                  # Deployment & performance scripts
│── benchmarks/               # Benchmarking tools
│── config.json               # Configuration file for API keys, risk settings
│── .gitignore                # Git ignore file
│── CMakeLists.txt            # CMake build configuration
│── README.md                 # Project documentation
```

## 🔧 Installation & Setup
### **1. Clone the Repository**
```bash
git clone git@github.com:williamsryan/OptiTrade.git
cd OptiTrade
```

### **2. Build the Project (CMake)**
```bash
mkdir build && cd build
cmake ..
make -j$(nproc)
```

### **3. Run the System**
```bash
./OptiTrade
```

## 📜 API Integration
### **Market Data Streaming**
- **Deribit WebSocket API**: `wss://www.deribit.com/ws/api/v2`
- **Binance WebSocket API**: `wss://stream.binance.com:9443/ws`

### **Order Execution (Example Request)**
```cpp
std::string place_order(std::string exchange, std::string instrument, double size, double price) {
    std::stringstream request;
    request << "{\"exchange\": \"" << exchange << "\", \"instrument\": \"" << instrument
            << "\", \"size\": " << size << ", \"price\": " << price << "}";
    return send_http_request("https://api.exchange.com/order", request.str());
}
```

## 🚀 Roadmap
1. **Implement real-time WebSocket market data ingestion** ✅
2. **Develop high-performance order execution logic** ✅
3. **Integrate options pricing models (IV, greeks, vol surfaces)** ✅
4. **Optimize for ultra-low latency performance** (ongoing) 🟡
5. **Implement risk management & hedging strategies** (upcoming) 🔜
6. **Backtesting & simulation integration** (upcoming) 🔜
7. **Deploy optimized trading system to AWS/GCP** (future) 🚀

## 📄 License
This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing
Contributions are welcome! Please fork this repository and submit a pull request for any improvements.

## 📫 Contact
For questions, suggestions, or collaborations, contact:
- **GitHub**: [@williamsryan](https://github.com/williamsryan)
- **Email**: ryan.williams@assertionlabs.com

---
