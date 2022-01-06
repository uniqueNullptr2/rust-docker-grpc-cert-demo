use std::net::SocketAddr;
use tonic::{Request, Response, Status, transport::{Identity, Server, ServerTlsConfig}};
use crate::hello::{greeter_server::{GreeterServer, Greeter}, HelloReply, HelloRequest};

use anyhow::Result;

pub mod hello {
    tonic::include_proto!("hello");
}


#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(&self, request: Request<HelloRequest>) -> std::result::Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());
        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr: SocketAddr = "127.0.0.1:8000".parse()?;
    let greeter = MyGreeter::default();

    let cert = std::fs::read_to_string("cert/server.pem")?;
    let key = std::fs::read_to_string("cert/server.key")?;
    Server::builder()
        .tls_config(ServerTlsConfig::new()
                        .identity(Identity::from_pem(&cert, &key)))?
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}