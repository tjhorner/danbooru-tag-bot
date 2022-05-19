CREATE TABLE subscriptions (
  id        SERIAL    PRIMARY KEY,
  tag       text      NOT NULL,
  user_id   bigint    NOT NULL
);