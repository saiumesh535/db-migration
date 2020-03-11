use crate::migration::run_migration;

mod errors;
mod migration;
mod fs_helpers;
mod postgres_db;


fn main() {
    match run_migration() {
        Ok(_) => {
            println!("migrations ran successfully")
        },
        Err(err) => {
            eprintln!("{}", err)
        }
    };
}
