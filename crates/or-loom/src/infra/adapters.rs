use crate::domain::errors::LoomError;
use std::collections::{BTreeSet, HashMap};

pub(crate) fn validate_graph_shape<'a>(
    node_names: impl Iterator<Item = &'a String>,
    edges: &HashMap<String, Vec<String>>,
    entry: Option<&str>,
    exit: Option<&str>,
) -> Result<(), LoomError> {
    let collected = node_names.collect::<Vec<_>>();
    if collected.is_empty() {
        return Err(LoomError::EmptyGraph);
    }

    let entry = entry.ok_or(LoomError::MissingEntry)?;
    let exit = exit.ok_or(LoomError::MissingExit)?;
    let mut seen = BTreeSet::new();
    for name in &collected {
        if name.trim().is_empty() {
            return Err(LoomError::BlankNodeName);
        }
        if !seen.insert((*name).clone()) {
            return Err(LoomError::DuplicateNode((*name).clone()));
        }
    }

    if !seen.contains(entry) {
        return Err(LoomError::UnknownNode(entry.to_owned()));
    }
    if !seen.contains(exit) {
        return Err(LoomError::UnknownNode(exit.to_owned()));
    }

    for (from, targets) in edges {
        if !seen.contains(from) {
            return Err(LoomError::EdgeReferencesUnknownNode(from.clone()));
        }
        for to in targets {
            if !seen.contains(to) {
                return Err(LoomError::EdgeReferencesUnknownNode(to.clone()));
            }
        }
    }
    Ok(())
}
