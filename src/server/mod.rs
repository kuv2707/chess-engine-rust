use std::io;
mod models;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

use crate::engine::{self, move_as_string};

use self::models::BestMove;

#[derive(Deserialize)]
pub struct Fen{
    fen: String
}

async fn find_best_move(fen: web::Json<Fen>) -> impl Responder {
    let mut board =
        engine::board::create_board(&fen.fen);
    let (score, best_move) = board.best_move(4);
    HttpResponse::Ok().json(BestMove {
        score,
        best_move: move_as_string(&best_move),
    })
}
#[actix_rt::main]
pub async fn main() -> io::Result<()> {
    println!("Server is ready");
    HttpServer::new(|| App::new().route("/", web::post().to(find_best_move)))
        .bind("127.0.0.1:8000")?
        .run()
        .await?;
    Ok(())
}
