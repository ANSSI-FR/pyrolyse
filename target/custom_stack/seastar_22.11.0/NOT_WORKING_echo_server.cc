#include <seastar/core/app-template.hh>
#include <seastar/util/log.hh>
#include <iostream>
#include <stdexcept>
#include <seastar/core/seastar.hh>
#include <seastar/core/reactor.hh>
#include <seastar/core/future-util.hh>
#include <seastar/net/api.hh>
#include <seastar/net/inet_address.hh>
#include <seastar/net/virtio.hh>
#include <seastar/net/native-stack.hh>
#include <seastar/core/aligned_buffer.hh>
#include <seastar/net/arp.hh>
#include <seastar/net/ip.hh>
#include <seastar/net/net.hh>

#include <seastar/net/arp.hh>
#include <seastar/core/alien.hh>


using namespace seastar;
using namespace net;


seastar::future<> handle_connection(seastar::connected_socket s) {
    auto out = s.output();
    auto in = s.input();
    return do_with(std::move(s), std::move(out), std::move(in),
            [] (auto& s, auto& out, auto& in) {
        return seastar::repeat([&out, &in] {
            return in.read().then([&out] (auto buf) {
                if (buf) {
                    return out.write(std::move(buf)).then([&out] {
                        return out.flush();
                    }).then([] {
                        return seastar::stop_iteration::no;
                    });
                } else {
                    return seastar::make_ready_future<seastar::stop_iteration>(
                            seastar::stop_iteration::yes);
                }
            });
        }).then([&out] {
            return out.close();
        });
    });
}

seastar::future<> service_loop_3() {
    seastar::listen_options lo;
    lo.reuse_address = true;
    seastar::sstring ip_addr_s("192.168.57.33");
    uint16_t port_i(7);
    seastar::net::inet_address ip_addr(ip_addr_s);
    seastar::socket_address s_addr(ip_addr,port_i);
    return seastar::do_with(seastar::listen(s_addr, lo),
            [] (auto& listener) {
        return seastar::keep_doing([&listener] () {
            return listener.accept().then(
                    [] (seastar::accept_result res) {
                // Note we ignore, not return, the future returned by
                // handle_connection(), so we do not wait for one
                // connection to be handled before accepting the next one.
                (void)handle_connection(std::move(res.connection)).handle_exception(
                        [] (std::exception_ptr ep) {
                    fmt::print(stderr, "Could not handle connection: {}\n", ep);
                });
            });
        });
    });
}

int main(int argc, char** argv) {
   
    seastar::app_template app;
    try {
        app.run(argc, argv, service_loop_3);
    } catch(...) {
        std::cerr << "Couldn't start application: "
                  << std::current_exception() << "\n";
        return 1;
    }
    return 0;
}