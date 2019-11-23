include!("generated/io.bisq.protobuffer.rs");

pub async fn get_version() -> Result<String, Box<dyn std::error::Error>> {
    let mut client = client::GetVersionClient::connect("http://localhost:8888").await?;
    let request = tonic::Request::new(GetVersionRequest {});
    let response = client.get_version(request).await?;

    Ok(response.into_inner().version)
}
