#![doc(html_root_url = "https://docs.rs/rusty-pager")]
#![doc(issue_tracker_base_url = "https://github.com/sbruton/rusty-pager/issues/")]
#![deny(
    missing_docs, missing_debug_implementations, missing_copy_implementations, trivial_casts,
    trivial_numeric_casts, unsafe_code, unstable_features, unused_import_braces,
    unused_qualifications, unused_variables, unreachable_code, unused_comparisons, unused_imports,
    unused_must_use
)]

//! The `rust_pager` crate provides a Rust SDK for the PagerDuty APIs.
//!
//! Currently only the Events v2 API is supported.
//!
//! ## Usage Example
//!
//! ```rust
//! use rusty_pager::events::{EventManager, EventSeverity};
//!
//! fn main() {
//!    // your pagerduty integration key
//!    let integration_key = String::from("your-integration-key");
//!    // create a new event manager
//!    let event_mgr = EventManager::new(integration_key);
//!
//!    // starting out without an event id (until we trigger one...)
//!    let mut event_id: Option<String> = None;
//!
//!    // trigger a new event
//!    event_id = match event_mgr.trigger(
//!        &event_id,            // this is currently None
//!        "some message",
//!        "my-monitoring-agent",
//!        EventSeverity::Critical,
//!    ) {
//!        Ok(event_id) => Some(event_id),
//!        Err(err) => panic!("failed to trigger pagerduty event! {:?}", err),
//!    };
//!
//!    // re-trigger an existing event
//!    match event_mgr.trigger(
//!        &event_id,            // since we set this to the last event_id, it'll retrigger
//!        "some message",
//!        "my-monitoring-agent",
//!        EventSeverity::Critical,
//!    ) {
//!        Ok(_) => {},
//!        Err(err) => panic!("failed to re-trigger pagerduty event! {:?}", err),
//!    };
//!
//!    // resolve an existing event
//!    match event_mgr.resolve(event_id.as_ref().unwrap()) {
//!        Ok(()) => {},
//!        Err(err) => panic!("failed to resolve pagerduty event! {:?}", err),
//!    };
//! }
//! ```
#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

mod errors {
    use reqwest::Error as HttpError;
    use serde_json::Error as JsonError;

    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{
        foreign_links {
            Json(JsonError);
            Http(HttpError);
        }
    }
}

/// interfaces for the PagerDuty Events API v2
pub mod events;
