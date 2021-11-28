use anyhow::{Context, Result};
use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::lovers_server::{Lovers};
use hello_world::{HelloReply, HelloRequest};

pub mod hello_world {
    //#[path = "grpc_proto.hello_world.rs"]
    //pub mod hello_world;
    tonic::include_proto!("helloworld");
    tonic::include_proto!("music_lovers");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    let greeter = GreeterServer::new(greeter);

    Server::builder()
    .accept_http1(true)
        .add_service(greeter)
        .serve(addr)
        .await?;

    Ok(())
}
