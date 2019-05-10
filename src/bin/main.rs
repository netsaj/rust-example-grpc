#[allow(dead_code)]
#[allow(unused_imports)]
use std::thread;
use models::example_grpc::ExampleServiceServer;
use service::myexample::ExampleServiceImpl;
use models::example::AddRequest;
use models::example_grpc::ExampleService;

fn main() {
    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(8080);
    server.add_service(ExampleServiceServer::new_service_def(ExampleServiceImpl));
    server.http.set_cpu_pool_threads(4);
    let _server = server.build().expect("bin");

    loop {
        thread::park();
    }
}



