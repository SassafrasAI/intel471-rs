use intel471_rs::Intel471Client;

fn client() -> Intel471Client {
    dotenvy::dotenv_override().ok();
    let username = std::env::var("INTEL471_USERNAME").expect("INTEL471_USERNAME not set");
    let api_key = std::env::var("INTEL471_API_KEY").expect("INTEL471_API_KEY not set");
    Intel471Client::new(username, api_key)
}

#[tokio::test]
async fn test_girs_tree() {
    let client = client();
    let result = client.girs().tree().await;
    assert!(result.is_ok(), "girs tree failed: {:?}", result.err());
    let tree = result.unwrap();
    assert!(tree.count > 0, "expected non-zero GIR count, got {}", tree.count);
    assert!(!tree.girs.is_empty(), "expected non-empty girs list");
}

#[tokio::test]
async fn test_actors_stream() {
    let client = client();
    let page = client.actors()
        .stream_page(intel471_rs::models::actors::ActorStreamRequest {
            actor: "lazarus".to_string(),
            forum: None,
            from: None,
            until: None,
            server_type: None,
            size: Some(5),
            cursor: None,
        })
        .await
        .expect("actors stream failed");
    assert!(page.count > 0, "expected non-zero actor count");
}

#[tokio::test]
async fn test_entities_stream() {
    let client = client();
    let page = client.entities()
        .stream_page(intel471_rs::models::entities::EntityStreamRequest {
            entity: "intel.com".to_string(),
            type_: None,
            from: None,
            until: None,
            size: Some(5),
            cursor: None,
        })
        .await
        .expect("entities stream failed");
    assert!(page.count > 0, "expected non-zero entity count");
}

#[tokio::test]
async fn test_observables_stream() {
    let client = client();
    let result = client.observables()
        .stream_page(intel471_rs::models::observables::ObservableStreamRequest {
            observable: "8.8.8.8".to_string(),
            type_: None,
            from: None,
            until: None,
            size: Some(5),
            cursor: None,
        })
        .await;
    assert!(result.is_ok(), "observables stream failed: {:?}", result.err());
}

#[tokio::test]
async fn test_credentials_stream() {
    let client = client();
    let page = client.credentials()
        .credential_stream_page(intel471_rs::models::credentials::CredentialStreamRequest {
            credential_set_name: None,
            credential_set_id: None,
            domain: None,
            affiliation_group: None,
            password_strength: None,
            password_length_gte: None,
            password_lowercase_gte: None,
            password_uppercase_gte: None,
            password_numbers_gte: None,
            password_punctuation_gte: None,
            password_symbols_gte: None,
            password_separators_gte: None,
            password_other_gte: None,
            password_entropy_gte: None,
            password_plain: None,
            credential_login: None,
            detected_malware: None,
            girs: None,
            from: None,
            until: None,
            last_updated_from: None,
            last_updated_until: None,
            size: Some(1),
            cursor: None,
        })
        .await
        .expect("credentials stream failed");
    assert!(page.count >= 0, "credentials stream should not error");
}

#[tokio::test]
async fn test_indicators_stream() {
    let client = client();
    let page = client.indicators()
        .stream_page(intel471_rs::models::indicators::IndicatorStreamRequest {
            type_: Some(intel471_rs::models::indicators::IndicatorType::Domain),
            threat_type: None,
            confidence: None,
            text_filter: None,
            malware_id: None,
            malware_family_id: None,
            malware_family_name: None,
            girs: None,
            from: None,
            until: None,
            size: Some(5),
            cursor: None,
        })
        .await
        .expect("indicators stream failed");
    assert!(page.count > 0, "expected non-zero indicator count");
}

#[tokio::test]
async fn test_malware_list() {
    let client = client();
    let result = client.malware()
        .list_malware(intel471_rs::models::malware::MalwareListRequest {
            text_filter: None,
            malware_family_id: None,
            malware_family_name: None,
            girs: None,
            sort_by: None,
            order_by: None,
            page: None,
            size: Some(5),
            from: None,
            until: None,
        })
        .await;
    if let Err(ref e) = result {
        if e.is_forbidden() {
            eprintln!("malware list skipped: API key lacks malware-intel access (403)");
            return;
        }
    }
    let list = result.expect("malware list failed");
    assert!(list.count.unwrap_or(0) >= 0, "malware list should not error");
}