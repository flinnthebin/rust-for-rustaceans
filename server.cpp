#include <boost/asio.hpp>

using boost::asio::ip::tcp;

class Server {
public:
    Server()
        : socket(context)
    {}

private:
    boost::asio::io_context context;
    tcp::socket socket;
};
