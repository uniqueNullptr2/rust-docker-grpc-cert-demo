
use std::time::Duration;

use anyhow::Result;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};
use crate::hello::{greeter_client::GreeterClient,HelloRequest};
pub mod hello {
    tonic::include_proto!("hello");
}

//https://betterprogramming.pub/trusted-self-signed-certificate-and-local-domains-for-testing-7c6e6e3f9548
#[tokio::main]
async fn main() -> Result<()> {
    let cert = std::fs::read_to_string("cert/myca.pem")?;

    let channel = Channel::from_static("https://localhost:8000")
        .tls_config(ClientTlsConfig::new()
            .ca_certificate(Certificate::from_pem(&cert))
            .domain_name("localhost".to_string()))?
        .timeout(Duration::from_secs(5))
        .rate_limit(5, Duration::from_secs(1))
        .concurrency_limit(256)
        .connect()
        .await?;
        let mut client = GreeterClient::new(channel);
    // creating a new Request
        let request = tonic::Request::new(
            HelloRequest {
                name:String::from("anshul")
            },
        );
    // sending request and waiting for response
        let response = client.say_hello(request).await?;
        println!("RESPONSE={:?}", response);
        Ok(())
    }