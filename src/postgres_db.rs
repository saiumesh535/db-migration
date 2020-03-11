use postgres::{Client, Row};
use snafu::{ ResultExt };
use crate::errors::{Result};
use crate::errors::*;

pub struct PGClient(pub Client);

impl PGClient {
    // check if migration table available
    pub fn check_migration_table(&mut self) -> Result<()> {
        self.0.query("SELECT 'public.migrations'::regclass", &[]).context(
            DBErrorConfig { }
        )?;
        Ok(())
    }

    pub fn get_migrations(&mut self) -> Result<Vec<Row>> {
        Ok(self.0.query("SELECT file_name FROM migrations", &[]).context(DBErrorConfig { })?)
    }

    pub fn run_query(&mut self, query: &str, message: String) -> Result<()> {
        self.0.query(query, &[]).context(MigrationFileConfig {
            message
        })?;
        Ok(())
    }

    pub fn insert_migration_file(&mut self, file_name: &str) -> Result<()> {
        self.0.query("INSERT INTO migrations (file_name) VALUES ($1)", &[&file_name], ).context(DBErrorConfig { })?;
        Ok(())
    }
}