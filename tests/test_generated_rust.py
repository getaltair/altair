import json
from pathlib import Path

ROOT = Path("packages/contracts")
REGISTRY = ROOT / "registry"
RS_FILE = ROOT / "generated/rust/contracts.rs"


def load_json(name: str) -> dict:
    return json.loads((REGISTRY / name).read_text(encoding="utf-8"))


def to_rust_variant(value: str) -> str:
    return "".join(part.capitalize() for part in value.split("_"))


def test_rust_contains_all_registry_variants():
    text = RS_FILE.read_text(encoding="utf-8")

    entity_data = load_json("entity-types.json")
    relation_data = load_json("relation-types.json")
    stream_data = load_json("sync-streams.json")

    values = []
    for group_values in entity_data["entityTypes"].values():
        values.extend(group_values)
    values.extend(relation_data["relationTypes"])
    values.extend(relation_data["sourceTypes"])
    values.extend(relation_data["statusTypes"])
    values.extend(stream_data["autoSubscribed"])
    values.extend(stream_data["onDemand"])

    for value in values:
        variant = to_rust_variant(value)
        assert variant in text, f"Missing Rust variant: {variant}"
