#ifndef SERVER_HPP
#define SERVER_HPP

#include <boost/asio.hpp>

using boost::asio::ip::tcp;

class Server {
 public:
	Server(short port);

	auto run_context() -> void;
	auto accept_connection() -> void;
	auto read_client() -> void;

 private:
	boost::asio::io_context context;
	tcp::acceptor acceptor;
	tcp::socket socket;
};

#endif //SERVER_HPP
