CREATE TABLE item (
    id SERIAL PRIMARY KEY,
    instance_of INT NOT NULL DEFAULT 0,
    title TEXT NOT NULL
);

CREATE TABLE tag (
    id SERIAL PRIMARY KEY,
    instance_of INT NOT NULL DEFAULT 0,
    title TEXT NOT NULL
);

CREATE TABLE item_foo (
    id INT PRIMARY KEY NOT NULL,
    text TEXT
);

CREATE TABLE item_bar (
    id INT PRIMARY KEY NOT NULL,
    number int
);

CREATE TABLE tag_foo (
    id INT PRIMARY KEY NOT NULL,
    text TEXT
);

CREATE TABLE tag_bar (
    id INT PRIMARY KEY NOT NULL,
    number int
);