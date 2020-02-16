CREATE TABLE posts (
  id INTEGER NOT NULL PRIMARY KEY,
  title VARCHAR NOT NULL,
  author_id INTEGER NOT NULL,
  category_id INTEGER NOT NULL,
  FOREIGN KEY(author_id) REFERENCES authors(id),
  FOREIGN KEY(category_id) REFERENCES category(id)
);

CREATE TABLE authors (
  id INTEGER NOT NULL PRIMARY KEY,
  first_name VARCHAR NOT NULL
);

CREATE TABLE category (
  id INTEGER NOT NULL PRIMARY KEY,
  title VARCHAR NOT NULL
);


INSERT INTO authors (first_name) VALUES ("author0");
INSERT INTO authors (first_name) VALUES ("author1");
INSERT INTO authors (first_name) VALUES ("author2");
INSERT INTO authors (first_name) VALUES ("author3");


INSERT INTO category (id, title) VALUES (1, "category1");
INSERT INTO category (id, title) VALUES (2, "category2");
INSERT INTO category (id, title) VALUES (3, "category3");

INSERT INTO posts (title, author_id, category_id) VALUES ("post0", 1, 1);
INSERT INTO posts (title, author_id, category_id) VALUES ("post_", 1, 2);
INSERT INTO posts (title, author_id, category_id) VALUES ("post_", 1, 3);

