CREATE TABLE items (
    id SERIAL PRIMARY KEY,
    instance_of INT NOT NULL DEFAULT 0,
    title TEXT NOT NULL
);

CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
    instance_of INT NOT NULL DEFAULT 0,
    title TEXT NOT NULL
);

CREATE TABLE item_foos (
    id INT PRIMARY KEY NOT NULL,
    text TEXT
);

CREATE TABLE item_bars (
    id INT PRIMARY KEY NOT NULL,
    number int
);

CREATE TABLE tag_foos (
    id INT PRIMARY KEY NOT NULL,
    text TEXT
);

CREATE TABLE tag_bars (
    id INT PRIMARY KEY NOT NULL,
    number int
);