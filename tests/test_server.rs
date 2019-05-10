#[cfg(test)]
pub mod tests {
    #[allow(dead_code)]
    #[allow(unused_imports)]
    use models::example_grpc::ExampleServiceServer;
    use service::myexample::ExampleServiceImpl;
    use models::example_grpc::ExampleService;
    use models::example::{AddRequest};

    #[test]
    pub fn test_add() {
        let _s = ExampleServiceImpl {};
        let mut request = AddRequest::new();
        request.number1 = 1 as f64;
        request.number2 = 2 as f64;
        let (_, response, _) = _s.add(grpc::RequestOptions::new(), request).wait().unwrap();
        assert_eq!(response.result, 3 as f64)
    }
}
