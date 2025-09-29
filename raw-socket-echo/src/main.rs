extern crate libc;

use std::{f32::consts, ffi::CString, mem, os::raw::c_void};

use libc::{
    AF_INET, INADDR_ANY, SOCK_STREAM, accept, bind, close, htons, listen, pause, sa_family_t,
    sleep, sockaddr, sockaddr_in, socket, socklen_t, write,
};

fn main() {
    unsafe {
        let sockfd = socket(AF_INET, SOCK_STREAM, 0);
        if sockfd < 0 {
            println!("Error create socket {}", sockfd);
        }
        let mut addr: sockaddr_in = mem::zeroed();
        addr.sin_port = htons(8080);
        addr.sin_family = AF_INET as sa_family_t;
        addr.sin_addr.s_addr = INADDR_ANY;

        let bind_result = bind(
            sockfd,
            &addr as *const sockaddr_in as *const sockaddr,
            mem::size_of_val(&addr) as socklen_t,
        );
        if bind_result < 0 {
            println!("Error bind {}", bind_result);
        }

        let listen_result = listen(sockfd, 0);
        if listen_result < 0 {
            println!("Error bind {}", listen_result);
        }

        loop {
            let mut client_add: sockaddr_in = mem::zeroed();
            let mut addrlen: socklen_t = mem::size_of::<sockaddr_in>() as socklen_t;
            let client_fd = accept(
                sockfd,
                &mut client_add as *mut sockaddr_in as *mut sockaddr,
                &mut addrlen,
            );
            if client_fd < 0 {
                println!("Error accept {}", client_fd);
            }

            println!("Accepted connection, fd = {}", client_fd);


            let data = CString::new("ABCDEFG").expect("error");

            let result = write(client_fd, data.as_ptr() as *const c_void, 3);
            if result < 0 {
                let err = unsafe { *libc::__error() };
                println!(
                    "write failed, errno={} ({})",
                    err,
                    std::io::Error::from_raw_os_error(err)
                );
            }
        }
    }
}
