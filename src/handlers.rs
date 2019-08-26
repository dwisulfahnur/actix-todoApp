use actix_web::{web, Error, HttpResponse, HttpRequest, FromRequest, ResponseError, error::BlockingError};
use futures::Future;

use crate::diesel::prelude::*;
use crate::models::{Todo, TodoForm, query};
use crate::schema;
use crate::validator::{Validate, ValidationErrors};
use crate::diesel::MysqlConnection;
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
        let conn: &MysqlConnection = &_pool.get().unwrap();
        schema::todos::dsl::todos.load::<Todo>(conn)
    }).then(|res| match res {
        Ok(_posts) => Ok(HttpResponse::Ok().json(_posts)),
        Err(_err) => Err(ServiceError::InternalServerError),
    })
}


pub fn get_todo(path: web::Path<TodoDetailPath>, _pool: web::Data<Pool>) -> impl Future<Item=HttpResponse, Error=ServiceError> {
    web::block(move || {
        let conn: &MysqlConnection = &_pool.get().unwrap();
        schema::todos::dsl::todos.find(path.todo_id).first::<Todo>(conn)
    }).then(|res| {
        match res {
            Ok(_todo) => Ok(HttpResponse::Ok().json(_todo)),
            Err(_e) => Err(ServiceError::NotFound("Todo Not Found".to_string()))
        }
    })
}


pub fn create_todo(_new_todo: web::Json<TodoForm>, _pool: web::Data<Pool>) -> impl Future<Item=HttpResponse, Error=ServiceError> {
    web::block(move || {
        query::create_todo(_new_todo.into_inner(), _pool)
    }).then(|result| match result {
        Ok(_todo) => Ok(HttpResponse::Created().json(_todo)),
        Err(_err) => match _err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        }
    })
}


pub fn remove_todo(_path: web::Path<TodoDetailPath>, _pool: web::Data<Pool>) -> impl Future<Item=HttpResponse, Error=ServiceError> {
    web::block(move || {
        let conn: &MysqlConnection = &_pool.get().unwrap();
        diesel::delete(schema::todos::dsl::todos.find(_path.todo_id)).execute(conn)
    }).then(|res| {
        match res {
            Ok(_) => Ok(HttpResponse::NoContent().finish()),
            Err(_e) => Err(ServiceError::InternalServerError)
        }
    })
}

pub fn update_todo(_path: web::Path<TodoDetailPath>, _todo: web::Json<TodoForm>, _pool: web::Data<Pool>) -> impl Future<Item=HttpResponse, Error=ServiceError> {
    web::block(move || {
        query::update_todo(_path.todo_id, _todo.into_inner(), _pool)
    }).then(|result| match result {
        Ok(_todo) => Ok(HttpResponse::Ok().json(_todo)),
        Err(_err) => match _err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError)
        }
    })
}
