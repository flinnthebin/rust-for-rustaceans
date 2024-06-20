#include "server.hpp"
#include <iostream>

Server::Server(short const port)
    : acceptor (context, tcp::endpoint(tcp::v4(), port)), socket(context) {
    accept_connection();
}

auto Server::run_context() -> void {
    context.run();
}

auto Server::accept_connection() -> void {
    acceptor.async_accept(socket, [this](boost::system::error_code error) {
        if (!error) {
            std::cout << "Client connection successful" << std::endl;
            read_client();
        }
        accept_connection();
    });
}

auto Server::read_client() -> void {
    auto buffer = std::make_shared<std::array<char, 1024>>();
    socket.async_read_some(boost::asio::buffer(*buffer), [this, buffer](boost::system::error_code error, std::size_t len) {
        if (!error) {
            std::cout << "Client Message:" << std::string(buffer->data(), len) << std::endl;
            read_client();
        } else {
            std::cerr << "Error in method 'read_client':" << error.message() << std::endl;
            
        }
    });
};
