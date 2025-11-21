use crate::{structs::plugin::Plugin, structs::core::{Request, Response}};

pub struct PluginHelloWorld;

impl Plugin for PluginHelloWorld {
    fn plugin_name(&self) -> &str {
        "HelloWorld"
    }

    fn plugin_init(&self) {
        println!("HelloWorld from a plugin.");
    }

    fn plugin_match(&self, req: &Request) -> bool {
        req.method == "GET"
    }

    fn plugin_serve(&self, req: &Request) -> Response {
        println!("HelloWorld plugin serving a request for the request:");
        println!("{}", req.raw);
        println!("{}", req.path);

        Response {
            body: String::from(
                "HTTP/1.1 200 OK\r\n\
Content-Type: text/html; charset=utf-8\r\n\
\r\n\
<!DOCTYPE html>\
<html><body><h1>Hello World Plugin</h1></body></html>",
            ),
        }
    }
}
