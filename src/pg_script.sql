CREATE TABLE migrations (
   id serial PRIMARY KEY,
   file_name VARCHAR UNIQUE NOT NULL,
   created_on TIMESTAMP NOT NULL DEFAULT NOW()
);
