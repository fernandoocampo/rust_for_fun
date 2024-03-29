# Rust for fun

this is a repository to learn Rust.

## HTTP server

0. start server just running

```shell
➜  rust_for_fun git:(main) cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.37s
     Running `target/debug/rust_for_fun`
> Hello, starting on 1337
```

1. GET people

```shell
➜  ~ curl http://localhost:1337/people
{"id":"1","name":"Fernando"}
```

2. OPTIONS people

```shell
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

3. ADD people

to add a person run the following command

```shell
curl --location --request POST 'http://localhost:1337/people' \
--header 'content-type: application/json' \
--data '{ "id": "3", "name": "aebutius" }'
```

4. UPDATE people

to update an existing person run the following command

```shell
curl --location --request PUT 'http://localhost:1337/people/3' \
--header 'Content-Type: application/json' \
--data '{"id": "3", "name": "Aebetius" }'
```

5. DELETE people

to delete an existing person run the following command

```shell
curl --location --request DELETE 'http://localhost:1337/people/2'
```

6. ADD pet

to add a pet using data-urlencode parameters

```shell
curl --location --request POST 'http://localhost:1337/pets' \
--header 'content-type: application/x-www-form-urlencoded' \
--data-urlencode 'name=ivana' \
--data-urlencode 'personID=1'
```
