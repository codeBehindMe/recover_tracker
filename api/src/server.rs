use recovery_tracker::tracker_server::{Tracker, TrackerServer};
use recovery_tracker::{EmptyRequest, HealthzResponse};
use tonic::{transport::Server, Request, Response, Status};

pub mod recovery_tracker {
    tonic::include_proto!("recovery_tracker");
}

#[derive(Debug, Default)]
pub struct MyTracker {}

#[tonic::async_trait]
impl Tracker for MyTracker {
    async fn healthz(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<HealthzResponse>, Status> {
        let reply = HealthzResponse {
            status: "ok".into(),
        };
        Ok(Response::new(reply))
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port: String;

    match std::env::var("PORT") {
        Ok(val) => port = val,
        Err(_) => port = "8080".into(),
    }

    println!("port is {:?}", port);

    let addr = format!("0.0.0.0:{port}").parse()?;
    let tracker = MyTracker::default();

    println!("starting server on port: {:?}", port);

    Server::builder()
        .add_service(TrackerServer::new(tracker))
        .serve(addr)
        .await?;

    Ok(())
}
