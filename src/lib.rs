mod card_deck;
mod front;
use crate::front::main_page;
use actix_cors::Cors;
use actix_web::web;
use actix_web::http::header;

pub fn card_games_app() -> impl actix_web::dev::HttpServiceFactory {
    let frontend_origin = std::env::var("FRONTEND_ORIGIN")
        .unwrap_or_else(|_| "http://localhost:5173".into());
    web::scope("/card_games")
        .wrap(
            Cors::default()
                .allowed_origin(&frontend_origin) // allow your frontend dev server
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![header::CONTENT_TYPE])
                .max_age(3600),
        )
        .service(main_page)
}