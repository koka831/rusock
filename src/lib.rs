extern crate libc;

pub use libc::{
    AF_INET,
    AF_UNIX,
    SOCK_STREAM,
    SOCK_DGRAM,
    SOCK_RAW,

    size_t,
    socket,
    sockaddr,
    sockaddr_in,
    socklen_t,

    bind,
    listen,
    accept,
    connect,
    send,
    recv,
    close,
};

struct Socket {
    fd : i32,
}

impl Drop for Socket {
    fn drop(&mut self) {
        self.close();
    }
}

impl Socket {
    fn new(sock_family: i32, sock_type: i32, addr_family: i32) -> Socket {
        // get fd
        let fd = unsafe { socket(sock_family, sock_type, addr_family) };
        Socket{ fd: fd }
    }

    fn close(&self) -> bool {
        unsafe { close(self.fd) };
        true
    }
}
