CREATE TABLE post_index (
  id SERIAL PRIMARY KEY,
  last_seen_post integer NOT NULL
);

INSERT INTO post_index (last_seen_post) VALUES (0);