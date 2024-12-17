use sqlx::PgPool;
use crate::data::errors::DataError;

pub async fn create_user(pool: &PgPool, email: &str, password: &str) -> Result<(), DataError> {
    let hashed_password = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;

    let bytea_hash = hashed_password.as_bytes();

    sqlx::query!(
        "INSERT INTO users(email, password_hash)
    VALUES($1, $2)",
        email,
        bytea_hash )
        .execute(pool)
        .await.map_err(|err|{
        match err {
            sqlx::Error::Database(e) => {
                if e.constraint() == Some("users_email_key") {
                    DataError::FailedQuery("This email address is already in use".to_string())
                } else {
                    DataError::Internal(e.to_string())
                }
            },
            e => DataError::Query(e)
        }
    })?;

    Ok(())
}

pub async fn authenticate_user( pool: &PgPool, email: &str, password: &str) -> Result<i32, DataError> {
    let user = sqlx::query!("SELECT id, password_hash, email FROM users WHERE email = $1", email)
        .fetch_one(pool)
        .await.map_err(|e| {
        match e {
            sqlx::Error::RowNotFound => DataError::FailedQuery("Invalid credentials".to_string()),
            e => DataError::Query(e)
        }
    })?;

    let hashed_password = String::from_utf8(user.password_hash)?;
    let valid_password = bcrypt::verify(password, &hashed_password)?;

    if !valid_password {
        Err(DataError::FailedQuery("Invalid email or password".to_string()))
    } else {
       Ok(user.id)
    }
}