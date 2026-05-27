# intel471-rs

An async Rust client library for the [Intel471](https://intel471.com) cyber threat intelligence platform.

## Overview

`intel471-rs` provides typed, ergonomic access to the Intel471 API. It covers all 13 API services with streaming pagination, single-resource lookups, monitor CRUD, and raw response handling where appropriate.

The library uses HTTP Basic Auth with your Verity API application credentials (client ID and client secret) and supports configurable base URLs for testing environments.

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
intel471-rs = "0.1"
```

Or via `cargo add`:

```sh
cargo add intel471-rs
```

## Authentication

The Intel471 API authenticates via HTTP Basic Auth using your Verity application credentials. Create an application in the Verity Developer Portal to obtain a client ID and client secret, then pass them as the username and password respectively:

```rust
use intel471_rs::Intel471Client;

let client = Intel471Client::new("client_id", "client_secret");
```

To use a custom base URL:

```rust
let client = Intel471Client::with_base_url("client_id", "client_secret", "https://custom.api.example.com");
```

## Quick Start

```rust
use intel471_rs::Intel471Client;
use intel471_rs::models::actors::ActorStreamRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Intel471Client::new("client_id", "client_secret");

    // Stream actors matching a search term
    let mut stream = client.actors().stream(ActorStreamRequest {
        actor: "Conti".to_string(),
        from: None,
        until: None,
        forum: None,
        server_type: None,
        size: Some(10),
        cursor: None,
    })?;

    while let Some(page) = stream.next_page().await? {
        for actor in &page.actors {
            println!("{}: {}", actor.id, actor.activity.first_seen_ts);
        }
    }

    Ok(())
}
```

## API Coverage

| Service | Client Accessor | Key Methods |
|---------|----------------|-------------|
| Actors | `client.actors()` | `stream`, `stream_page` |
| ASE | `client.ase()` | `list_monitors` |
| Brand Exposure | `client.brand_exposure()` | Monitor CRUD, scan data endpoints, config endpoints |
| Credentials | `client.credentials()` | Credential/set/occurrence streams and lookups |
| Entities | `client.entities()` | `stream`, `stream_page` |
| GIRS | `client.girs()` | `tree` |
| Indicators | `client.indicators()` | `stream`, `stream_page`, `get_by_id` |
| Malware Intel | `client.malware()` | `stream`, `get_by_id`, `list_malware`, `get_malware_family`, `download_file` |
| Observables | `client.observables()` | `stream`, `stream_page` |
| Reports | `client.reports()` | Stream/detail for FINTel, Breach Alert, Geopol, Info, Spot, Malware, Vulnerability reports |
| Sources | `client.sources()` | Stream/detail for data-leak-site posts, forum posts/messages, chat messages |
| TPRM | `client.tprm()` | Monitor CRUD, scan data endpoints, config endpoints |
| Watchers | `client.watchers()` | `alerts_stream`, `update_alert_status`, `watcher_groups`, `watchers` |

## Usage

### Streaming with Pagination

Many Intel471 endpoints return paginated results via a cursor-based API. The library exposes two patterns:

- **`stream()`** returns a `PaginatedStream` for automatic cursor-based pagination.
- **`stream_page()`** fetches a single page and returns the response directly.

```rust
use intel471_rs::Intel471Client;
use intel471_rs::models::actors::ActorStreamPage;
use intel471_rs::api::actors::ActorStreamRequest;

let client = Intel471Client::new("client_id", "client_secret");

// Automatic pagination
let mut stream = client.actors().stream(ActorStreamRequest {
    actor: "Conti".to_string(),
    from: Some(1700000000),
    until: None,
    forum: None,
    server_type: None,
    size: Some(10),
    cursor: None,
})?;

while let Some(page) = stream.next_page().await? {
    println!("Page with {} actors", page.actors.len());
}

// Collect all pages
let all_pages: Vec<ActorStreamPage> = client
    .actors()
    .stream(ActorStreamRequest {
        actor: "Conti".to_string(),
        from: None,
        until: None,
        forum: None,
        server_type: None,
        size: None,
        cursor: None,
    })?
    .collect_all()
    .await?;
```

### Single-Resource Lookups

```rust
// Get a credential by ID
let credential = client.credentials().get_credential("credential-id").await?;

// Get an indicator by ID
let indicator = client.indicators().get_by_id("indicator-id").await?;

// Get a malware event by ID
let event = client.malware().get_by_id("event-id").await?;

// Get the GIRS tree
let tree = client.girs().tree().await?;
for gir in &tree.girs {
    println!("{}: {}", gir.id.as_deref().unwrap_or("-"), gir.name.as_deref().unwrap_or("-"));
}
```

### Monitor Management

The Brand Exposure and TPRM services support full monitor lifecycle operations:

```rust
use intel471_rs::models::brand_exposure::{
    CreateMonitorRequest, EditMonitorRequest, Frequency, Impact, ListMonitorsRequest,
};

// Create a monitor
let request = CreateMonitorRequest {
    name: "example.com".to_string(),
    targets: vec!["example.com".to_string()],
    labels: vec!["brand-monitor".to_string()],
    frequency: Frequency::daily,
    impact: Impact::major,
    alerts: None,
    created_at: None,
    disabled_modules: None,
    event_types: None,
    iteration: None,
    start_at: None,
};

let response = client.brand_exposure().create_monitor(&request).await?;

// List monitors
let monitors = client.brand_exposure().list_monitors(ListMonitorsRequest {
    last_run_after: None,
    last_run_before: None,
}).await?;

// Edit a monitor
client.brand_exposure().edit_monitor(&response.id, &EditMonitorRequest {
    name: "updated-name".to_string(),
    targets: vec!["example.com".to_string()],
    labels: vec!["brand-monitor".to_string()],
    frequency: Frequency::daily,
    impact: Impact::major,
    alerts: None,
    created_at: None,
    disabled_modules: None,
    event_types: None,
    iteration: None,
    start_at: None,
}).await?;

// Delete a monitor
client.brand_exposure().delete_monitor(&response.id).await?;
```

### Error Handling

The library provides a structured error type with convenience methods:

```rust
use intel471_rs::Error;

match client.credentials().get_credential("invalid-id").await {
    Ok(cred) => println!("{:?}", cred),
    Err(Error::Api { status: 404, .. }) => println!("Not found"),
    Err(Error::Api { status: 401, .. }) => println!("Unauthorized"),
    Err(Error::Api { status: 403, .. }) => println!("Forbidden"),
    Err(Error::Api { status: 429, .. }) => println!("Rate limited"),
    Err(e) => println!("Other error: {}", e),
}

// Convenience methods
if let Err(e) = result {
    if e.is_not_found() { /* handle 404 */ }
    if e.is_unauthorized() { /* handle 401 */ }
    if e.is_forbidden() { /* handle 403 */ }
    if e.is_rate_limited() { /* handle 429/503 */ }
}
```

## Crate Features

The library is built on top of:

- **reqwest** — HTTP client with native TLS and JSON support
- **serde** / **serde_json** — Serialization and deserialization
- **chrono** — Date and time handling with Serde support
- **tokio** — Async runtime (dev-dependency for tests)

## Minimum Supported Rust Version

This crate requires Rust 2024 edition (MSRV 1.85+).

## License

Licensed under the [BSD 3-Clause License](LICENSE).