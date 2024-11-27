use chrono::{DateTime, Utc};

struct Documentation {
    description: String,
    examples: Vec<String>,
}

struct Metadata {
    author: String,
    version: String,
    timestamp: DateTime<Utc>,
    documentation: Documentation,
}

enum ReturnType {
    Int,
    Void,
}

struct Signature {
    return_type: ReturnType,
}

struct ContentSnapshot {
    source_code: String,
    commit_message: String,
    timestamp: DateTime<Utc>,
    author: String,
}

pub struct MainResource {
    id: String,
    name: String,
    metadata: Metadata,
    signature: Signature,
    content: Vec<ContentSnapshot>,
}
