mod card_deck;
mod front;
use crate::front::main_page;
use actix_cors::Cors;
use actix_web::web;

pub fn card_games_app() -> impl actix_web::dev::HttpServiceFactory {
    web::scope("/card_games")
        .wrap(
            Cors::default()
                .allowed_origin("http://localhost:5173") // allow your frontend dev server
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![http::header::CONTENT_TYPE])
                .max_age(3600),
        )
        .service(main_page)
}