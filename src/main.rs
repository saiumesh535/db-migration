use postgres::{Client, Error as PGError, NoTls, Row};
use std::env;
use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;
use std::process;

struct PGClient {
    pg_instance: Client,
}

fn read_file(path: PathBuf) -> String {
    read_to_string(path).unwrap()
}

fn print_error(message: &str, err: String) {
    println!("{message} {err}", message = message, err = err);
    process::exit(1);
}

impl PGClient {
    fn check_migration_table(&mut self) {
        match self
            .pg_instance
            .query("SELECT 'public.migrations'::regclass", &[])
        {
            Ok(_) => (),
            Err(err) => {
                return print_error("unable to find migrations table", err.to_string());
            }
        }
    }

    fn get_migrations(&mut self) -> Result<Vec<Row>, PGError> {
        self.pg_instance
            .query("SELECT file_name FROM migrations", &[])
    }

    fn run_query(&mut self, query: &str, err_message: &str) {
        match self.pg_instance.query(query, &[]) {
            Ok(_) => (),
            Err(err) => {
                return print_error(err_message, err.to_string());
            }
        }
    }

    fn insert_file(&mut self, file_name: &str, err_message: &str) {
        match self.pg_instance.query(
            "INSERT INTO migrations (file_name) VALUES ($1)",
            &[&file_name],
        ) {
            Ok(_) => (),
            Err(err) => {
                return print_error(err_message, err.to_string());
            }
        }
    }
}

fn get_all_paths() -> Vec<PathBuf> {
    let dir = match read_dir("src/migrations") {
        Ok(dir) => dir,
        Err(err) => {
            println!("{}", err.to_string());
            process::exit(1);
        }
    };
    let mut sql_paths: Vec<PathBuf> = vec![];
    for file in dir {
        sql_paths.push(file.unwrap().path());
    }
    sql_paths
}

fn main() {
    let migration_type = match env::var("type") {
        Ok(migration_type) => migration_type,
        Err(err) => {
            return print_error("type cannot be empty", err.to_string());
        }
    };
    let db_url = "postgresql://postgres:postgres@localhost/migration-test";
    let client = match Client::connect(db_url, NoTls) {
        Ok(client) => client,
        Err(err) => {
            return print_error("unable to connect to DB due to", err.to_string());
        }
    };
    let mut pg_client = PGClient {
        pg_instance: client,
    };
    // first ensure that migrations table exists
    pg_client.check_migration_table();

    let migrations_rows = match pg_client.get_migrations() {
        Ok(migrations_rows) => migrations_rows,
        Err(err) => {
            return print_error("failed to get migration files", err.to_string());
        }
    };

    let mut migration_files: Vec<String> = vec![];
    for migration_row in migrations_rows {
        migration_files.push(migration_row.get::<&str, String>("file_name"))
    }

    // get all paths and run them
    let sql_paths = get_all_paths();
    let mut filtered_sql_paths: Vec<PathBuf> = vec![];
    for path in sql_paths.clone() {
        let file_name = String::from(path.clone().file_name().unwrap().to_str().unwrap());
        let file_migration_type: Vec<String> = file_name.clone().split(".")
            .map(|s| s.to_string())
            .collect();
        if !migration_files.contains(&file_name) && migration_type == file_migration_type[file_migration_type.len() - 2] {
            filtered_sql_paths.push(path);
        }
    }
    if filtered_sql_paths.len() == 0 {
        println!("No migrations to run");
        process::exit(0);
    }
    for path in filtered_sql_paths {
        let path_split: Vec<String> = path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .split(".")
            .map(|s| s.to_string())
            .collect();

        if migration_type == path_split[path_split.len() - 2] {
            let err_message = format!("error while running file {:?}", path);
            let insert_err_message = format!("error while inserting file {:?}", path);
            let file_name = String::from(path.clone().file_name().unwrap().to_str().unwrap());
            pg_client.run_query(read_file(path.clone()).as_str(), err_message.as_str());
            pg_client.insert_file(file_name.as_str(), insert_err_message.as_str());
        }
    }
}
