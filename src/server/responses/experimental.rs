use std::collections::HashMap;
use std::net::TcpStream;
use std::io;

use crate::html_builder::Page;
use crate::server::util::mime_types::MimeType;
use crate::server::util::barcode_encoding::encode_to_barcode_bitstring;
use crate::server::util::images::bitmap::Bitmap;

use super::ok::{respond_ok_with_body_and_type, respond_ok_with_body_bytes_and_type};

pub fn respond_redirect(stream: &mut TcpStream, url: &str) -> io::Result<()> {
    todo!("implement redirect")
}

pub fn respond_ok_webgl(stream: &mut TcpStream, params: HashMap<String, String>) -> io::Result<()> {
    
    let mut response  = Page::new();
    let title = "WebGL HTTP Server Demo Page";
    response.add_title(title);
    response.add_heading(1, title);
    response.add_canvas(800, 600, None, true);

    respond_ok_with_body_and_type(stream, response.to_string().as_str(), MimeType::Html)
}

pub fn respond_ok_barcode(stream: &mut TcpStream, params: HashMap<String, String>) -> io::Result<()> {
    let to_encode = params.get("data").unwrap();
    let bitstring = encode_to_barcode_bitstring(to_encode);

    let mut content = Vec::new();
    for _ in 0..20 {
        content.push(bitstring.clone());
    }
    let content: Vec<&bool> = content.iter().flatten().collect();

    let pixels = content.iter().map(|b| if **b { [0, 0, 0] } else { [255, 255, 255] }).collect::<Vec<[u8; 3]>>();

    let response = Bitmap::new(pixels.iter().flat_map(|p| {
        vec![p[0], p[1], p[2]]
    }).collect::<Vec<u8>>(),(bitstring.len() as u32, 20), 24);
    respond_ok_with_body_bytes_and_type(stream, &response.write_bitmap(), MimeType::Bitmap)
}
