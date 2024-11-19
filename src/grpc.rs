use proto::admin_server::Admin;
use proto::calculator_server::Calculator;
use tonic::metadata::MetadataValue;
use tonic::{Request, Status};
pub mod proto {
    tonic::include_proto!("calculator");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("calculator_descriptor");
}

pub type State = std::sync::Arc<tokio::sync::RwLock<u64>>;

#[derive(Debug, Default)]
pub struct CalculatorService {
    pub state: State,
}

impl CalculatorService {
    async fn increment_counter(&self) {
        let mut count = self.state.write().await;
        *count += 1;

        println!("Request count: {}", *count);
    }
}

#[tonic::async_trait]
impl Calculator for CalculatorService {
    async fn add(
        &self,
        request: tonic::Request<proto::CalculationRequest>,
    ) -> Result<tonic::Response<proto::CalculationResponse>, tonic::Status> {
        self.increment_counter().await;

        let input = request.get_ref();
        let response = proto::CalculationResponse {
            result: input.a + input.b,
        };

        Ok(tonic::Response::new(response))
    }

    async fn divide(
        &self,
        request: tonic::Request<proto::CalculationRequest>,
    ) -> Result<tonic::Response<proto::CalculationResponse>, tonic::Status> {
        self.increment_counter().await;

        let input = request.get_ref();

        if input.b == 0 {
            return Err(tonic::Status::invalid_argument("cannot divide by zero"));
        }

        let response = proto::CalculationResponse {
            result: input.a / input.b,
        };

        Ok(tonic::Response::new(response))
    }
}

#[derive(Debug, Default)]
pub struct AdminService {
    pub state: State,
}

#[tonic::async_trait]
impl Admin for AdminService {
    async fn get_request_count(
        &self,
        _request: tonic::Request<proto::GetCountRequest>,
    ) -> Result<tonic::Response<proto::CounterResponse>, tonic::Status> {
        let count = self.state.read().await;

        let response = proto::CounterResponse { count: *count };

        Ok(tonic::Response::new(response))
    }
}

pub fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    let token: MetadataValue<_> = "Bearer token.dipesh.paudel".parse().unwrap();

    match req.metadata().get("authorization") {
        Some(auth_token) if token == auth_token => Ok(req),
        _ => Err(Status::unauthenticated("no valid auth token")),
    }
}
