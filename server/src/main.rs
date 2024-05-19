use std::net::TcpListener;

fn conncetion_listener() {

    let addr = "127.0.0.1:80";
    let listener = match TcpListener::bind(addr) {
        Ok(bound_listener) => {
            println!("Listener bound to addr {}", addr);
            bound_listener
        },
        Err(_) => todo!(),
    };

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("A client connected");
            },
            Err(_) => {
                println!("A client failed to connect");
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    conncetion_listener()
}
