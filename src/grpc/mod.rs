pub mod proto;
pub mod services;

use tonic::transport::Server;
use tonic_reflection::server::Builder;

use crate::grpc::proto::user::user_service_server::UserServiceServer;

use crate::db;
use crate::grpc::services::UserServiceImpl;

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;

    println!("gRPC server will start at {}", addr);

    let user_service = UserServiceImpl {
        db: db::connect().await,
    };

    let reflection_service = Builder::configure()
        .register_encoded_file_descriptor_set(tonic::include_file_descriptor_set!(
            "grpc_descriptor"
        ))
        .build_v1()?;

    let user_server = UserServiceServer::new(user_service);
    Server::builder()
        .add_service(user_server)
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}
