use crate::structs::core::{Request, Response};
use crate::structs::file_cache::FileCache;
use crate::structs::plugin::Plugin;

pub struct PluginStaticFile;

impl Plugin for PluginStaticFile {
    fn plugin_name(&self) -> &str {
        "StaticFile"
    }

    fn plugin_match(&self, req: &Request) -> bool {
        let path = if req.path == "/" {
            "/index.html"
        } else {
            req.path.as_str()
        };
        FileCache::get(path).is_some()
    }

    fn plugin_serve(&self, req: &Request) -> Response {
        let path = if req.path == "/" {
            "/index.html"
        } else {
            req.path.as_str()
        };

        let file = FileCache::get(path).unwrap_or("<h1>File not found</h1>");

        Response {
            body: format!(
                "HTTP/1.1 200 OK\r\n\
Content-Type: text/html; charset=utf-8\r\n\
\r\n\
{}",
                file
            ),
        }
    }
}
