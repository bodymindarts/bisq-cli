use grpcio::{ChannelBuilder, EnvBuilder};
use std::sync::Arc;

mod pb {
    include!("generated/pb.rs");
}

mod grpc {
    include!("generated/grpc.rs");
}

mod service {
    include!("generated/grpc_grpc.rs");
}

pub fn get_version() {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:8888");
    let client = service::GetVersionClient::new(ch);

    let req = grpc::GetVersionRequest::default();
    match client.get_version(&req) {
        Err(_) => eprintln!("Error contacting daemon"),
        Ok(reply) => println!("Daemon is on version: v{}", reply.version),
    }
}
