use crate::structs::core::{Request, Response};



pub trait Plugin {
    fn plugin_name(&self) -> &str;
    fn plugin_init(&self) {}
    fn plugin_match(&self, req: &Request) -> bool;
    fn plugin_serve(&self, req: &Request) -> Response;
}
