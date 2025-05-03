<h1 align="center"> <code> dioxus-grpc </code> </h1>

`dioxus-grpc` provides a convenient way to use `gRPC` with `Dioxus`

## Example

```rs
fn app() -> Element {
    let req = use_signal(|| HelloRequest { name: String::new() });
    let greeter = use_greeter_service();

    rsx! {
        input {
            value: "{req().name}",
            oninput: move |event| req.write().name = event.value()
        }
        match &*greeter.say_hello(req).read() {
            Some(Ok(resp)) => rsx!{"[{resp.message}] - From server"},
            Some(Err(err)) => rsx!{"Couldn't get the name {err:#?}"},
            None => rsx!{"..."},
        }
    }
}
```

```proto
syntax = "proto3";

package helloworld;

service Greeter {
  rpc SayHello (HelloRequest) returns (HelloReply) {}
}

message HelloRequest {
  string name = 1;
}

message HelloReply {
  string message = 1;
}
```

_A complete example can be found here: [`./examples/`](https://github.com/tkr-sh/dioxus-grpc/blob/main/examples)_


## How to use ?

To use it, you will need to also use `tonic-build` and disable the `transport` feature. Therefore, something like:

```toml
[build-dependencies]
    dioxus-grpc = "*"
    tonic-build = { version = "0.13", default-features = false, features = ["prost"] }
```

But, you will also need to import some runtime dependencies:

```toml
[dependencies]
    dioxus = { version = "0.6", features = ["web"] }
    tonic = { version = "0.13", default-features = false, features = ["codegen", "prost"] }
    prost = "0.13"
    tonic-web-wasm-client = "0.7"
```

Once this is done, you can call [`generate_hooks`]() in `./build.rs`. See the [examples](https://github.com/tkr-sh/dioxus-grpc/blob/main/examples) for more details.

