use std::path::PathBuf;
use std::env::var;
use crate::errors::Result;
use crate::errors::*;
use std::fs::{read_dir, read_to_string};
use snafu::{ ResultExt };

pub fn get_all_sql_paths() ->  Result<Vec<PathBuf>> {
    let path = var("migration_path").unwrap_or("src/migrations".to_string());
    let dir = read_dir(&path).context(
        DirError { message: path }
    )?;
    let mut sql_paths: Vec<PathBuf> = vec![];
    for entry in dir {
        let file = entry.unwrap();
        if file.path().extension().unwrap() ==  "sql" {
            sql_paths.push(file.path());
        }
    }
    Ok(sql_paths)
}

fn split_string(input: String, by: &str) -> Vec<String> {
    input.split(by).map(|s| s.to_string()).collect()
}

/**
filter migration files which are yet to run
*/
pub fn get_yet_to_run_migration_files(sql_paths: Vec<PathBuf>, migration_type: String, migrated_files: &Vec<String>) -> Vec<PathBuf> {
    let mut filtered_sql_paths: Vec<PathBuf> = vec![];
    for path in sql_paths {
        let file_name = String::from(path.clone().file_name().unwrap().to_str().unwrap());
        let file_migration_type: Vec<String> = split_string(file_name.clone(), ".");
        if !migrated_files.contains(&file_name) && path.clone().extension().unwrap() == "sql" && migration_type == file_migration_type[file_migration_type.len() - 2] {
            filtered_sql_paths.push(path);
        }
    }
    filtered_sql_paths
}

pub fn get_queries_from_file(path: PathBuf) -> Result<Vec<String>> {
    let mut queries: Vec<String> = vec![];
    let query = read_to_string(path.clone()).context(ReadConfiguration { path: path.clone() })?;
    queries.extend(split_string(query, ";"));
    Ok(queries)
}