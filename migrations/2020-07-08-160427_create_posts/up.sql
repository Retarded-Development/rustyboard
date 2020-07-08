-- Your SQL goes here
CREATE TABLE posts (
	name TEXT default 'Bernd',
    text TEXT NOT NULL,
	post_id serial PRIMARY KEY,
	board_id INTEGER REFERENCES boards(board_id),
    ip TEXT,
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);