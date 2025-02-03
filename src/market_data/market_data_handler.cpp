#include "market_data/market_data_handler.h"
#include <boost/asio/io_context.hpp>
#include <boost/asio/steady_timer.hpp>
#include <boost/asio/strand.hpp>
#include <thread>

MarketDataHandler::MarketDataHandler()
    : io_context(), ws_client(), timer(io_context) {

  ws_client.init_asio(); // Use no arguments for compatibility

  ws_client.set_message_handler(std::bind(&MarketDataHandler::onMessage, this,
                                          std::placeholders::_1,
                                          std::placeholders::_2));

  // ✅ Ensure strand is properly initialized
  strand = std::make_shared<
      boost::asio::strand<boost::asio::io_context::executor_type>>(
      io_context.get_executor());
}

void MarketDataHandler::connect(const std::string &uri) {
  websocketpp::lib::error_code ec;
  auto con = ws_client.get_connection(uri, ec);

  if (ec) {
    std::cerr << "WebSocket Error: " << ec.message() << std::endl;
    return;
  }

  ws_client.connect(con);
  exchange_url = uri; 
}

void MarketDataHandler::onMessage(
    websocketpp::connection_hdl hdl,
    websocketpp::client<websocketpp::config::asio_client>::message_ptr msg) {
  std::cout << "Market Data Received: " << msg->get_payload() << std::endl;
}

void MarketDataHandler::run() {
  std::thread io_thread([this]() { io_context.run(); });
  io_thread.detach();
}
