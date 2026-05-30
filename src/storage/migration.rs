use anyhow::Ok;
use rusqlite::Connection;
use std::path::PathBuf;

#[derive(Debug)]
struct MigrationFile {
    id: String,
    path: PathBuf,
    sql: String,
}

pub fn run_migrations(connection: &mut Connection) -> anyhow::Result<()> {
    ensure_migration_table(&connection)?;

    let mut migration_files = load_migration_files()?;
    sort_migration_files(&mut migration_files);

    let applied_migrations = get_applied_migrations(&connection)?;

    let pending_migrations: Vec<MigrationFile> =
        get_pending_migrations(migration_files, applied_migrations);

    for migration in pending_migrations {
        apply_migration(migration, connection)?;
    }

    Ok(())
}

fn load_migration_files() -> anyhow::Result<Vec<MigrationFile>> {
    let files_dir = PathBuf::from("migrations");
    let mut migrations = Vec::new();

    for entry in std::fs::read_dir(files_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|ext| ext.to_str()) == Some("sql") {
            let id = path
                .file_stem()
                .ok_or_else(|| anyhow::anyhow!("Invalid file name: {:?}", path))?
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("Non-UTF8 file name: {:?}", path))?
                .to_string();

            let sql = std::fs::read_to_string(&path)?;

            migrations.push(MigrationFile { id, path, sql });
        }
    }

    Ok(migrations)
}

fn ensure_migration_table(connection: &Connection) -> anyhow::Result<()> {
    let sql = r#"
        CREATE TABLE IF NOT EXISTS schema_migrations (
            id TEXT PRIMARY KEY,
            applied_at TEXT NOT NULL
        );
    "#;

    connection.execute(sql, [])?;
    Ok(())
}

fn get_applied_migrations(connection: &Connection) -> anyhow::Result<Vec<String>> {
    let sql = "SELECT id FROM schema_migrations";
    let mut stmt = connection.prepare(sql)?;
    let rows = stmt.query_map([], |row| row.get(0))?;
    let mut ids = Vec::new();
    for id in rows {
        ids.push(id?);
    }
    Ok(ids)
}

fn apply_migration(migration: MigrationFile, connection: &mut Connection) -> anyhow::Result<()> {
    let transaction = connection.transaction()?;

    transaction.execute_batch(&migration.sql)?;
    tracing::info!("migrations applied: {}", migration.id);
    transaction.execute(
        "INSERT INTO schema_migrations (id, applied_at) VALUES (?1, datetime('now'))",
        [migration.id],
    )?;

    transaction.commit()?;

    Ok(())
}

fn sort_migration_files(migration_files: &mut [MigrationFile]) {
    migration_files.sort_by(|a, b| a.id.cmp(&b.id));
}

fn get_pending_migrations(
    migration_files: Vec<MigrationFile>,
    applied_migrations: Vec<String>,
) -> Vec<MigrationFile> {
    migration_files
        .into_iter()
        .filter(|migration| !applied_migrations.contains(&migration.id))
        .collect()
}
