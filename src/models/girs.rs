use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GirsTreeRequest;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GirsTreeResponse {
    pub count: i64,
    pub girs: Vec<GirTree>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GirTree {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub deprecated: Option<bool>,
    pub children: Option<Vec<GirTree>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_girs_tree_response_deserialize() {
        let json = r#"{"count":3,"girs":[{"id":"g1","name":"Root GIR","description":"Top level","deprecated":false,"children":[{"id":"g2","name":"Child GIR","description":"Nested","deprecated":true,"children":[]}]},{"id":"g3","name":"Another Root","description":"Desc","deprecated":false}]}"#;
        let resp: GirsTreeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.count, 3);
        assert_eq!(resp.girs.len(), 2);
        assert_eq!(resp.girs[0].id, Some("g1".to_string()));
        assert_eq!(resp.girs[1].name, Some("Another Root".to_string()));
        let children = resp.girs[0].children.as_ref().unwrap();
        assert_eq!(children.len(), 1);
        assert!(children[0].deprecated.unwrap());
    }

    #[test]
    fn test_gir_tree_deserialize_only() {
        let json = r#"{"id":"abc-123","name":"Financial Services","description":"Banking sector","deprecated":false,"children":[]}"#;
        let tree: GirTree = serde_json::from_str(json).unwrap();
        assert_eq!(tree.id, Some("abc-123".to_string()));
        assert_eq!(tree.name, Some("Financial Services".to_string()));
        assert!(tree.children.as_ref().unwrap().is_empty());
    }

    #[test]
    fn test_girs_tree_with_null_children() {
        let json = r#"{"id":"g1","name":"Leaf","description":"No children","deprecated":false,"children":null}"#;
        let tree: GirTree = serde_json::from_str(json).unwrap();
        assert!(tree.children.is_none());
    }
}