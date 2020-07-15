use crate::models;
use actix_web::web;
use diesel::pg::PgConnection;
use diesel::{QueryResult, NotFound, OptionalExtension};

use diesel::prelude::*;

use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;

pub fn insert_new_post(
    form_data: web::Form<models::NewPostForm>,
    conn: &PgConnection,
) -> Result<models::Post, diesel::result::Error> {
    use crate::schema::posts;
    use crate::schema::posts::dsl::*;
    let new_post = models::NewPost {
        name: form_data.name.to_owned(),
        text: form_data.text.to_owned(),
        board_id: form_data.board_id,
        parent_id: form_data.parent_id,
        thread_id: form_data.thread_id
    };
    let res: i32 = diesel::insert_into(posts::table)
        .values(new_post)
        .returning(post_id)
        .get_result(conn)?;
    Ok(models::Post {
        name: Some(form_data.name.to_owned()),
        text: form_data.text.to_owned(),
        board_id: Some(form_data.board_id),
        post_id: res,
        parent_id: form_data.parent_id,
        thread_id: form_data.thread_id,
        ip: None,
        created_at: None,
    })
}

pub fn get_board(id: i32, conn: &PgConnection) -> Result<String, diesel::result::Error> {
    use crate::diesel::ExpressionMethods;
    use crate::diesel::QueryDsl;
    use crate::diesel::RunQueryDsl;
    use crate::schema::boards::dsl::*;

    let res = boards
        .select(name)
        .filter(board_id.eq(id))
        .limit(1)
        .load::<String>(conn)?;
    if res.len() > 0 {
        return Ok(res[0].clone());
    }
    Ok("/".into())
}

pub fn get_board_id_by_slug(
    slug: String,
    conn: &PgConnection,
) -> Result<(i32, Option<String>), diesel::result::Error> {
    use crate::diesel::ExpressionMethods;

    use crate::schema::boards::dsl::*;

    let res = boards
        .select((board_id,title))
        .filter(name.eq(slug))
        .limit(1)
        .load::<(i32, Option<String>)>(conn)?;
    if res.len() > 0 {
        return Ok(res[0].clone());
    }
    Err(NotFound)   
}

pub fn get_posts(
    id: i32,
    conn: &PgConnection,
) -> Result<Vec<models::Post>, diesel::result::Error> {
    use crate::schema::posts::dsl::*;
    Ok(posts.filter(thread_id.eq(id)).order_by(created_at)
        .limit(50)
        .load::<models::Post>(conn)?)
} 