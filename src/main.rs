mod html_builder;
mod http_builder;
mod server;

use server::server::Server;
use server::routes::Route;
use server::directories::Directory;
use server::responses::respond_ok;

fn main() {
    //TODO: if a route is named a file, attempt to serve the file and return it instead and ignore route function/functionality
    let server = Server::new(vec![
        Route::new("/", "GET", respond_ok),
    ],
    vec![
        Directory::new("/", false),
    ]);
    server.run();
}
