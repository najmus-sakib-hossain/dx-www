// Server-side RPC handler for dx-query

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::ecosystem::EcosystemState;

/// Binary RPC request
#[derive(Debug, Deserialize)]
pub struct RPCRequest {
    pub query_id: String,
    pub params: Vec<u8>, // Binary parameters
}

/// Binary RPC response
#[derive(Debug, Serialize)]
pub struct RPCResponse {
    pub data: Vec<u8>, // Binary data
    pub cached: bool,
}

/// Handle RPC query request
pub async fn handle_rpc(
    State(state): State<Arc<EcosystemState>>,
    Json(req): Json<RPCRequest>,
) -> impl IntoResponse {
    // Check cache first
    #[cfg(feature = "query")]
    if let Some(ref cache) = state.query_cache {
        let query_hash = hash_query(&req.query_id, &req.params);

        if let Some(cached_data) = cache.get(query_hash) {
            return (
                StatusCode::OK,
                Json(RPCResponse {
                    data: cached_data.to_vec(),
                    cached: true,
                }),
            );
        }
    }

    // Execute query
    let data = execute_query(&req.query_id, &req.params).await;

    // Cache result
    #[cfg(feature = "query")]
    if let Some(ref cache) = state.query_cache {
        let key = dx_query::QueryKey::from_bytes(req.query_id.as_bytes());
        cache.set_with_ttl(key, data.clone(), 300); // 5 min TTL
    }

    (
        StatusCode::OK,
        Json(RPCResponse {
            data,
            cached: false,
        }),
    )
}

/// Execute query (placeholder)
async fn execute_query(query_id: &str, params: &[u8]) -> Vec<u8> {
    // This would dispatch to actual query handlers
    vec![0, 1, 2, 3] // Placeholder response
}

/// Hash query for caching
fn hash_query(query_id: &str, params: &[u8]) -> u32 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    query_id.hash(&mut hasher);
    params.hash(&mut hasher);
    hasher.finish() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_query() {
        let hash1 = hash_query("getUser", &[1, 2, 3]);
        let hash2 = hash_query("getUser", &[1, 2, 3]);
        let hash3 = hash_query("getUser", &[4, 5, 6]);

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }
}
