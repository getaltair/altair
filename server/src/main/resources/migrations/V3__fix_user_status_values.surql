-- Fix user status field to match UserStatus enum
-- The enum has: active, disabled, deleted
-- Previous schema only allowed: active, suspended, pending

REMOVE FIELD status ON user;
DEFINE FIELD status ON user TYPE string ASSERT $value IN ['active', 'disabled', 'deleted'];
