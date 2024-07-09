# HTTP Request examples

This project demonstrates both asynchronous and synchronous HTTP requests in various languages (except for JavaScript, since Node.js doesn't support synchronous HTTP requests due to lack of XMLHttpRequest support).

Use the provided Makefile to run the examples.

- For C, use the libcurl library
- For Rust, use the [reqwest](https://docs.rs/reqwest/latest/reqwest/) create
- The JavaScript example uses Fetch, which is the `async`-compatible replacement for XMLHttpRequest
- A well-known alternative to the JavaScript Fetch API is [Axios](https://axios-http.com/docs/intro)
