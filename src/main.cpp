#include "market_data/market_data_handler.h"
#include <iostream>

int main() {
  std::cout << "Starting OptiTrade Market Data Handler..." << std::endl;

  MarketDataHandler marketData;
  marketData.connect("ws://fstream.binance.com");
  // (wss://www.deribit.com/ws/api/v1/)

  std::thread data_thread(&MarketDataHandler::run, &marketData);
  data_thread.join(); // Run market data handler

  return 0;
}
