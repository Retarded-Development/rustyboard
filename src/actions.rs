
use diesel::pg::PgConnection;
use crate::models;
use actix_web::{web};

pub fn insert_new_post(
    // prevent collision with `name` column imported inside the function
    form_data: web::Form<models::NewPostForm>,
    conn: &PgConnection,
) -> Result<models::Post, diesel::result::Error> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.

    use crate::schema::posts;
    use crate::diesel::RunQueryDsl;
    use crate::schema::posts::dsl::*;
    let new_post = models::NewPost {
        name: form_data.name.to_owned(),
        text: form_data.text.to_owned(),
        board_id: form_data.board_id,
    };
    let res: i32 = diesel::insert_into(posts::table).values(new_post).returning(post_id).get_result(conn)?;
    dbg!(res);
    Ok(models::Post{
        name: form_data.name.to_owned(),
        text: form_data.text.to_owned(),
        board_id: form_data.board_id,
        post_id: res,
    })
}