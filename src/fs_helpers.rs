use crate::errors::Result;
use crate::errors::*;
use snafu::{ OptionExt, ResultExt };
use std::env::var;
use std::ffi::OsStr;
use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;

pub fn get_all_sql_paths() -> Result<Vec<PathBuf>> {
    let path = var("migration_path").unwrap_or("src/migrations".to_string());
    let dir = read_dir(&path).context(DirError { message: &path })?;
    let require_file_extension = OsStr::new("sql");
    let mut sql_paths: Vec<PathBuf> = vec![];
    for entry in dir {
        let file = entry.context(DirError { message: &path })?;
        if file.path().extension() == Some(require_file_extension) {
            sql_paths.push(file.path());
        }
    }
    Ok(sql_paths)
}

fn split_string(input: &String, by: &str) -> Vec<String> {
    input.split(by).map(|s| s.to_string()).collect()
}

/**
filter migration files which are yet to run
*/
pub fn get_yet_to_run_migration_files<'a>(
    sql_paths: &'a Vec<PathBuf>,
    migration_type: &String,
    migrated_files: &Vec<String>,
) -> Result<Vec<&'a PathBuf>> {
    let mut filtered_sql_paths: Vec<&PathBuf> = vec![];
    for path in sql_paths {
        let file_name = String::from(
            path.file_name().context(NoneError)?.to_str().context(NoneError)?,
        );
        let file_migration_type: Vec<String> = split_string(&file_name, ".");
        if !migrated_files.contains(&file_name)
            && migration_type == &file_migration_type[file_migration_type.len() - 2]
        {
            filtered_sql_paths.push(&path);
        }
    }
    Ok(filtered_sql_paths)
}

pub fn get_query_from_file(path: &PathBuf) -> Result<String> {
    Ok(read_to_string(path).context(ReadConfiguration { path })?)
}
