#include "market_data/order_book.h"

void OrderBook::updateOrderBook(const std::string &side, double price,
                                double size) {
  std::lock_guard<std::mutex> lock(book_mutex);

  if (side == "buy") {
    if (size > 0) {
      bids[price] = size;
    } else {
      bids.erase(price);
    }
  } else if (side == "sell") {
    if (size > 0) {
      asks[price] = size;
    } else {
      asks.erase(price);
    }
  }
}

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
