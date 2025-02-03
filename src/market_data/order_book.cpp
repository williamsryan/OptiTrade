#include "market_data/order_book.h"
#include <iostream>

void OrderBook::printOrderBook() {
  std::lock_guard<std::mutex> lock(book_mutex);

  std::cout << "\nOrder Book:" << std::endl;
  std::cout << "Bids:" << std::endl;
  for (const auto &bid : bids) {
    std::cout << "Price: " << bid.first << " | Size: " << bid.second
              << std::endl;
  }

  std::cout << "Asks:" << std::endl;
  for (const auto &ask : asks) {
    std::cout << "Price: " << ask.first << " | Size: " << ask.second
              << std::endl;
  }
}
