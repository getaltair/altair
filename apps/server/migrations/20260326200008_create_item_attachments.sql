CREATE TABLE item_attachments (
    item_id UUID NOT NULL REFERENCES tracking_items(id) ON DELETE CASCADE,
    attachment_id UUID NOT NULL REFERENCES attachments(id) ON DELETE CASCADE,
    PRIMARY KEY (item_id, attachment_id)
);
