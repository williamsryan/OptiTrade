#ifndef MARKET_DATA_HANDLER_H
#define MARKET_DATA_HANDLER_H

#include <boost/asio/io_context.hpp>
#include <boost/asio/steady_timer.hpp>
#include <boost/asio/strand.hpp>
#include <iostream>
#include <memory>
#include <string>
#include <websocketpp/client.hpp>
#include <websocketpp/config/asio_client.hpp>

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
  boost::asio::io_context io_context;
  boost::asio::steady_timer timer;
  std::shared_ptr<boost::asio::strand<boost::asio::io_context::executor_type>>
      strand;
  std::string exchange_url;
};

#endif // MARKET_DATA_HANDLER_H
