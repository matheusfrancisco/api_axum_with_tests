use super::connect::DB;
use eyre::Result;
use serde::{Deserialize, Serialize};

pub async fn insert_post(db: DB, text: &str, parent_id: Option<i32>) -> Result<i32> {
    let result = sqlx::query!(
        r#"
            INSERT INTO posts (text, parent_id) 
            SELECT $1, $2 
            RETURNING post_id;
        "#,
        text,
        parent_id
    )
    .fetch_one(&db)
    .await?;

    Ok(result.post_id)
}

pub async fn get_all_top_level(db: DB) -> Result<Vec<Post>> {
    Ok(sqlx::query_as!(
        Post,
        "SELECT post_id as id, text, likes FROM posts WHERE parent_id IS NULL ORDER BY post_id DESC",
    ).fetch_all(&db).await?)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    id: i32,
    text: String,
    likes: i32,
}
