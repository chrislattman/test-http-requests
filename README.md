# HTTP Request examples

This project demonstrates both asynchronous and synchronous HTTP requests in various languages (except for JavaScript and C, since Node.js doesn't support synchronous HTTP requests due to lack of XMLHttpRequest support, and since C doesn't have native async support, and I didn't want to complicate things with libcurl using C++).

Use the provided Makefile to run the examples.

- The JavaScript example uses Fetch, which is the `async`-compatible replacement for XMLHttpRequest
- A well-known alternative to the JavaScript Fetch API is [Axios](https://axios-http.com/docs/intro)
