use crate::domain::errors::BeaconError;

pub(crate) fn sanitize_text(input: &str) -> String {
    input
        .chars()
        .filter(|ch| !ch.is_control() || matches!(ch, '\n' | '\r' | '\t'))
        .collect()
}

pub(crate) fn extract_variables(template: &str) -> Result<Vec<String>, BeaconError> {
    let mut variables = Vec::new();
    let mut remaining = template;
    while let Some(start) = remaining.find("{{") {
        let rest = &remaining[start + 2..];
        let Some(end) = rest.find("}}") else {
            return Err(BeaconError::InvalidTemplate(
                "unclosed template variable".to_owned(),
            ));
        };
        let name = rest[..end].trim();
        if name.is_empty()
            || !name
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-' | '.'))
        {
            return Err(BeaconError::InvalidTemplate(format!(
                "invalid variable name: {name}"
            )));
        }
        if !variables.iter().any(|existing| existing == name) {
            variables.push(name.to_owned());
        }
        remaining = &rest[end + 2..];
    }
    Ok(variables)
}
