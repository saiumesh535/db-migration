# db-migration
Postgres DB migration tool, written in rust 🦀

## This tool follows specific file structure for sql files as follows

```
project
│   README.md
└───migrations
│   │   1_some.up.sql
|   |   1_some.down.sql
│   │
└───folder2
    │   file021.txt
    │   file022.txt
```
more info on structure

```
{number_filename.{type(up|down)}.sql}
1_init.up.sql
1_init.down.sql
```

you can refer [this](https://github.com/saiumesh535/db-migration/tree/master/src/migrations) folder


### commands

1. First you need to migrations folder in database, query can be found [here](https://github.com/saiumesh535/db-migration/blob/master/src/pg_script.sql)

Commands to run

## with migration_type
```cmd
migration_type=down ./target/release/db_migraiton.exe
```

## with database URL
optionally it uses **DB_URL** as

```cmd
postgresql://postgres:postgres@localhost/migration-test
```

you can override by writing following command
```cmd
migration_type=down DB_URL=postgresql://postgres:postgres@localhost/migration-test ./target/release/db_migraiton.exe
```

optionally it uses **migration_path** as follows
```cmd
migration_path=src/migrations
```


you can also override **migration_path** folder path as follows
```cmd
migration_type=down migration_path=src/scripts/migration_path ./target/release/db_migraiton.exe
```