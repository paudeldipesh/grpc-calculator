mod grpc;
use grpc::{check_auth, proto, AdminService, CalculatorService, State};
use proto::admin_server::AdminServer;
use proto::calculator_server::CalculatorServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::1]:50051".parse()?;

    let state = State::default();

    let calculator = CalculatorService {
        state: state.clone(),
    };

    let admin = AdminService {
        state: state.clone(),
    };

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .accept_http1(true)
        .layer(tower_http::cors::CorsLayer::permissive())
        .add_service(service)
        .add_service(tonic_web::enable(CalculatorServer::new(calculator)))
        .add_service(AdminServer::with_interceptor(admin, check_auth))
        .serve(address)
        .await?;

    Ok(())
}
