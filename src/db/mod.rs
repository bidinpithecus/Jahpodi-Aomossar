pub mod user;

use deadpool_diesel::postgres::{Manager, Pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub async fn init_db() -> Pool {
    let db_url = env::var("DATABASE_URL").unwrap();

    let manager = Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    let pool = Pool::builder(manager).build().unwrap();

    {
        let conn = pool.get().await.unwrap();
        conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
            .await
            .unwrap()
            .unwrap();
    }

    pool
}
