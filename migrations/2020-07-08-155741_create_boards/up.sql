-- Your SQL goes here
CREATE TABLE boards (
	name TEXT NOT NULL,
    title TEXT,
	board_id serial PRIMARY KEY,
	last_bumped TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);