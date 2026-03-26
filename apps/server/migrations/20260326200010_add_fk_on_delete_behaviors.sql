-- tracking_items.category_id: SET NULL on category delete
ALTER TABLE tracking_items DROP CONSTRAINT tracking_items_category_id_fkey;
ALTER TABLE tracking_items
    ADD CONSTRAINT tracking_items_category_id_fkey
    FOREIGN KEY (category_id) REFERENCES tracking_categories(id) ON DELETE SET NULL;

-- tracking_items.location_id: SET NULL on location delete
ALTER TABLE tracking_items DROP CONSTRAINT tracking_items_location_id_fkey;
ALTER TABLE tracking_items
    ADD CONSTRAINT tracking_items_location_id_fkey
    FOREIGN KEY (location_id) REFERENCES tracking_locations(id) ON DELETE SET NULL;

-- tracking_shopping_list_items.item_id: SET NULL on item delete
ALTER TABLE tracking_shopping_list_items DROP CONSTRAINT tracking_shopping_list_items_item_id_fkey;
ALTER TABLE tracking_shopping_list_items
    ADD CONSTRAINT tracking_shopping_list_items_item_id_fkey
    FOREIGN KEY (item_id) REFERENCES tracking_items(id) ON DELETE SET NULL;
