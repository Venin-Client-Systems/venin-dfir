use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub timestamp_utc: DateTime<Utc>,
    pub source_tool: String,
    pub artifact_type: String,
    pub event_type: String,
    pub description: String,
    pub source_path: Option<PathBuf>,
    pub evidence_id: Option<String>,
    pub fields: BTreeMap<String, Value>,
}

#[derive(Debug, Default)]
pub struct TimelineBuilder {
    events: Vec<TimelineEvent>,
}

impl TimelineBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, event: TimelineEvent) {
        self.events.push(event);
    }

    pub fn extend<I>(&mut self, events: I)
    where
        I: IntoIterator<Item = TimelineEvent>,
    {
        self.events.extend(events);
    }

    pub fn build(mut self) -> Vec<TimelineEvent> {
        self.events.sort_by_key(|event| event.timestamp_utc);
        self.events
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_events_by_utc_timestamp() {
        let mut builder = TimelineBuilder::new();
        builder.push(event_at("2024-01-02T00:00:00Z"));
        builder.push(event_at("2024-01-01T00:00:00Z"));

        let events = builder.build();
        assert_eq!(
            events[0].timestamp_utc.to_rfc3339(),
            "2024-01-01T00:00:00+00:00"
        );
        assert_eq!(
            events[1].timestamp_utc.to_rfc3339(),
            "2024-01-02T00:00:00+00:00"
        );
    }

    fn event_at(timestamp: &str) -> TimelineEvent {
        TimelineEvent {
            timestamp_utc: chrono::DateTime::parse_from_rfc3339(timestamp)
                .unwrap()
                .with_timezone(&Utc),
            source_tool: "test".to_string(),
            artifact_type: "test_artifact".to_string(),
            event_type: "test_event".to_string(),
            description: "test event".to_string(),
            source_path: None,
            evidence_id: None,
            fields: BTreeMap::new(),
        }
    }
}
