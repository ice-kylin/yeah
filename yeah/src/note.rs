use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Notes {
    notes: Vec<Note>,
}

#[derive(Deserialize, Serialize)]
struct Note {
    author: String,
    content: String,
}
