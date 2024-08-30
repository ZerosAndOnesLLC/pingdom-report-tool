use reqwest::{Client, header};
use serde_json::Value;
use chrono::{NaiveDate, DateTime, Utc};
use std::collections::HashMap;
use std::error::Error;
use clap::Parser;
use dotenv::dotenv;
use std::env;
use futures::stream::{self, StreamExt};
use tokio::time::{Duration, sleep};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Start date in MM/DD/YYYY format (e.g., 01/01/2024)
    #[arg(short, long)]
    start_date: Option<String>,

    /// End date in MM/DD/YYYY format (e.g., 12/31/2024)
    #[arg(short, long)]
    end_date: Option<String>,
}

#[derive(Clone)]
struct PingdomApi {
    pingdom_uri: String,
    api_key: String,
    client: Client,
}

impl PingdomApi {
    fn new(api_key: &str, pingdom_uri: &str) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        PingdomApi {
            pingdom_uri: pingdom_uri.to_string(),
            api_key: api_key.to_string(),
            client,
        }
    }

    async fn get_checks(&self) -> Result<String, Box<dyn Error>> {
        let response = self.client
            .get(&format!("{}/checks", self.pingdom_uri))
            .send()
            .await?;

        Ok(response.text().await?)
    }

    async fn get_perf_summary(
        &self,
        check_id: &str,
        from: &str,
        to: &str,
        includeuptime: &str,
        resolution: &str,
    ) -> Result<String, Box<dyn Error>> {
        let url = format!(
            "{}/summary.performance/{}?from={}&to={}&includeuptime={}&resolution={}",
            self.pingdom_uri, check_id, from, to, includeuptime, resolution
        );

        let response = self.client.get(&url).send().await?;

        Ok(response.text().await?)
    }

    async fn calculate_uptime(
        &self,
        check_id: &str,
        check_name: &str,
        from: &str,
        to: &str,
    ) -> Result<HashMap<String, Value>, Box<dyn Error>> {
        let mut uptime_calc = HashMap::new();
        uptime_calc.insert("id".to_string(), Value::String(check_id.to_string()));
        uptime_calc.insert("name".to_string(), Value::String(check_name.to_string()));
        uptime_calc.insert("uptime".to_string(), Value::Number(0.into()));
        uptime_calc.insert("downtime".to_string(), Value::Number(0.into()));
        uptime_calc.insert("unmonitored".to_string(), Value::Number(0.into()));
        uptime_calc.insert("max_uptime".to_string(), Value::Number(0.into()));
        uptime_calc.insert("percentage".to_string(), Value::Number(serde_json::Number::from_f64(0.0).unwrap()));
        uptime_calc.insert("downtime_mins".to_string(), Value::Number(0.into()));

        let check_uptime: Value = serde_json::from_str(&self.get_perf_summary(check_id, from, to, "true", "week").await?)?;

        for u in check_uptime["summary"]["weeks"].as_array().unwrap() {
            let uptime = uptime_calc["uptime"].as_u64().unwrap() + u["uptime"].as_u64().unwrap();
            let downtime = uptime_calc["downtime"].as_u64().unwrap() + u["downtime"].as_u64().unwrap();
            let downtime_mins = uptime_calc["downtime_mins"].as_u64().unwrap() + u["downtime"].as_u64().unwrap() / 60;
            let unmonitored = uptime_calc["unmonitored"].as_u64().unwrap() + u["unmonitored"].as_u64().unwrap();

            uptime_calc.insert("uptime".to_string(), Value::Number(uptime.into()));
            uptime_calc.insert("downtime".to_string(), Value::Number(downtime.into()));
            uptime_calc.insert("downtime_mins".to_string(), Value::Number(downtime_mins.into()));
            uptime_calc.insert("unmonitored".to_string(), Value::Number(unmonitored.into()));
        }

        let max_uptime = uptime_calc["uptime"].as_u64().unwrap() + uptime_calc["downtime"].as_u64().unwrap() + uptime_calc["unmonitored"].as_u64().unwrap();
        uptime_calc.insert("max_uptime".to_string(), Value::Number(max_uptime.into()));

        let percentage = ((uptime_calc["uptime"].as_u64().unwrap() as f64 + uptime_calc["unmonitored"].as_u64().unwrap() as f64) / max_uptime as f64 * 100.0 * 10000.0).round() / 10000.0;
        uptime_calc.insert("percentage".to_string(), Value::Number(serde_json::Number::from_f64(percentage).unwrap()));

        Ok(uptime_calc)
    }
}

fn parse_date(date_str: &str) -> Result<DateTime<Utc>, Box<dyn Error>> {
    let naive_date = NaiveDate::parse_from_str(date_str, "%m/%d/%Y")?;
    Ok(DateTime::<Utc>::from_utc(naive_date.and_hms(0, 0, 0), Utc))
}

fn print_usage() {
    println!("Pingdom Uptime Calculator");
    println!("Usage:");
    println!("  pingdom --start-date <MM/DD/YYYY> --end-date <MM/DD/YYYY>");
    println!("\nExample:");
    println!("  pingdom --start-date 01/01/2024 --end-date 12/31/2024");
    println!("\nNote:");
    println!("  Make sure to set the PINGDOM_API_KEY and PINGDOM_API_URL environment variables or add them to a .env file.");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok(); // Load .env file if it exists

    let args = Args::parse();

    if args.start_date.is_none() || args.end_date.is_none() {
        print_usage();
        return Ok(());
    }

    let start_date = parse_date(&args.start_date.unwrap())?;
    let end_date = parse_date(&args.end_date.unwrap())?;

    let uptime_from = start_date.timestamp();
    let uptime_to = end_date.timestamp();

    println!("Calculating uptime from {} to {}", start_date.format("%Y-%m-%d"), end_date.format("%Y-%m-%d"));

    let api_key = env::var("PINGDOM_API_KEY").expect("PINGDOM_API_KEY must be set in environment or .env file");
    let api_url = env::var("PINGDOM_API_URL").expect("PINGDOM_API_URL must be set in environment or .env file");
    let pingdom_api = PingdomApi::new(&api_key, &api_url);
    let all_checks: Value = serde_json::from_str(&pingdom_api.get_checks().await?)?;

    let uptime_calculations = stream::iter(all_checks["checks"].as_array().unwrap())
        .map(|c| {
            let pingdom_api = pingdom_api.clone();
            let check_id = c["id"].to_string();
            let check_name = c["name"].to_string();
            let uptime_from = uptime_from.to_string();
            let uptime_to = uptime_to.to_string();
            async move {
                let result = pingdom_api.calculate_uptime(&check_id, &check_name, &uptime_from, &uptime_to).await;
                sleep(Duration::from_millis(200)).await; // Add a small delay to avoid rate limiting
                result
            }
        })
        .buffer_unordered(10) // Process up to 10 requests concurrently
        .collect::<Vec<_>>()
        .await;

    let mut uptime_calculations: Vec<_> = uptime_calculations.into_iter().filter_map(Result::ok).collect();
    uptime_calculations.sort_by(|a, b| a["name"].to_string().cmp(&b["name"].to_string()));

    for u in uptime_calculations {
        println!("{}, {}%, {} mins", u["name"], u["percentage"], u["downtime_mins"]);
    }

    Ok(())
}