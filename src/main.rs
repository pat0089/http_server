mod html_builder;
mod http_builder;
mod server;

use server::server::Server;
use server::routes::Route;
use server::directories::Directory;
use server::responses::ok::{ respond_ok_abxy, respond_ok_id, respond_ok_memes, respond_ok };
use server::responses::experimental::{ respond_ok_barcode, respond_ok_webgl };
use http_builder::HttpMethod::GET;

fn main() {
    let server = Server::new(vec![
        Route::new("/", GET, respond_ok),
        Route::new("/memes", GET, respond_ok_memes),
        Route::new("/yourid/:id", GET, respond_ok_id),
        Route::new("/a/:b/x/:y", GET, respond_ok_abxy),
        Route::new("/webgl", GET, respond_ok_webgl),
        Route::new("/barcode/:data", GET, respond_ok_barcode),
    ],
    vec![
        Directory::new("/", false),
        Directory::new("/src/", true),
    ]);
    server.run();
}
