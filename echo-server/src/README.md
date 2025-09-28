## Simple HTTP echo server.

### About
Echo-server responds with the same data (body) that was contained in the request.
You can send get and post request. All other http verbs are not supported and will generate 405 method not allowed code.

### Covered topics
#### Web
1. How to build single-thread web server explains [here](https://doc.rust-lang.org/book/ch21-01-single-threaded.html).

#### Functional things
1. About closures read [here](https://doc.rust-lang.org/book/ch13-01-closures.html)
2. About iterators read [here](https://doc.rust-lang.org/book/ch13-02-iterators.html)

### How to use
#### Run
#### Send POST request
##### Request
```
curl -X POST -i --header "Content-Length: 5" --header "Content-Type: application/json" -d "{AAAAA}" localhost:8080
```
##### Response
```
HTTP/1.1 200 OK
Content-Length: 5
Content-Type: application/json
Connection: close

{AAAA}
```

#### Send POST request with actual content-length is less `content-length` header
##### Request
```
curl -X POST -i --header "Content-Length: 5" --header "Content-Type: application/json" -d "{A}" localhost:8080
```
##### Response
```
HTTP/1.1 408 Request Timeout
Content-Length: 46
Content-Type: application/json
Connection: close
```

#### Send GET request
##### Request
```
curl -X GET -i --header "Content-Length: 0" --header "Content-Type: application/json" localhost:8080
```

##### Response
```
HTTP/1.1 200 OK
Content-Length: 0
Connection: close
```

#### Send request with any other HTTP verb
##### Request
```
curl -X DELETE -i --header "Content-Length: 0" --header "Content-Type: application/json" localhost:8080
```

##### Response
```
HTTP/1.1 405 Method not allowed
Connection: close
```