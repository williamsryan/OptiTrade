#ifndef ORDER_BOOK_H
#define ORDER_BOOK_H

#include <map>
#include <mutex>
#include <vector>

class OrderBook {
public:
  void updateOrderBook(const std::string &side, double price, double size);
  void printOrderBook();

private:
  std::map<double, double> bids; // Price -> Size
  std::map<double, double> asks;
  std::mutex book_mutex;
};

#endif // ORDER_BOOK_H
