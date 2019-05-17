CREATE TABLE item_tag (
    item_id INT REFERENCES item(id),
    tag_id INT REFERENCES tag(id),
    value INT NOT NULL DEFAULT 0,
    CONSTRAINT item_tags_pkey PRIMARY KEY (item_id, tag_id)
);