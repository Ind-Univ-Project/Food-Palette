mod error;
mod image_analyzer;
mod pixel_data;
mod rgb_ext;

use error::Error;
use image_analyzer::ImageAnalyzer;

use tide::log::info;
use tide::prelude::*;
use tide::Request;

#[async_std::main]
async fn main() -> tide::Result<()> {
    println!("Starting Server");
    let mut app = tide::new();
    app.at("").get(index);
    app.at("/upload_image").post(upload_image);
    //app.at("get images(color, preference as filter)")
    //app.at("")

    app.listen("0.0.0.0:8089").await?;

    Ok(())
}

async fn index(mut req: Request<()>) -> tide::Result {
    Ok("Hello World".into())
}

async fn get_image_list(req: Request<()>) -> tide::Result {
    //get color and preference
    //query to db with color and preference
    //get images
    //response images
    todo!()
}

async fn upload_image(mut req: Request<()>) -> tide::Result {
    info!(
        "upload_image request Peer: {}",
        req.peer_addr().ok_or(Error::PeerAddressError)?
    );

    //get body with multipart
    //parse image category, image format, image data
    //get pixel data

    todo!()
}
