use chrono::{DateTime, Utc};
use uuid::Uuid;

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

pub enum ReturnType {
    Int,
    Void,
}

pub struct Signature {
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

impl MainResource {
    pub fn new(
        name: String,
        author: String,
        version: String,
        description: String,
        examples: Vec<String>,
        signature: Signature,
    ) -> Self {
        let current_time = Utc::now();
        let documentation = Documentation {
            description,
            examples,
        };
        let metadata = Metadata {
            author: author.to_string(),
            version,
            timestamp: current_time,
            documentation,
        };
        // Create initial content snapshot with a basic "Hello, World!" program
        let initial_content = ContentSnapshot {
            source_code: String::from("fn main() {\n    println!(\"Hello, World!\");\n}"),
            commit_message: String::from("Initial commit"),
            timestamp: current_time,
            author,
        };

        Self {
            id: Uuid::new_v4().to_string(),
            name,
            metadata,
            signature,
            content: vec![initial_content],
        }
    }

    /// Adds a new content snapshot to the history
    pub fn update_content(&mut self, new_source: String, commit_message: String, author: String) {
        self.content.push(ContentSnapshot {
            source_code: new_source,
            commit_message,
            timestamp: Utc::now(),
            author,
        });
    }
}
