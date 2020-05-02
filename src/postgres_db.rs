use crate::errors::Result;
use crate::errors::*;
use postgres::{Client, Row};
use snafu::ResultExt;

pub struct PGClient(pub Client);

pub struct QueryTransaction {
    pub query: String,
    pub message: String,
}

impl PGClient {
    // check if migration table available
    pub fn check_migration_table(&mut self, schema: &String) -> Result<()> {
        let query = format!("SELECT '{}.migrations'::regclass", schema);
        self.0
            .query(query.as_str(), &[])
            .context(DBErrorConfig {})?;
        Ok(())
    }

    pub fn get_migrations(&mut self, schema: &String) -> Result<Vec<Row>> {
        let query = format!("SELECT file_name FROM {}.migrations", schema);
        Ok(self
            .0
            .query(query.as_str(), &[])
            .context(DBErrorConfig {})?)
    }

    pub fn insert_migration_file(&mut self, file_name: &str, schema: &String) -> Result<()> {
        let query = format!("INSERT INTO {}.migrations (file_name) VALUES ($1)", schema);
        self.0
            .query(query.as_str(), &[&file_name])
            .context(DBErrorConfig {})?;
        Ok(())
    }

    /**
    here we don't need schema since, user will mention that
    */
    pub fn transaction(&mut self, query_transactions: Vec<QueryTransaction>) -> Result<()> {
        let mut transaction = self.0.transaction().context(DBErrorConfig {})?;

        for query_transactions in query_transactions {
            transaction
                .batch_execute(query_transactions.query.as_str())
                .context(MigrationFileConfig {
                    message: query_transactions.message,
                })?;
        }
        transaction.commit().context(DBErrorConfig {})?;
        Ok(())
    }
}
