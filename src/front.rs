use actix_files::NamedFile;
use actix_web::{get, Error, HttpRequest};

#[get("/")]
pub async fn main_page(_req: HttpRequest) -> Result<NamedFile, Error>{
    Ok(NamedFile::open("./static/index.html")?)
}