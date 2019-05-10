use models::example_grpc::{ExampleServiceClient, ExampleService};
use grpc::*;
use models::example::AddRequest;

fn main(){
    let client = ExampleServiceClient::new_plain(
        "127.0.0.1", 8080,  ClientConf::new()
    ).expect("client");

    let mut request = AddRequest::new();
    request.number1 = 10 as f64;
    request.number2 = 1.12 as f64;

    let response = client
        .add(grpc::RequestOptions::new(), request)
        .wait()
        .expect("add");
    println!("{:?}", response); // metadata and response
    let (_, add_response , _) = response;
    println!("result is: {:?}",add_response.result);
}