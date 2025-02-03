#include "market_data/market_data_handler.h"
#include <boost/asio/io_context.hpp>
#include <boost/asio/steady_timer.hpp>
#include <boost/asio/strand.hpp>

MarketDataHandler::MarketDataHandler()
    : io_context(), ws_client(), timer(io_context) {

  ws_client.init_asio(); // Use no arguments for compatibility

  ws_client.set_message_handler(std::bind(&MarketDataHandler::onMessage, this,
                                          std::placeholders::_1,
                                          std::placeholders::_2));

  // Ensure strand is properly initialized
  strand = std::make_shared<
      boost::asio::strand<boost::asio::io_context::executor_type>>(
      io_context.get_executor());
}

void MarketDataHandler::connect(const std::string &uri) {
  std::cout << "[WebSocket] Connecting to: " << uri << std::endl;

  websocketpp::lib::error_code ec;
  auto con = ws_client.get_connection(uri, ec);

  if (ec) {
    std::cerr << "[WebSocket] Connection Error: " << ec.message() << std::endl;
    return;
  }

  con->set_open_handler([this](websocketpp::connection_hdl hdl) {
    std::cout << "[WebSocket] Connection Established!" << std::endl;

    // Send a subscription request if required by the exchange
    // std::string subscribe_message = R"({
    //     "method": "SUBSCRIBE",
    //     "params": ["btcusdt@aggTrade"],
    //     "id": 1
    // })";
    // ws_client.send(hdl, subscribe_message, websocketpp::frame::opcode::text);
  });

  con->set_close_handler([this](websocketpp::connection_hdl hdl) {
    std::cerr << "[WebSocket] Connection Closed!" << std::endl;
  });

  con->set_fail_handler([this](websocketpp::connection_hdl hdl) {
    std::cerr << "[WebSocket] Connection Failed!" << std::endl;
  });

  ws_client.connect(con);
  exchange_url = uri;
}

void MarketDataHandler::onMessage(
    websocketpp::connection_hdl hdl,
    websocketpp::client<websocketpp::config::asio_client>::message_ptr msg) {

  std::cout << "[WebSocket] Received Message: " << msg->get_payload()
            << std::endl;
}

void MarketDataHandler::run() {
  std::cout << "Starting WebSocket event loop..." << std::endl;

  std::thread io_thread([this]() {
    std::cout << "Thread running io_context.run()" << std::endl;
    io_context.run();
    std::cout << "io_context.run() has exited!" << std::endl;
  });

  io_thread.detach(); // Detach so it runs independently
}
