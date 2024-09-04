# Dummy HTTP server

This is an HTTP server that will always return `200 OK` for any request. It can
therefore be used to test a fuzzer with any specification to see if it works
without actually getting the corresponding target to run, or to test features
that are not implemented in any of our targets.

## Usage

The easiest way to use this server is by executing `cargo run` in your shell. If
you need the server to listen on a specific port, use the `--port` flag:

```sh
cargo run -- --port 8081
```

The default port for listening is 8080 (and this is printed at program startup).

The server will print a line

```
Request: GET /api/abc HTTP/1.1
```

for each request made against it. This allows you to see the method, path, and
any parameters in the path. If you need to see the entire request, use the
`--verbose` flag:

```sh
cargo run -- --verbose
```
