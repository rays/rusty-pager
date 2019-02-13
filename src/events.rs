use super::errors::*;

use reqwest::{Client as HttpClient, StatusCode};
use serde_json::to_string as stringify;
use uuid::Uuid;

#[derive(Debug, Serialize)]
struct TriggerEventLink {
    href: String,
    text: String,
}

#[derive(Debug, Copy, Clone)]
/// Indicates the severity of the event
pub enum EventSeverity {
    /// Critical
    Critical,
    /// Error
    Error,
    /// Warning
    Warning,
    /// Informational
    Info,
}

#[derive(Debug, Serialize)]
struct TriggerEventPayload {
    summary: String,
    source: String,
    severity: String,
}

#[derive(Debug, Serialize)]
struct TriggerEvent {
    event_action: String,
    routing_key: String,
    dedup_key: String,
    links: Vec<TriggerEventLink>,
    payload: TriggerEventPayload,
}

impl TriggerEvent {
    fn new(
        routing_key: &str,
        dedup_key: &Option<String>,
        summary: &str,
        source: &str,
        severity: EventSeverity,
    ) -> TriggerEvent {
        TriggerEvent {
            event_action: "trigger".to_owned(),
            routing_key: routing_key.to_owned(),
            dedup_key: match dedup_key {
                Some(dedup_key) => dedup_key.to_owned(),
                None => Uuid::new_v4().to_string(), //.hyphenated().to_string(),
            },
            links: vec![],
            payload: TriggerEventPayload {
                summary: summary.to_owned(),
                source: source.to_owned(),
                severity: match severity {
                    EventSeverity::Critical => "critical",
                    EventSeverity::Error => "error",
                    EventSeverity::Warning => "warning",
                    EventSeverity::Info => "info",
                }.to_owned(),
            },
        }
    }
}

#[derive(Debug, Serialize)]
struct ResolveEvent {
    event_action: String,
    routing_key: String,
    dedup_key: String,
}

impl ResolveEvent {
    fn new(routing_key: &str, dedup_key: String) -> ResolveEvent {
        ResolveEvent {
            event_action: "resolve".to_owned(),
            routing_key: routing_key.to_owned(),
            dedup_key: dedup_key.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct RawEventResponse {
    status: String,
    message: String,
    dedup_key: String,
}

/// Interface for triggering, acknlowledging, and resolving events
#[derive(Debug)]
pub struct EventManager {
    integration_key: String,
    client: HttpClient,
}

impl EventManager {
    /// Instantiate a new EventManager using your PagerDuty integration key
    pub fn new(integration_key: String) -> EventManager {
        let client = HttpClient::new();
        EventManager {
            integration_key,
            client,
        }
    }

    /// Trigger an event and, if successful, return the event id
    pub fn trigger(
        &self,
        event_id: &Option<String>,
        summary: &str,
        source: &str,
        severity: EventSeverity,
    ) -> Result<String> {
        let trigger_event =
            TriggerEvent::new(&self.integration_key, event_id, summary, source, severity);
        let payload = stringify(&trigger_event)?;
        let response = self
            .client
            .post("https://events.pagerduty.com/v2/enqueue")
            .body(payload)
            .send()?;
        match response.status() {
            StatusCode::ACCEPTED => Ok(trigger_event.dedup_key),
            _ => Err(format!("invalid status code: {}", response.status()).into()),
        }
    }

    /// Resolve an event
    pub fn resolve(&self, event_id: &String) -> Result<()> {
        let resolve_event = ResolveEvent::new(&self.integration_key, event_id.to_owned());
        let payload = stringify(&resolve_event)?;
        let response = self
            .client
            .post("https://events.pagerduty.com/v2/enqueue")
            .body(payload)
            .send()?;
        match response.status() {
            StatusCode::ACCEPTED => Ok(()),
            _ => Err(format!("invalid status code: {}", response.status()).into()),
        }
    }
}
