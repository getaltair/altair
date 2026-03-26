CREATE TABLE item_tags (
    item_id UUID NOT NULL REFERENCES tracking_items(id) ON DELETE CASCADE,
    tag_id UUID NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (item_id, tag_id)
);
