# Generate Code from Proto Files

This is a tutorial for generating code from Proto files using gRPC-Rust. This tutorial will utilize the RouteGuide example.

First, follow the instructions here: https://github.com/hyperium/tonic/blob/master/protoc-gen-rust-grpc/README.md

## Generating client and server code

Next we need to generate the gRPC client and server interfaces from our `.proto`
service definition. 

### Dependencies
Edit `Cargo.toml` and add the dependency we'll need for this example, which is tonic-protobuf-build:

```console
cargo add tonic-protobuf-build
```

### Compiling and Building Proto  
Create a `build.rs` file at the root of your crate. A build.rs script is a Rust program that Cargo executes before compiling your main project. Its purpose is to perform tasks like generating source code, linking to non-Rust libraries, or setting environment variables that influence the build process.

In this case, we will be putting the command to compile and build the `.proto` file in build.rs. We will use gRPC's tonic_protobuf_build crate to generate code from the `.proto` file.
```rust
fn main() {
    tonic_protobuf_build::CodeGen::new()
    .include("proto")
    .inputs(["routeguide.proto"])
    .output_dir("generated")
    .compile()
    .unwrap();
}
```
Now, run 
```shell
$ cargo build
```

## Bringing Generated Code into Scope

The generated code is placed inside our target directory, in a location defined by the `output_dir`.

We can use gRPC's `include_generated_proto` macro to bring the generated code into scope:

```rust
pub mod routeguide {
    grpc::include_generated_proto!("generated", "routeguide");
}
```

**Note**: The token passed to the `include_generated_proto` macro (in our case "routeguide") is the name of
the package declared in our `.proto` file, not a filename, e.g "routeguide.rs".

With this in place, we can stub out our service implementation:




