use serde::{Deserialize, Serialize};
use super::schema::posts;
use super::schema::boards;
use chrono::naive::NaiveDateTime;
#[derive(Insertable, Queryable)]
#[table_name="boards"]
pub struct Board {
    pub name: String,
    pub title: String,
    pub board_id: i32,
}


#[derive(Queryable, Debug, Serialize)]
pub struct Post {
    pub name: Option<String>,
    pub text: String,
    pub post_id: i32,
    pub board_id: Option<i32>,
    pub ip: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub parent_id: Option<i32>,
    pub thread_id: Option<i32>,
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub name: String,
    pub text: String,
    pub board_id: i32,
    pub parent_id: Option<i32>,
    pub thread_id: Option<i32>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewPostForm {
    pub name: String,
    pub text: String,
    pub board_id: i32,
    pub parent_id: Option<i32>,
    pub thread_id: Option<i32>,
}