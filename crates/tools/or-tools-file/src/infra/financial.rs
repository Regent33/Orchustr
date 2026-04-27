use super::shared::{load_credential, transport};
use crate::domain::contracts::DataSource;
use crate::domain::entities::FinancialRecord;
use crate::domain::errors::FileError;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::Value;
use url::Url;

const PROVIDER: &str = "financial";
const DEFAULT_URL: &str = "https://financialdatasets.ai/prices/snapshot";
const API_KEY_ENV: &str = "FINANCIAL_DATASETS_API_KEY";

pub struct FinancialDatasetsSource {
    client: reqwest::Client,
    endpoint: String,
    api_key: String,
}

impl FinancialDatasetsSource {
    pub fn from_env() -> Result<Self, FileError> {
        Ok(Self {
            client: reqwest::Client::new(),
            endpoint: DEFAULT_URL.to_string(),
            api_key: load_credential(API_KEY_ENV)?,
        })
    }

    #[must_use]
    pub fn with_config(
        client: reqwest::Client,
        endpoint: impl Into<String>,
        api_key: impl Into<String>,
    ) -> Self {
        Self {
            client,
            endpoint: endpoint.into(),
            api_key: api_key.into(),
        }
    }
}

#[derive(Deserialize)]
struct SnapshotResponse {
    snapshot: SnapshotData,
}

#[derive(Deserialize)]
struct SnapshotData {
    #[serde(default)]
    price: f64,
    #[serde(default)]
    day_change_percent: f64,
    #[serde(default)]
    volume: u64,
    #[serde(default)]
    market_cap: Option<f64>,
    #[serde(default)]
    name: String,
}

#[async_trait]
impl DataSource for FinancialDatasetsSource {
    fn name(&self) -> &'static str {
        PROVIDER
    }

    async fn fetch(&self, query: Value) -> Result<Value, FileError> {
        let symbol = query.get("symbol").and_then(|v| v.as_str()).unwrap_or("");
        if symbol.is_empty() {
            return Err(FileError::Json("missing `symbol`".into()));
        }
        let mut url =
            Url::parse(&self.endpoint).map_err(|e| FileError::Transport(e.to_string()))?;
        url.query_pairs_mut().append_pair("ticker", symbol);
        let resp = self
            .client
            .get(url)
            .header("X-API-KEY", &self.api_key)
            .send()
            .await
            .map_err(transport)?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(FileError::Upstream {
                provider: PROVIDER.into(),
                status: status.as_u16(),
                body,
            });
        }
        let parsed: SnapshotResponse = resp
            .json()
            .await
            .map_err(|e| FileError::Transport(e.to_string()))?;
        let record = FinancialRecord {
            symbol: symbol.to_uppercase(),
            name: parsed.snapshot.name,
            price: parsed.snapshot.price,
            change_pct: parsed.snapshot.day_change_percent,
            volume: parsed.snapshot.volume,
            market_cap: parsed.snapshot.market_cap,
            currency: "USD".into(),
        };
        Ok(serde_json::to_value(&record).unwrap_or(Value::Null))
    }
}
