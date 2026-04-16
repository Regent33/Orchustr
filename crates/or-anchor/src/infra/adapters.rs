use crate::domain::entities::AnchorChunk;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub(crate) fn chunk_text(document_id: &str, text: &str, chunk_size: usize) -> Vec<AnchorChunk> {
    text.split_whitespace()
        .collect::<Vec<_>>()
        .chunks(chunk_size.max(1))
        .enumerate()
        .map(|(index, words)| AnchorChunk {
            id: format!("{document_id}:{index}"),
            text: words.join(" "),
        })
        .collect()
}

pub(crate) fn embed(text: &str) -> Vec<f32> {
    let mut vector = vec![0.0f32; 64];
    for token in text
        .split(|character: char| !character.is_alphanumeric())
        .filter(|token| !token.is_empty())
    {
        let mut hasher = DefaultHasher::new();
        token.to_ascii_lowercase().hash(&mut hasher);
        let slot = (hasher.finish() as usize) % vector.len();
        vector[slot] += 1.0;
    }
    let norm = vector.iter().map(|value| value * value).sum::<f32>().sqrt();
    if norm > 0.0 {
        for value in &mut vector {
            *value /= norm;
        }
    }
    vector
}
