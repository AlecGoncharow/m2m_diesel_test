CREATE TABLE item_tags (
    item_id INT REFERENCES items(id),
    tag_id INT REFERENCES tags(id),
    value INT NOT NULL DEFAULT 0,
    CONSTRAINT item_tags_pkey PRIMARY KEY (item_id, tag_id)
);