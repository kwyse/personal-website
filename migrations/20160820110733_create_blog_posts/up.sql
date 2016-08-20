CREATE TABLE blogposts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  created DATE NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f',
  url VARCHAR NOT NULL,
  summary TEXT NOT NULL,
  body TEXT NOT NULL
)
