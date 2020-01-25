use postgres::{Client, NoTls};
use std::error::Error;
use std::process;
use std::fs::{ read_to_string };

fn read_file() -> String {
    read_to_string("src/migrations/some.sql").unwrap()
}

fn run_query(client: &mut Client, query: &str) {
    match client.query(query, &[]) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("unable to run query {}", e.to_string());
            process::exit(1);
        }
    };
}

fn main() {
    let db_url = "postgresql://postgres:postgres@localhost/migration-test";
    let mut client = match Client::connect(db_url, NoTls) {
        Ok(client) => client,
        Err(e) => {
            eprintln!("unable to connect to DB {}", e.to_string());
            process::exit(1);
        }
    };
    let sql_file = read_file();
    println!("{}", sql_file);
    run_query(&mut client, sql_file.as_str());
}
