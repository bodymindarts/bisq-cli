include!("generated/io.bisq.protobuffer.rs");

pub fn get_version() {
    println!("GET VERSION");
    // let env = Arc::new(EnvBuilder::new().build());
    // let ch = ChannelBuilder::new(env).connect("localhost:8888");
    // let client = service::GetVersionClient::new(ch);

    // let req = grpc::GetVersionRequest::default();
    // match client.get_version(&req) {
    //     Err(_) => eprintln!("Error contacting daemon"),
    //     Ok(reply) => println!("Daemon is on version: v{}", reply.version),
    // }
}
