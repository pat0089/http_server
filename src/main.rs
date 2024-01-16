mod html_builder;
mod http_builder;
mod server;

use server::server::Server;
use server::routes::Route;
use server::directories::Directory;
use server::responses::respond_ok;

fn main() {
    let server = Server::new(vec![
        Route::new("/", "GET", respond_ok),
    ],
    vec![
        Directory::new("/", false),
    ]);
    server.run();
}
