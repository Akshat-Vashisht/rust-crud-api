-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
IF NOT EXISTS category (
    category_id  UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    category_name Varchar(255) NOT NULL UNIQUE,
    created_at TIMESTAMP
    WITH 
        TIME ZONE DEFAULT NOW()
);

CREATE TABLE 
IF NOT EXISTS item (
     item_id  UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
     item_name Varchar(255) NOT NULL UNIQUE,
     item_price INT NOT NULL,
     item_qty SMALLINT NOT NULL,
     category_id UUID NOT NULL,
     created_at TIMESTAMP 
     WITH 
        TIME ZONE DEFAULT NOW()
);
