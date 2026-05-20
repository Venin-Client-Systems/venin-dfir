from __future__ import annotations

from dataclasses import dataclass
from datetime import datetime, timezone
import json
from pathlib import Path
from typing import Any


@dataclass(frozen=True)
class TimelineEvent:
    timestamp_utc: datetime
    source_tool: str
    artifact_type: str
    event_type: str
    description: str
    source_path: str | None
    evidence_id: str | None
    fields: dict[str, Any]


def load_timeline(path: str | Path) -> list[TimelineEvent]:
    """Load normalized VENIN timeline JSON without mutating source evidence."""
    raw_events = json.loads(Path(path).read_text(encoding="utf-8"))
    events: list[TimelineEvent] = []

    for raw in raw_events:
        timestamp = datetime.fromisoformat(raw["timestamp_utc"].replace("Z", "+00:00"))
        events.append(
            TimelineEvent(
                timestamp_utc=timestamp.astimezone(timezone.utc),
                source_tool=raw["source_tool"],
                artifact_type=raw["artifact_type"],
                event_type=raw["event_type"],
                description=raw["description"],
                source_path=raw.get("source_path"),
                evidence_id=raw.get("evidence_id"),
                fields=raw.get("fields", {}),
            )
        )

    return sorted(events, key=lambda event: event.timestamp_utc)
