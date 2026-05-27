use intel471_rs::Intel471Client;
use intel471_rs::models::actors::ActorStreamRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let username = std::env::var("INTEL471_USERNAME").expect("INTEL471_USERNAME not set");
    let password = std::env::var("INTEL471_PASSWORD").expect("INTEL471_PASSWORD not set");

    let client = Intel471Client::new(&username, &password);

    println!("=== Streaming Actors ===\n");
    stream_actors(&client).await?;

    println!("\n=== Getting a Credential by ID ===\n");
    get_credential_by_id(&client).await?;

    println!("\n=== Getting GIRS Tree ===\n");
    get_girs_tree(&client).await?;

    println!("\n=== Creating and Listing Brand Exposure Monitors ===\n");
    create_and_list_monitors(&client).await?;

    Ok(())
}

async fn stream_actors(client: &Intel471Client) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = client.actors().stream(ActorStreamRequest {
        actor: "Conti".to_string(),
        from: None,
        until: None,
        forum: None,
        server_type: None,
        size: Some(5),
        cursor: None,
    })?;

    if let Some(page) = stream.next_page().await? {
        println!("Found {} actors on first page:", page.actors.len());
        for actor in &page.actors {
            println!("  - ID: {}", actor.id);
        }
    } else {
        println!("No actors found.");
    }

    Ok(())
}

async fn get_credential_by_id(client: &Intel471Client) -> Result<(), Box<dyn std::error::Error>> {
    let credential_id = "example-credential-id";

    match client.credentials().get_credential(credential_id).await {
        Ok(cred) => {
            println!("Credential ID: {}", cred.id);
        }
        Err(e) if e.is_not_found() => {
            println!("Credential '{}' not found (expected for demo)", credential_id);
        }
        Err(e) => return Err(e.into()),
    }

    Ok(())
}

async fn get_girs_tree(client: &Intel471Client) -> Result<(), Box<dyn std::error::Error>> {
    let tree = client.girs().tree().await?;

    println!("GIRS tree contains {} root entries:", tree.girs.len());
    for gir in &tree.girs {
        let name = gir.name.as_deref().unwrap_or("(unnamed)");
        let id = gir.id.as_deref().unwrap_or("(no-id)");
        println!("  - {} ({})", name, id);
    }

    Ok(())
}

async fn create_and_list_monitors(client: &Intel471Client) -> Result<(), Box<dyn std::error::Error>> {
    use intel471_rs::models::brand_exposure::{
        CreateMonitorRequest, Frequency, Impact, ListMonitorsRequest,
    };

    let monitors = client.brand_exposure().list_monitors(ListMonitorsRequest {
        last_run_after: None,
        last_run_before: None,
    }).await?;

    println!("Found {} existing monitors:", monitors.len());
    for m in &monitors {
        println!("  - {} (ID: {})", m.definition.name, m.id);
    }

    let request = CreateMonitorRequest {
        name: "example.com".to_string(),
        targets: vec!["example.com".to_string()],
        labels: vec!["demo".to_string()],
        frequency: Frequency::weekly,
        impact: Impact::moderate,
        alerts: None,
        created_at: None,
        disabled_modules: None,
        event_types: None,
        iteration: None,
        start_at: None,
    };

    match client.brand_exposure().create_monitor(&request).await {
        Ok(response) => {
            println!("\nCreated monitor with ID: {}", response.id);

            let monitor = client.brand_exposure().get_monitor(&response.id).await?;
            println!("Monitor name: {}", monitor.definition.name);

            client.brand_exposure().delete_monitor(&response.id).await?;
            println!("Deleted monitor {}", response.id);
        }
        Err(e) => {
            println!("Could not create monitor (may require specific API permissions): {}", e);
        }
    }

    Ok(())
}