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
        request: Request<EmptyRequest>,
    ) -> Result<Response<HealthzResponse>, Status> {
        let reply = HealthzResponse {
            status: "ok".into(),
        };
        Ok(Response::new(reply))
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:5051".parse()?;
    let tracker = MyTracker::default();

    Server::builder()
        .add_service(TrackerServer::new(tracker))
        .serve(addr)
        .await?;

    Ok(())
}
