extern crate grpc;
extern crate protobuf;

use models::example::*;
use models::example_grpc::*;
use grpc::*;
// implement service
pub struct ExampleServiceImpl;

impl ExampleService for ExampleServiceImpl {
    fn add(&self, _o: RequestOptions, _p: AddRequest) -> SingleResponse<AddResponse> {
        let mut response = AddResponse::new();
        response.result = &_p.number1 + &_p.number2;
        grpc::SingleResponse::completed(response)
    }
}
