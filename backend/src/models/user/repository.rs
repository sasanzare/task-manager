use sqlx::PgPool;
use uuid::Uuid;
use bcrypt::{hash, verify, DEFAULT_COST};
use super::model::{User, NewUser, LoginUser};

pub struct UserRepository;

impl UserRepository {
    pub async fn create(pool: &PgPool, new_user: NewUser) -> Result<User, sqlx::Error> {
        let password_hash = hash(new_user.password, DEFAULT_COST).unwrap();
        
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, email, password_hash)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            new_user.username,
            new_user.email,
            password_hash
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users WHERE email = $1
            "#,
            email
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn verify_credentials(pool: &PgPool, login_user: LoginUser) -> Result<User, String> {
        let user = match Self::find_by_email(pool, &login_user.email).await {
            Ok(user) => user,
            Err(_) => return Err("Invalid credentials".to_string()),
        };

        match verify(login_user.password, &user.password_hash) {
            Ok(valid) if valid => Ok(user),
            _ => Err("Invalid credentials".to_string()),
        }
    }
}
