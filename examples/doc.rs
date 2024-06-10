use futures::StreamExt;
use subsquid_rs::{Query, Result, Source};

const ETHEREUM_MAINNET: &str =
    "https://v2.archive.subsquid.io/network/ethereum-mainnet";
const USDC_ADDRESS: &str = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48";

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let source = Source::new(ETHEREUM_MAINNET)?;
    let height = source.height().await?;
    println!("ETH block: {height}");

    let query = serde_json::json!({
        "fromBlock": height,
        "fields": {
            "transaction": {
                "hash": true
            }
        },
        "transactions": [
            {
                "to": [
                    USDC_ADDRESS
                ]
            }
        ]
    });
    let query: Query = serde_json::from_value(query)?;

    let batch = source.query(height - 1000, query.clone()).await?;
    println!("{}\n", serde_json::to_string_pretty(&batch[0])?);
    println!("Batch entries count: {}", batch.len());

    let stream = source.stream(height - 2000, query).await?;
    let count = stream.count().await;
    println!("Stream items count: {count}");

    Ok(())
}
