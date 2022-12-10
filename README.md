# Rust for fun

this is a repository to learn Rust.

## HTTP server

1. GET people

```sh
➜  ~ curl http://localhost:1337/people
{"id":"1","name":"Fernando"}
```

2. OPTIONS people

```sh
curl -X OPTIONS localhost:1337/people -H "Access-Control-Request-Method: PUT" -H "Access-Control-Request-Headers: content-type" -H "Origin: https://not-origin.io" --verbose

*   Trying 127.0.0.1:1337...
* Connected to localhost (127.0.0.1) port 1337 (#0)
> OPTIONS /people HTTP/1.1
> Host: localhost:1337
> User-Agent: curl/7.84.0
> Accept: */*
> Access-Control-Request-Method: PUT
> Access-Control-Request-Headers: content-type
> Origin: https://not-origin.io
>
* Mark bundle as not supporting multiuse
< HTTP/1.1 200 OK
< access-control-allow-headers: content-type
< access-control-allow-methods: POST, PUT, DELETE, GET
< access-control-allow-origin: https://not-origin.io
< content-length: 0
< date: Thu, 08 Dec 2022 21:33:19 GMT
<
* Connection #0 to host localhost left intact
```