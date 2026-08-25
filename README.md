# HTTP Stub
Lightweight HTTP stub server with a web-based interface, developed in Rust.

## Installation
Download the executable for your machine from the GitHub Releases page.

## Usage
Run
```
./http-stub [port]
```

The web UI will be available at the root of the selected port, defaulting to 8080.

### Behaviour
Immediately returns `"Hello, World!"` with status `200` on all unconfigured endpoints. Responses may be configured per endpoint through the web interface. 

### Persistence
A given configuration can be downloaded as JSON and loaded back into the application at any time. Loading a configuration replaces the current configuration.
```
[
  {
    "method": "GET",
    "path": "/hello",
    "response_code": 200,
    "body": {
      "Text": "Hello, world!"
    }
  }
]
```
