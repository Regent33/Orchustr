use crate::domain::entities::PrismConfig;
use crate::domain::errors::PrismError;

pub(crate) fn prism_config(otlp_endpoint: &str) -> Result<PrismConfig, PrismError> {
    let endpoint = otlp_endpoint.trim();
    if endpoint.is_empty() {
        return Err(PrismError::InvalidEndpoint(
            "endpoint must not be blank".to_owned(),
        ));
    }
    reqwest::Url::parse(endpoint)
        .map_err(|error| PrismError::InvalidEndpoint(error.to_string()))?;
    Ok(PrismConfig {
        otlp_endpoint: endpoint.to_owned(),
        service_name: "orchustr".to_owned(),
    })
}
