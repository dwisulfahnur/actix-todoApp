use actix_web::web;

use crate::handlers::{get_todos, get_todo, remove_todo, update_todo, create_todo};

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todos")
            .service(
                web::resource("")
                    .route(web::get().to_async(get_todos))
                    .route(web::post().to_async(create_todo)),
            )
            .service(
                web::scope("/{todo_id}")
                    .service(
                        web::resource("")
                            .route(web::get().to_async(get_todo))
                            .route(web::delete().to_async(remove_todo))
                            .route(web::put().to_async(update_todo))
                    )
            ),
    );
}