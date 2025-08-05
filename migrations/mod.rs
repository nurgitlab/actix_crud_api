use refinery::embed_migrations;
embed_migrations!("migrations");

pub async fn run_migrations(pool: &deadpool_postgres::Pool) -> Result<(), Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let migration_report = migrations::runner()
        .run_async(&mut **client)
        .await?;

    for migration in migration_report.applied_migrations() {
        log::info!(
            "Migration Applied - Name: {}, Version: {}",
            migration.name(),
            migration.version()
        );
    }

    Ok(())
}