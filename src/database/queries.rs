use super::connect::DB;
use eyre::Result;

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
