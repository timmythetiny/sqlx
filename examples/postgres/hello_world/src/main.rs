use sqlx::{Connect, Connection, Executor, PgConnection};

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let mut conn = PgConnection::connect("postgres://postgres@localhost").await?;

    let affected_rows: u64 = conn.execute("SELECT 1").await?;

    println!("affected_rows: {}", affected_rows);

    Ok(())
}
