use std::io;
use std::str;
use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpStream;

#[allow(unused_variables)]
pub trait Events {
    fn on_connect(&self, host: &str, port: i32) {}
    fn on_error(&self, err: &str) {}
    fn on_read(&self, resp: &[u8]) {}
    fn on_shutdown(&self) {}
    fn on_pre_read(&self) {}
    fn on_post_read(&self) {}
}

struct Logger;

impl Events for Logger {
    fn on_connect(&self, host: &str, port: i32) {
        println!("Connected to {} on port {}", host, port);
    }

    fn on_error(&self, err: &str) {
        println!("error: {}", err);
    }

    fn on_read(&self, resp: &[u8]) {
        print!("{}", str::from_utf8(resp).unwrap());
    }

    fn on_shutdown(&self) {
        println!("Connection closed.");
    }

    fn on_pre_read(&self) {
        println!("Receiving content:\n");
    }

    fn on_post_read(&self) {
        println!("\nFinished receiving content.")
    }
}

struct HttpClient {
    host: String,
    port: i32,
    hooks: Vec<Box<Events>>,
}

impl HttpClient {
    pub fn new(host: &str, port: i32) -> Self {
        Self {
            host: host.to_owned(),
            port: port,
            hooks: Vec::new(),
        }
    }

    pub fn add_events_hook<E: Events + 'static>(&mut self, hook: E) {
        self.hooks.push(Box::new(hook));
    }

    pub fn get(&self, endpoint: &str) {
        let cmd = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", endpoint, self.host).into_bytes();
        let mut socket = self.connect().unwrap();
        socket.write(cmd.as_slice()).unwrap();
        socket.flush().unwrap();
        for hook in &self.hooks {
            hook.on_pre_read();
        }
        loop {
            let mut buf = vec![0; 512usize];
            let cnt = socket.read(&mut buf[..]).unwrap();
            buf.truncate(cnt);
            if !buf.is_empty() {
                for hook in &self.hooks {
                    hook.on_read(buf.as_slice());
                }
            } else {
                for hook in &self.hooks {
                    hook.on_post_read();
                }
                break;
            }
        }
        for hook in &self.hooks {
            hook.on_shutdown();
        }

    }

    pub fn connect(&self) -> io::Result<TcpStream>{
        let addr = format!("{}:{}", self.host, self.port);
        match TcpStream::connect(addr) {
            Ok(stream) => {
                for hook in &self.hooks {
                    hook.on_connect(&self.host, self.port);
                }
                Ok(stream)
            },
            Err(error) => {
                for hook in &self.hooks {
                    hook.on_error(error.description());
                }
                Err(error)
            },
        }

    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
