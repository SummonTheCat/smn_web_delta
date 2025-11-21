mod structs;
mod server;
mod plugins;

use server::Server;
use crate::plugins::{
    plugin_helloworld::PluginHelloWorld,
    plugin_static_files::PluginStaticFile
};
use crate::structs::plugin::Plugin;

fn main() {
    let plugins: Vec<Box<dyn Plugin>> = vec![
        Box::new(PluginStaticFile),
        Box::new(PluginHelloWorld),
    ];

    let server = Server::new("33030", plugins, "./static");

    server.run();
}
