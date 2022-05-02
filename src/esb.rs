use std::io;
use std::net::{Ipv4Addr, SocketAddr, ToSocketAddrs, UdpSocket};
use std::time::Duration;

pub enum ESB{

}

impl ESB {
    //For now this is just a wrapper around some functions from https://doc.rust-lang.org/stable/std/net/struct.UdpSocket.html

    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<UdpSocket> {
        return UdpSocket::bind(addr);
    }

    pub fn recv_from( socket: UdpSocket, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        return UdpSocket::recv_from(&socket, buf);
    }

    pub fn join_multicast_v4(socket: UdpSocket, multiaddr: &Ipv4Addr, interface: &Ipv4Addr) -> io::Result<()> {
        return UdpSocket::join_multicast_v4(&socket, multiaddr, interface)
    }

    pub fn leave_multicast_v4(socket: UdpSocket, multiaddr: &Ipv4Addr, interface: &Ipv4Addr) -> io::Result<()> {
        return UdpSocket::leave_multicast_v4(&socket, multiaddr, interface);
    }

    pub fn send_to<A: ToSocketAddrs>(socket: UdpSocket, buf: &[u8], addr: A) -> io::Result<usize> {
        return UdpSocket::send_to(&socket, buf, addr);
    }

    pub fn set_multicast_loop_v4(socket: UdpSocket, multicast_loop_v4: bool) -> io::Result<()> {
        return UdpSocket::set_multicast_loop_v4(&socket, multicast_loop_v4);
    }

    pub fn multicast_loop_v4(socket: UdpSocket) -> io::Result<bool> {
        return UdpSocket::multicast_loop_v4(&socket);
    }

    pub fn set_read_timeout(socket: UdpSocket, dur: Option<Duration>) -> io::Result<()> {
        return UdpSocket::set_read_timeout(&socket, dur);
    }

    pub fn set_multicast_ttl_v4(socket: UdpSocket, multicast_ttl_v4: u32) -> io::Result<()> {
        return UdpSocket::set_multicast_ttl_v4(&socket, multicast_ttl_v4);
    }

    pub fn multicast_ttl_v4(socket: UdpSocket) -> io::Result<u32> {
        return UdpSocket::multicast_ttl_v4(&socket);
    }
}