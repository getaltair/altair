#!/usr/bin/env python3
"""
Generate shared contract bindings from canonical registry JSON.

Expected layout:
  packages/contracts/
    registry/
      entity-types.json
      relation-types.json
      sync-streams.json
    generated/
      typescript/
      kotlin/
      rust/

Usage:
  python scripts/generate_contracts.py --root packages/contracts
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path


def load_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def flatten_entity_types(data: dict) -> list[str]:
    result = []
    for group in ("core", "guidance", "knowledge", "tracking"):
        result.extend(data["entityTypes"].get(group, []))
    return result


def to_ts_const_name(value: str) -> str:
    return value.upper()


def to_kotlin_enum_name(value: str) -> str:
    return value.upper()


def to_rust_enum_name(value: str) -> str:
    return "".join(part.capitalize() for part in value.split("_"))


def generate_typescript(
    entity_types: list[str], relation_data: dict, stream_data: dict
) -> str:
    relation_types = relation_data["relationTypes"]
    source_types = relation_data["sourceTypes"]
    status_types = relation_data["statusTypes"]
    attachment_processing_states = relation_data["attachmentProcessingStates"]
    streams = stream_data["autoSubscribed"] + stream_data["onDemand"]

    def block(name: str, values: list[str]) -> str:
        lines = [f"export const {name} = {{"]
        for v in values:
            lines.append(f'  {to_ts_const_name(v)}: "{v}",')
        lines.append("} as const;")
        lines.append(
            f"export type {''.join(w.capitalize() for w in name.lower().split('_'))}Value = typeof {name}[keyof typeof {name}];"
        )
        return "\n".join(lines)

    return (
        "\n\n".join(
            [
                "// Generated from registry JSON. Do not edit by hand.",
                block("ENTITY_TYPES", entity_types),
                block("RELATION_TYPES", relation_types),
                block("RELATION_SOURCE_TYPES", source_types),
                block("RELATION_STATUS_TYPES", status_types),
                block("ATTACHMENT_PROCESSING_STATES", attachment_processing_states),
                block("SYNC_STREAMS", streams),
            ]
        )
        + "\n"
    )


def generate_kotlin(
    entity_types: list[str], relation_data: dict, stream_data: dict
) -> str:
    relation_types = relation_data["relationTypes"]
    source_types = relation_data["sourceTypes"]
    status_types = relation_data["statusTypes"]
    attachment_processing_states = relation_data["attachmentProcessingStates"]
    streams = stream_data["autoSubscribed"] + stream_data["onDemand"]

    def enum_block(name: str, values: list[str]) -> str:
        body = ",\n".join([f'    {to_kotlin_enum_name(v)}("{v}")' for v in values])
        return f"""enum class {name}(val wire: String) {{
{body};

    companion object {{
        fun fromWire(value: String): {name} =
            entries.firstOrNull {{ it.wire == value }}
                ?: error("Unknown {name}: $value")
    }}
}}"""

    return (
        "\n\n".join(
            [
                "// Generated from registry JSON. Do not edit by hand.",
                "package com.altair.contracts",
                enum_block("EntityType", entity_types),
                enum_block("RelationType", relation_types),
                enum_block("RelationSourceType", source_types),
                enum_block("RelationStatusType", status_types),
                enum_block("AttachmentProcessingState", attachment_processing_states),
                enum_block("SyncStream", streams),
            ]
        )
        + "\n"
    )


def generate_rust(
    entity_types: list[str], relation_data: dict, stream_data: dict
) -> str:
    relation_types = relation_data["relationTypes"]
    source_types = relation_data["sourceTypes"]
    status_types = relation_data["statusTypes"]
    attachment_processing_states = relation_data["attachmentProcessingStates"]
    streams = stream_data["autoSubscribed"] + stream_data["onDemand"]

    def enum_block(name: str, values: list[str]) -> str:
        body = ",\n\t".join(to_rust_enum_name(v) for v in values)
        return f"""#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum {name} {{
\t{body},
}}"""

    return (
        "\n\n".join(
            [
                "// Generated from registry JSON. Do not edit by hand.",
                "use serde::{Deserialize, Serialize};",
                enum_block("EntityType", entity_types),
                enum_block("RelationType", relation_types),
                enum_block("RelationSourceType", source_types),
                enum_block("RelationStatusType", status_types),
                enum_block("AttachmentProcessingState", attachment_processing_states),
                enum_block("SyncStream", streams),
            ]
        )
        + "\n"
    )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", required=True, help="Path to packages/contracts")
    args = parser.parse_args()

    root = Path(args.root)
    registry = root / "registry"
    generated = root / "generated"

    entity_data = load_json(registry / "entity-types.json")
    relation_data = load_json(registry / "relation-types.json")
    stream_data = load_json(registry / "sync-streams.json")

    entity_types = flatten_entity_types(entity_data)

    (generated / "typescript").mkdir(parents=True, exist_ok=True)
    (generated / "kotlin").mkdir(parents=True, exist_ok=True)
    (generated / "rust").mkdir(parents=True, exist_ok=True)

    (generated / "typescript" / "contracts.ts").write_text(
        generate_typescript(entity_types, relation_data, stream_data),
        encoding="utf-8",
    )
    (generated / "kotlin" / "Contracts.kt").write_text(
        generate_kotlin(entity_types, relation_data, stream_data),
        encoding="utf-8",
    )
    (generated / "rust" / "contracts.rs").write_text(
        generate_rust(entity_types, relation_data, stream_data),
        encoding="utf-8",
    )

    print("Generated contracts under:", generated)


if __name__ == "__main__":
    main()
