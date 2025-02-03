#ifndef MARKET_DATA_HANDLER_H
#define MARKET_DATA_HANDLER_H

#include <iostream>
#include <string>
#include <thread>
#include <websocketpp/client.hpp>
#include <websocketpp/config/asio_client.hpp>

using websocketpp::connection_hdl;

class MarketDataHandler {
public:
  MarketDataHandler();
  void connect(const std::string &uri);
  void onMessage(
      websocketpp::connection_hdl hdl,
      websocketpp::client<websocketpp::config::asio_client>::message_ptr msg);
  void run();

private:
  websocketpp::client<websocketpp::config::asio_client> ws_client;
  std::string exchange_url;
};

#endif // MARKET_DATA_HANDLER_H
