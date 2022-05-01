use std::io;
use std::net::{Ipv6Addr, SocketAddr, ToSocketAddrs, UdpSocket};
use std::time::Duration;

pub enum ESB{

}

impl ESB {
    //For now this is just a wrapper around some functions from https://doc.rust-lang.org/stable/std/net/struct.UdpSocket.html
    // pub fn bind<A: ToSocketAddrs>(addr: A) -> Result<UdpSocket> {
    //     //UdpSocket::bind("127.0.0.1:3400").expect("couldn't bind to address");
    // }

    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<UdpSocket> {
        return UdpSocket::bind(A);
    }

    pub fn recv_from( socket: UdpSocket, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        return UdpSocket::recv_from(&socket, buf);
    }

    pub fn join_multicast_v6(socket: UdpSocket, multiaddr: &Ipv6Addr, interface: u32) -> io::Result<()> {
        return UdpSocket::join_multicast_v6(&socket, multiaddr, interface)
    }

    pub fn leave_multicast_v6(socket: UdpSocket, multiaddr: &Ipv6Addr, interface: u32) -> io::Result<()> {
        return UdpSocket::leave_multicast_v6(&socket, multiaddr, interface);
    }

    pub fn send_to<A: ToSocketAddrs>(socket: UdpSocket, buf: &[u8], addr: A) -> io::Result<usize> {
        return UdpSocket::send_to(&socket, buf, addr);
    }

    pub fn set_multicast_loop_v6(socket: UdpSocket, multicast_loop_v6: bool) -> io::Result<()> {
        return UdpSocket::set_multicast_loop_v6(&socket, multicast_loop_v6);
    }

    pub fn multicast_loop_v6(socket: UdpSocket) -> io::Result<bool> {
        return UdpSocket::multicast_loop_v6(&socket);
    }

    pub fn set_read_timeout(socket: UdpSocket, dur: Option<Duration>) -> io::Result<()> {
        return UdpSocket::set_read_timeout(&socket, dur);
    }
}