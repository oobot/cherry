use sqlx::Database;

pub trait DataSource {
    type Database: Database;
}
