#include "market_data/market_data_handler.h"
#include <iostream>

int main() {
  std::cout << "Starting OptiTrade Market Data Handler..." << std::endl;

  MarketDataHandler marketData;
  marketData.connect(
      "wss://www.deribit.com/ws/api/v2"); // Deribit WebSocket API

  std::thread data_thread(&MarketDataHandler::run, &marketData);
  data_thread.join(); // Run market data handler

  return 0;
}
