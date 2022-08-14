// User data, which is stored and accessible in all command invocations
pub struct Data {
    pub pool: sqlx::PgPool,
    pub cache: super::Cache,
}
