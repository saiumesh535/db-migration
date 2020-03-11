use snafu::{ ResultExt };
use std::env::var;
use crate::errors::{ Result };
use crate::errors::*;
use postgres::{Client, NoTls};
use crate::postgres_db::PGClient;
use crate::fs_helpers::{get_all_sql_paths, get_yet_to_run_migration_files, get_queries_from_file};

const UP_TYPE: &str = "up";
const DOWN_TYPE: &str = "down";
const MIGRATION_TYPE: &str = "migration_type";

fn get_migration_type() -> Result<String> {
    Ok(var(MIGRATION_TYPE).context(EnvErrorConfig{
        message: format!("variable {} is missing", MIGRATION_TYPE)
    })?)
}

fn verify_migration_type(migration_type: &String) -> Result<()> {
    let types: Vec<&str> = vec![UP_TYPE, DOWN_TYPE];
    if  !types.contains(&migration_type.as_str()) {
        return MigrationTypeConfig {
            message: format!("{} should be either {} or {}", MIGRATION_TYPE, UP_TYPE, DOWN_TYPE)
        }.fail();
    }
    Ok(())
}


fn connect_postgres_db() -> Result<PGClient> {
    let db_url = var("DB_URL").unwrap_or("postgresql://postgres:postgres@localhost/migration-test".to_string());
    Ok(PGClient(Client::connect(db_url.as_str(), NoTls).context(DBErrorConfig{})?))
}

pub fn run_migration() -> Result<()> {

    let migration_type = get_migration_type()?;
    let _ = verify_migration_type(&migration_type)?;

    // connect database
    let mut pg_client = connect_postgres_db()?;
    // make sure migrations table exists
    let _ = pg_client.check_migration_table()?;

    // migrated files
    let mut migrated_files: Vec<String> = vec![];
    for migration_row in pg_client.get_migrations()? {
        migrated_files.push(migration_row.get::<&str, String>("file_name"))
    }

    // get sql paths
    let sql_paths = get_all_sql_paths()?;

    // run these migration files
    let migration_files = get_yet_to_run_migration_files(sql_paths.clone(), migration_type.clone(), &migrated_files);

    if migration_files.len() == 0 {
        return CustomMessageError {
            message: String::from("No migrations to run")
        }.fail();
    }

    // iterate through sql files and run query
    for migration_file in migration_files {
        let queries = get_queries_from_file(migration_file.clone())?;
        let file_name = String::from(migration_file.clone().file_name().unwrap().to_str().unwrap());
        for query in queries {
            let err_message = format!("error while running file {:?}", &file_name);
            pg_client.run_query(query.as_str(), err_message)?;
        }
        pg_client.insert_migration_file(file_name.as_str())?;
    }

    Ok(())
}