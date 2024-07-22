use recovery_tracker::tracker_client::TrackerClient;
use recovery_tracker::{EmptyRequest, HealthzResponse};

pub mod recovery_tracker {
    tonic::include_proto!("recovery_tracker");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TrackerClient::connect("http://0.0.0.0:8080").await?;

    let request = tonic::Request::new(EmptyRequest {});

    let response = client.healthz(request).await?;

    println!("RESPONSE={:?}", response);
    Ok(())
}
