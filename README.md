# rust example grpc

basic example for implement grpc service using rust lang and external proto

## proto repository

the proto files is alocated in another repository and is imported how a git dependency.
* [https://github.com/netsaj/rust-example-grpc-proto](https://github.com/netsaj/rust-example-grpc-proto)

## install dependencies
```bash
$ cargo build
```

## run test
```bash
$ cargo test
```

## run server
```bash
$ cargo run --bin main
```

## run client
```bash
$ cargo run --bin client
```

## Authors

* Fabio Moreno <fabiomoreno@outlook.com>