use futures::Stream;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to build Reqwest HTTP client: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Invalud worker URL: '{0}'")]
    Worker(String),
    #[error("Failed to parse height as integer: {0}")]
    Height(#[from] std::num::ParseIntError),
    #[error("Failed to parse JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    Message(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
pub struct Source {
    url: String,
    client: Client,
}

impl Source {
    pub fn new<S: AsRef<str>>(url: S) -> Result<Self> {
        let url = url.as_ref().to_owned();
        let _ = reqwest::Url::parse(&url)
            .map_err(|_| Error::Message(format!("Invalid URL: {url}")))?;
        Ok(Self {
            url,
            client: reqwest::ClientBuilder::new().build()?,
        })
    }

    pub async fn height(&self) -> Result<u64> {
        let url = format!("{}/height", self.url);
        tracing::debug!(url, "querying height");
        let height =
            self.client.get(url).send().await?.text().await?.parse()?;
        tracing::debug!(height, "querying height succeeded");
        Ok(height)
    }

    pub async fn worker(&self, height: u64) -> Result<Worker> {
        let url = format!("{}/{height}/worker", self.url);
        tracing::debug!(url, "querying worker");
        let url = self.client.get(url).send().await?.text().await?;
        let _ = reqwest::Url::parse(&url)
            .map_err(|_| Error::Worker(url.clone()))?;
        tracing::debug!(url, height, "worker received");
        Ok(Worker { url })
    }

    pub async fn query(&self, height: u64, query: Query) -> Result<Vec<Entry>> {
        let query = Query {
            from_block: height,
            ..query
        };
        let url = self.worker(height).await?.url;
        tracing::debug!(height, ?query, "running data query");
        let body = self
            .client
            .post(url)
            .json(&query)
            .send()
            .await?
            .text()
            .await?;
        tracing::debug!(body, "data query returned a response");

        let response: Response = serde_json::from_str(&body)?;
        match response {
            Response::Batch(entries) => {
                tracing::debug!(size = entries.len(), "data batch received");
                Ok(entries)
            }
            Response::Message(error) => {
                tracing::debug!(error, "data query failed");
                Err(Error::Message(error))
            }
        }
    }

    pub async fn stream(
        &self,
        height: u64,
        query: Query,
    ) -> Result<impl Stream<Item = Entry>> {
        struct Context {
            query: Query,
            source: Source,
            height: u64,
            current_batch: Vec<Entry>,
            current_index: usize,
        }

        impl Context {
            fn inc(self) -> Self {
                Self {
                    current_index: self.current_index + 1,
                    ..self
                }
            }

            fn next(self, height: u64, batch: Vec<Entry>) -> Self {
                Context {
                    height,
                    current_batch: batch,
                    current_index: 1,
                    ..self
                }
            }
        }

        async fn step(ctx: Context) -> Option<(Entry, Context)> {
            if ctx.current_index < ctx.current_batch.len() {
                let entry = ctx.current_batch[ctx.current_index].clone();
                return Some((entry, ctx.inc()));
            }

            let height = ctx
                .current_batch
                .last()
                .map(|e| e.header.number)
                .unwrap_or_default();
            let height = ctx.height.max(height) + 1;
            let batch =
                ctx.source.query(height, ctx.query.clone()).await.ok()?;

            let entry = batch.first().cloned()?;
            let ctx = ctx.next(height, batch);
            Some((entry, ctx))
        }

        let batch = self.query(height, query.clone()).await?;

        let ctx = Context {
            query,
            source: self.clone(),
            height,
            current_batch: batch,
            current_index: 0,
        };

        Ok(futures::stream::unfold(ctx, step))
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum Response {
    Message(String),
    Batch(Vec<Entry>),
}

#[derive(Debug)]
pub struct Worker {
    url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Header {
    pub number: u64,
    // TODO: consider wrapping `hash` into a value-object
    pub hash: String,
    #[serde(rename = "parentHash")]
    pub parent_hash: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tx {
    pub hash: String,
    #[serde(rename = "transactionIndex")]
    pub index: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Entry {
    header: Header,
    #[serde(default)]
    transactions: Vec<Tx>,
}

// TODO: add a nice builder for `Query`

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Query {
    #[serde(rename = "fromBlock")]
    from_block: u64,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "toBlock")]
    to_block: Option<u64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<FieldsQuery>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    transactions: Vec<TxQuery>,
    // TODO add remaining properties
    // logs: ()
    // traces: ()
    // stateDiffs: ()
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FieldsQuery {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    transaction: Option<TxFields>,
    // TODO add remaining properties
    // log: (),
    // state_diff: (),
    // trace: (),
    // block: (),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TxFields {
    #[serde(default)]
    hash: bool,
    // TODO add remaining properties
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TxQuery {
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    from: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    to: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    sighash: Vec<String>,
    #[serde(default)]
    logs: bool,
    #[serde(default)]
    traces: bool,
    #[serde(rename = "stateDiffs")]
    #[serde(default)]
    state_diffs: bool,
}

// TODO: rigorous testing (skipped for now)
