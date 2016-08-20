CREATE TABLE blogposts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  creationDate DATE NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f',
  urlPath VARCHAR NOT NULL,
  summary TEXT NOT NULL,
  body TEXT NOT NULL
)
