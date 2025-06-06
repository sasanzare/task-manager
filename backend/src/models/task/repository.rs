use sqlx::PgPool;
use super::model::{Task, TaskStatus};

pub struct TaskRepository;

impl TaskRepository {
    pub async fn create(pool: &PgPool, title: &str, user_id: i32) -> Result<Task, sqlx::Error> {
        // implementation
    }
    
    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Task, sqlx::Error> {
        // implementation
    }
    
}