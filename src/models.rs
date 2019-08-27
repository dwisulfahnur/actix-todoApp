use crate::serde_derive::{Deserialize, Serialize};
use crate::validator::{Validate, ValidationError};
use crate::schema::todos;
use crate::diesel::prelude::*;


#[derive(Debug, PartialEq, Deserialize, Serialize, Queryable)]
pub struct Todo {
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Validate, Deserialize, Serialize, Insertable)]
#[table_name = "todos"]
pub struct TodoForm {
    #[validate(length(min = 1))]
    pub title: String,
}

pub mod query {
    use crate::errors::ServiceError;
    use crate::models::{Todo, TodoForm};
    use crate::db::Pool;
    use crate::schema::todos::{table as todos_table, dsl::{todos, id, title}};
    use actix_web::web;
    use diesel::{MysqlConnection, RunQueryDsl, prelude::*};
    use diesel::result::{Error as DBError, DatabaseErrorKind};
    use actix_web::error::DispatchError::Service;

    pub fn select_all(_pool: web::Data<Pool>) -> Result<Vec<Todo>, ServiceError> {
        let conn: &MysqlConnection = &_pool.get().unwrap();
        match todos.load::<Todo>(conn) {
            Ok(_result) => Ok(_result),
            Err(_err) => Err(ServiceError::InternalServerError),
        }
    }

    pub fn create_todo(new_todo: TodoForm, _pool: web::Data<Pool>) -> Result<Todo, ServiceError> {
        let conn: &MysqlConnection = &_pool.get().unwrap();
        match diesel::insert_into(todos).values(&new_todo).execute(conn) {
            Ok(_) => {
                match todos.filter(title.eq(new_todo.title)).first::<Todo>(conn) {
                    Ok(_todo) => Ok(_todo),
                    Err(_e) => Err(ServiceError::InternalServerError),
                }
            }
            Err(_e) => Err(ServiceError::from(_e))
        }
    }

    pub fn update_todo(todo_id: i32, todo: TodoForm, _pool: web::Data<Pool>) -> Result<Todo, ServiceError> {
        let conn: &MysqlConnection = &_pool.get().unwrap();
        let todo = diesel::update(todos.find(todo_id))
            .set(title.eq(todo.title))
            .execute(conn);

        match todo {
            Ok(_) => {
                match todos_table.find(todo_id).first::<Todo>(conn) {
                    Ok(_todo) => Ok(_todo),
                    Err(_err) => match _err {
                        DBError::NotFound => Err(ServiceError::NotFound(format!("Todo with id:{} is Not Found", todo_id))),
                        _ => Err(ServiceError::InternalServerError)
                    }
                }
            }
            Err(_err) => Err(ServiceError::InternalServerError)
        }
    }

    pub fn remove_todo(_todo_id: i32, _pool: web::Data<Pool>) -> Result<(), ServiceError> {
        let conn: &MysqlConnection = &_pool.get().unwrap();
        println!("test");
        match todos.find(_todo_id).first::<Todo>(conn) {
            Ok(_todo) => {
                println!("OKKKKK {:#?}", _todo);
                match diesel::delete(todos.find(_todo_id)).execute(conn) {
                    Ok(_) => Ok(()),
                    Err(_err) => {
                        println!("{}", _err);
                        Err(ServiceError::InternalServerError)
                    }
                }
            }
            Err(_err) => {
                println!("not found man");
                Err(ServiceError::NotFound("Not Found".to_string()))
            }
        }
    }
}
