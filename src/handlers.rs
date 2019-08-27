use actix_web::{web, Error, HttpResponse, HttpRequest, FromRequest, ResponseError, error::BlockingError};
use futures::{future::{ok as fut_ok, err as fut_err}, Future};

use crate::diesel::prelude::*;
use crate::models::{Todo, TodoForm, query};
use crate::schema;
use crate::validator::{Validate, ValidationErrors};
use crate::diesel::{MysqlConnection, result::Error as DBError};
use crate::serde_derive::Deserialize;
use crate::serde_json;
use crate::db::Pool;
use crate::errors::ServiceError;


#[derive(Deserialize)]
pub struct TodoDetailPath {
    todo_id: i32,
}

pub fn get_todos(_pool: web::Data<Pool>) -> impl Future<Item=HttpResponse, Error=ServiceError> {
    web::block(move || {
        query::select_all(_pool)
    }).then(|res| match res {
        Ok(_posts) => fut_ok(HttpResponse::Ok().json(_posts)),
        Err(_err) => fut_ok(ServiceError::InternalServerError.error_response()),
    })
}


pub fn get_todo(path: web::Path<TodoDetailPath>, _pool: web::Data<Pool>) -> impl Future<Item=HttpResponse, Error=ServiceError> {
    web::block(move || {
        let conn: &MysqlConnection = &_pool.get().unwrap();
        schema::todos::dsl::todos.find(path.todo_id).first::<Todo>(conn)
    }).then(|res| {
        match res {
            Ok(_todo) => fut_ok(HttpResponse::Ok().json(_todo)),
            Err(_e) => fut_ok(ServiceError::NotFound("Todo Not Found".to_string()).error_response())
        }
    })
}


pub fn create_todo(_new_todo: web::Json<TodoForm>, _pool: web::Data<Pool>) -> impl Future<Item=HttpResponse, Error=ServiceError> {
    web::block(move || {
        query::create_todo(_new_todo.into_inner(), _pool)
    }).then(|result| match result {
        Ok(_todo) => fut_ok(HttpResponse::Created().json(_todo)),
        Err(_err) => match _err {
            BlockingError::Error(service_error) => fut_ok(service_error.error_response()),
            BlockingError::Canceled => fut_ok(ServiceError::InternalServerError.error_response()),
        }
    })
}

pub fn remove_todo(_path: web::Path<TodoDetailPath>, _pool: web::Data<Pool>) -> impl Future<Item=HttpResponse, Error=ServiceError> {
    web::block(move || {
        query::remove_todo(_path.todo_id, _pool)
    }).then(|res| {
        match res {
            Ok(_) => fut_ok(HttpResponse::NoContent().finish()),
            Err(_error) => fut_ok(_error.error_response())
        }
    })
}

pub fn update_todo(_path: web::Path<TodoDetailPath>, _todo: web::Json<TodoForm>, _pool: web::Data<Pool>) -> impl Future<Item=HttpResponse, Error=ServiceError> {
    web::block(move || {
        query::update_todo(_path.todo_id, _todo.into_inner(), _pool)
    }).then(|result| match result {
        Ok(_todo) => fut_ok(HttpResponse::Ok().json(_todo)),
        Err(_err) => match _err {
            BlockingError::Error(service_error) => fut_ok(service_error.error_response()),
            BlockingError::Canceled => fut_ok(ServiceError::InternalServerError.error_response())
        }
    })
}
