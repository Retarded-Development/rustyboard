use serde::{Deserialize, Serialize};
use super::schema::posts;
use super::schema::boards;

#[derive(Insertable, Queryable)]
#[table_name="boards"]
pub struct Board {
    pub name: String,
    pub title: String,
    pub board_id: i32,
}


#[derive(Queryable, Serialize, Insertable)]
#[table_name="posts"]
pub struct Post {
    pub post_id: i32,
    pub name: String,
    pub text: String,
    pub board_id: i32,
}

#[derive(Insertable, Queryable)]
#[table_name="posts"]
pub struct NewPost {
    pub name: String,
    pub text: String,
    pub board_id: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewPostForm {
    pub name: String,
    pub text: String,
    pub board_id: i32,
}