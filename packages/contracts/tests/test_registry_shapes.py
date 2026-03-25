import json
from pathlib import Path

ROOT = Path("packages/contracts")
REGISTRY = ROOT / "registry"


def load(name: str) -> dict:
    return json.loads((REGISTRY / name).read_text(encoding="utf-8"))


def test_entity_types_shape():
    data = load("entity-types.json")
    assert "version" in data
    assert "entityTypes" in data
    for group in ("core", "guidance", "knowledge", "tracking"):
        assert group in data["entityTypes"]
        assert isinstance(data["entityTypes"][group], list)
        assert all(isinstance(x, str) and x for x in data["entityTypes"][group])


def test_relation_types_shape():
    data = load("relation-types.json")
    for key in ("relationTypes", "sourceTypes", "statusTypes", "attachmentProcessingStates"):
        assert key in data
        assert isinstance(data[key], list)
        assert all(isinstance(x, str) and x for x in data[key])


def test_sync_streams_shape():
    data = load("sync-streams.json")
    assert "autoSubscribed" in data
    assert "onDemand" in data
    assert isinstance(data["autoSubscribed"], list)
    assert isinstance(data["onDemand"], list)


def test_no_duplicate_values():
    entity_data = load("entity-types.json")
    rel_data = load("relation-types.json")
    stream_data = load("sync-streams.json")

    entities = []
    for values in entity_data["entityTypes"].values():
        entities.extend(values)

    assert len(entities) == len(set(entities))
    assert len(rel_data["relationTypes"]) == len(set(rel_data["relationTypes"]))
    assert len(rel_data["sourceTypes"]) == len(set(rel_data["sourceTypes"]))
    assert len(rel_data["statusTypes"]) == len(set(rel_data["statusTypes"]))

    streams = stream_data["autoSubscribed"] + stream_data["onDemand"]
    assert len(streams) == len(set(streams))
