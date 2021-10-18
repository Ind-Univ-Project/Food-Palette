mod error;

use tide::log::info;
use tide::prelude::*;
use tide::{Request, StatusCode};

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
    let body = req.body_bytes().await.unwrap();
    let image_type = req
        .content_type()
        .ok_or(tide::Error::from_str(
            StatusCode::BadRequest,
            "Getting Content-Type failed",
        ))?
        .subtype();
    
    //calculate each pixels
    //save image to filesystem
    //save data to database
    todo!()
}
