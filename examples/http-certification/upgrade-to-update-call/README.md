# Upgrading HTTP calls to update calls

This guide walks through an example project that demonstrates how to use the ["Upgrade to Update call"](https://internetcomputer.org/docs/current/references/http-gateway-protocol-spec#upgrade-to-update-calls) feature of the HTTP gateway.

Since browsers are unable to directly interact with the ICP network, the HTTP gateway acts as a bridge between the two. The HTTP gateway forwards requests from clients to canisters and forwards responses from canisters back to clients. Before returning responses from the canister back to clients, the HTTP gateway verifies the certification of the response to ensure that they have not been tampered with.

Upgrading query calls to upgrade calls allows for the certification of any kind of dynamic response by leveraging ICP's consensus protocol without having to statically certify the response ahead of time. This is the simplest way to add _secure_ HTTP support to a canister.

A similarly simple yet more performant, but _insecure_ approach is to skip certification entirely. This is not recommended unless you are absolutely sure that certification really does not make sense for your canister. Check the ["Skipping certification for HTTP responses"](https://internetcomputer.org/docs/current/developer-docs/web-apps/http-compatible-canisters/skipping-certification-for-http-responses) guide for more details on how to do that.

## How it works

When the HTTP gateway receives a request from a client, it will forward the request to the target canister's `http_request` method as a query call. To upgrade this query call to an update call, the canister returns a response that sets the optional `upgrade` field to `opt true`. Omitting this field, or setting it to `opt false` will result in the HTTP Gateway treating the query call response as-is, without upgrading.

Upon receiving a response from the canister with the `upgrade` field set to `opt true`, the HTTP gateway will repeat the original request as an update call to the `http_request_update` method of the canister. The canister can then respond to the update call with any dynamic response and leverage the ICP consensus protocol for security. The certification resulting from putting this response through consensus will be verified by the HTTP Gateway to ensure it has not been tampered with.

## Rust

This example project features both Rust and Motoko code. If you would rather follow the Motoko version, you can skip this section and go straight to the [section covering Motoko](#motoko).

The Rust code is split into two functions: `http_request` and `http_request_update`. The `http_request` function is the entry point for the query call from the HTTP Gateway. It returns an `HttpResponse` with the `upgrade` field set to `Some(true)` (via the `build_update` method on the `HttpResponse::builder` struct). The `http_request_update` function is the entry point for the update call from the HTTP Gateway. It returns an `HttpUpdateResponse` with a custom status code and body.

```rust
use ic_cdk::*;
use ic_http_certification::{HttpResponse, HttpUpdateResponse};

#[query]
fn http_request() -> HttpResponse<'static> {
    HttpResponse::builder().with_upgrade(true).build()
}

#[update]
fn http_request_update() -> HttpUpdateResponse<'static> {
    HttpResponse::builder()
        .with_status_code(StatusCode::IM_A_TEAPOT)
        .with_body(b"I'm a teapot")
        .build_update()
}

```

## Motoko

The Motoko code is split into two functions: `http_request` and `http_request_update`. The `http_request` function is the entry point for the query call from the HTTP Gateway. It returns an `HttpResponse` with the `upgrade` field set to `Some(true)`. The `http_request_update` function is the entry point for the update call from the HTTP Gateway. It returns an `HttpUpdateResponse` with a custom status code and body.

```motoko
import Text "mo:base/Text";

actor Http {
  type HeaderField = (Text, Text);

  type HttpRequest = {
    method : Text;
    url : Text;
    headers : [HeaderField];
    body : Blob;
    certificate_version : ?Nat16;
  };

  type HttpUpdateRequest = {
    method : Text;
    url : Text;
    headers : [HeaderField];
    body : Blob;
  };

  type HttpResponse = {
    status_code : Nat16;
    headers : [HeaderField];
    body : Blob;
    upgrade : ?Bool;
  };

  type HttpUpdateResponse = {
    status_code : Nat16;
    headers : [HeaderField];
    body : Blob;
  };

  public query func http_request(_req: HttpRequest) : async HttpResponse {
    return {
      status_code = 200;
      headers = [];
      body = "";
      upgrade = ?true;
    };
  };

  public func http_request_update(_req: HttpUpdateRequest) : async HttpUpdateResponse {
    return {
      status_code = 418;
      headers = [];
      body = Text.encodeUtf8("I'm a teapot");
    };
  };
};
```

## Testing the canister

This example uses a Rust canister called `http_certification_upgrade_to_update_call_rust_backend` or a Motoko canister called `http_certification_upgrade_to_update_call_motoko_backend`.

To test the canister, you can use [`dfx`](https://internetcomputer.org/docs/current/developer-docs/getting-started/install) to start a local instance of the replica:

```shell
dfx start --background --clean
```

#### Testing the Rust canister

```shell
dfx deploy http_certification_upgrade_to_update_call_rust_backend
```

Make a request to the canister using curl:

```shell
curl -v http://localhost:$(dfx info webserver-port)?canisterId=$(dfx canister id http_certification_upgrade_to_update_call_rust_backend)
```

#### Testing the Motoko canister

```shell
dfx deploy http_certification_upgrade_to_update_call_motoko_backend
```

Make a request to the canister using curl:

```shell
curl -v http://localhost:$(dfx info webserver-port)?canisterId=$(dfx canister id http_certification_upgrade_to_update_call_motoko_backend)
```

## Resources

- [Example source code](https://github.com/dfinity/response-verification/tree/main/examples/http-certification/upgrade-to-update-call).
- [`ic-http-certification` crate](https://crates.io/crates/ic-http-certification).
- [`ic-http-certification` docs](https://docs.rs/ic-http-certification/latest/ic_http_certification).
- [`ic-http-certification` source code](https://github.com/dfinity/response-verification/tree/main/packages/ic-http-certification).
