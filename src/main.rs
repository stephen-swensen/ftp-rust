mod ftp {
    pub mod server {
        use std::net::TcpListener;
        use std::io::Write;
        pub fn start(host: &str, port: i32) {
            let addr = format!("{}:{}", host, port);
            println!("Binding to address {}", addr);
            let listener = TcpListener::bind(addr).expect("Couldn't bind to address");
            println!("Waiting for clients to connect...");
            for stream in listener.incoming() {
                match stream {
                    Ok(mut stream) => {
                        println!("New client!");
                        if let Err(_) = stream.write(b"hello") {
                            println!("Failed to send hello...");
                        }
                    }
                    _ => {
                        println!("A client tried, but failed, to connect...");
                    }
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    ftp::server::start("0.0.0.0", 1234);
}
