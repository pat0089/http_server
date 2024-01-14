mod html_builder;
mod http_builder;
mod server;

use server::server::Server;
use server::routes::Route;
use server::responses::respond_ok;

fn main() {
    let server = Server::new(vec![
        Route::new("/", "GET", respond_ok),
    ]);
    server.run();
}
