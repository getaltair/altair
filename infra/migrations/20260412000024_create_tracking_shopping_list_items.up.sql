-- migrate:up
CREATE TABLE tracking_shopping_list_items (
  id               UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
  shopping_list_id UUID         NOT NULL,
  item_id          UUID,
  name             VARCHAR(255) NOT NULL,
  quantity         NUMERIC      NOT NULL DEFAULT 1,
  status           VARCHAR(20)  NOT NULL DEFAULT 'pending',
  created_at       TIMESTAMPTZ  NOT NULL DEFAULT now(),
  updated_at       TIMESTAMPTZ  NOT NULL DEFAULT now(),
  deleted_at       TIMESTAMPTZ,

  CONSTRAINT fk_tracking_shopping_list_items_lists FOREIGN KEY (shopping_list_id) REFERENCES tracking_shopping_lists(id) ON DELETE CASCADE,
  CONSTRAINT fk_tracking_shopping_list_items_items FOREIGN KEY (item_id)          REFERENCES tracking_items(id)         ON DELETE SET NULL
);

CREATE INDEX idx_sli_list ON tracking_shopping_list_items(shopping_list_id);

CREATE TRIGGER tracking_shopping_list_items_updated_at
  BEFORE UPDATE ON tracking_shopping_list_items
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();
