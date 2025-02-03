#include "market_data/market_data_handler.h"
#include <chrono>
#include <cstring>
#include <iostream>
#include <thread>

void print_help() {
  std::cout << "Usage: OptiTrade [options]\n\n"
            << "Options:\n"
            << "  --url <ws_url>      Specify WebSocket URL (default: "
               "ws://fstream.binance.com)\n"
            << "  --help              Show this help message\n"
            << std::endl;
}

int main(int argc, char *argv[]) {
  std::string websocket_url = "ws://fstream.binance.com"; // Default URL

  // Parse command-line arguments
  for (int i = 1; i < argc; ++i) {
    if (std::strcmp(argv[i], "--help") == 0 ||
        std::strcmp(argv[i], "-h") == 0) {
      print_help();
      return 0;
    } else if (std::strcmp(argv[i], "--url") == 0 && i + 1 < argc) {
      websocket_url = argv[i + 1];
      ++i; // Skip next argument
    } else {
      std::cerr << "Unknown option: " << argv[i]
                << "\nUse --help for usage information." << std::endl;
      return 1;
    }
  }

  // Display startup message
  std::cout << "Starting OptiTrade Market Data Handler...\n";
//   std::cout << "Connecting to WebSocket: " << websocket_url << std::endl;

  // Initialize and run the market data handler
  MarketDataHandler marketData;
  marketData.connect(websocket_url);

  std::thread data_thread(&MarketDataHandler::run, &marketData);
  data_thread.detach(); // Allow WebSocket to run independently

  // Keep the main thread alive indefinitely
  while (true) {
    std::this_thread::sleep_for(std::chrono::seconds(10));
  }

  return 0;
}
