#include "market_data/market_data_handler.h"

MarketDataHandler::MarketDataHandler() {
  ws_client.init_asio();
  ws_client.set_message_handler(bind(&MarketDataHandler::onMessage, this,
                                     std::placeholders::_1,
                                     std::placeholders::_2));
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

void MarketDataHandler::run() { ws_client.run(); }
