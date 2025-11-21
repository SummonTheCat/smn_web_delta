use crate::structs::file_cache::FileCache;
use crate::structs::core::Request;
use crate::structs::plugin::Plugin;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub struct Server {
    port: String,
    plugins: Vec<Box<dyn Plugin>>,
    file_root: String,
}

impl Server {
    pub fn new(port: &str, plugins: Vec<Box<dyn Plugin>>, file_root: &str) -> Self {
        Self {
            port: port.to_string(),
            plugins,
            file_root: file_root.to_string(),
        }
    }

    pub fn run(&self) {
        println!("Initializing file cache...");
        FileCache::init(&self.file_root);

        let bind_addr = format!("127.0.0.1:{}", self.port);
        let listener = TcpListener::bind(&bind_addr)
            .unwrap_or_else(|_| panic!("Failed to bind to {}", bind_addr));

        println!("Serving on http://{}", bind_addr);

        for plugin in &self.plugins {
            plugin.plugin_init();
            println!("Loaded plugin: {}", plugin.plugin_name());
        }

        for incoming in listener.incoming() {
            if let Ok(mut stream) = incoming {
                self.handle_client(&mut stream);
            }
        }
    }

    fn handle_client(&self, stream: &mut TcpStream) {
        let mut buffer = [0_u8; 1024];
        let _ = stream.read(&mut buffer);

        let raw = String::from_utf8_lossy(&buffer).to_string();
        let req = Request::new(raw);

        for plugin in &self.plugins {
            if plugin.plugin_match(&req) {
                let resp = plugin.plugin_serve(&req);
                let _ = stream.write_all(resp.body.as_bytes());
                return;
            }
        }

        let _ = stream.write_all(b"HTTP/1.1 404 NOT FOUND\r\n\r\n");
    }
}
