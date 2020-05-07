use crate::migration::run_migration;
use std::time::Instant;

mod errors;
mod fs_helpers;
mod migration;
mod postgres_db;

fn main() {
    let now = Instant::now();
    match run_migration() {
        Ok(_) => println!("migrations ran successfully"),
        Err(err) => eprintln!("{}", err),
    };
    let elapsed = now.elapsed();
    println!("Executed in: {:?}", elapsed);
}
