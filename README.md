## What I would change

After writing the framework I have a better understanding of rust ownership. **Next time:** I would cut out the request context struct and just pass ownership of the request and response objects between functions, including user route functions. Not that the design is bad, but it feels a little awkward as a developer to need to clone a request, modify it and the set it again on the ctx.

What I actually mean is internally it is awkward, for the end user it is not so bad as generally they will be constructing a new `Response` instance and not modifying the request.



## High level function

```mermaid
flowchart LR
    
    Client <---> TcpListener
    TcpListener <---> ConnectionStream
    ConnectionStream <---> RouteHandling

```
## Data Flow

```mermaid
sequenceDiagram
    Connection ->> Context: instantiate

    activate Context
    Context ->> RawMiddleware: Call
    RawMiddleware ->> Context: 
    
    Context ->> Middleware: Call
    Middleware ->> Context: 
    
    Context ->> RouteMatcher: get route
    RouteMatcher ->> Context: 

    Context ->> RouteMiddleware: Call
    RouteMiddleware ->> Context: 

    Context ->> RouteFunc: get resulte
    RouteFunc ->> Context: 

    Context ->> Connection: write

    deactivate Context
```

Implementation Order:
- Define traits
- Define default request / responses
- Define default route
- Define request parsing
- define route matching

## Route Handling

```mermaid
flowchart LR
    ConnectionStream -- bytes --> RawMiddleware
    RawMiddleware -- bytes --> Ser/DeSer
    Ser/DeSer -- RequestObject --> Middleware
    Middleware -- RequestObject --> RouteMatcher
    RouteMatcher -- RequestObject --> Func
    Func -- ResponseObject --> Middleware
    Middleware -- ResponseObject --> Ser/DeSer
    Ser/DeSer -- bytes --> RawMiddleware
    RawMiddleware -- bytes --> ConnectionStream
```

### raw middleware:

- do something with the raw data after reading from the tcp socket
- do something with the raw data before writing to the tcp socket

Use cases:
- metrics e.g. num bytes

```mermaid
classDiagram
    class RawMiddleware{
        <<Trait>>
        -bytes request_bytes
        -bytes response_bytes
        +request(bytes) bytes
        +response(bytes) bytes
    }

    class RequestMiddleware{
        <<Trait>>
        -Request request
        -Response response
        +request(&mut request)
        +response(&request, &mut response)
    }


    class Route{
        <<Trait>>
        -String path
        -RequestMiddleware[] middleware 
        -Func handler
        -Method[] methods
        +handle(RequestContext)
    }
```

```mermaid
classDiagram
    class Request{
        <<Trait>>
        -Method method
        -Vec~TupleString~ path_params
        -Vec~TupleString~ query_params
        -Vec~TupleString~ headers
        -Option~bytes~ body
        +get_method() Method
        +get_path() String
        +get_path_param(str) Option~String~        
        +get_query_param(str) Option~String~
        +get_header(String) Option~String~        
        +get_body() Option~bytes~
    }

    class Response{
        <<Trait>>
        -u8 status_code
        -Vec~StringTuple~ headers 
        -bytes body
        +set_status_code(u8)
        +set_header(String, String)
        +set_headers(Vec~StringTuple~)
        +set_body(Bytes)
    }

    class RequestContext{
        <<Trait>>
        -mut impl Request request
        -mut impl Response response
        +get_mut_request() &mut Request
        +get_request() &Request
        +get_mut_response() &mut Response
        +get_response()
    }

```

### request middleware:

- do something with a request object before calling route function
- do something with a response object before sending writing to tcp socket

Use cases:
- Add authentication data to the request
- Transform RequestObject into a custom type
- ErrorHandling

### Route Middleware

same as request middle ware but registered per route

Registering middleware for a group of routes 

### Request object

### Response object

### Request Context

Encapsulate all data for the requests lifecycle
- request object
- response object
- connection metadata
- metrics

### Router

### Route

Message parsing
- http lib


- server / application lib
