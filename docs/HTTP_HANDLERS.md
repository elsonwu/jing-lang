# HTTP Handler Functions Guide

## Overview

The Jing HTTP server supports custom handler functions that can be registered to handle specific routes. This document explains how to define and use these handlers.

## Current Implementation Status

**✅ What Works Now:**
- Defining handler functions in Jing code
- Server handle-based route management  
- Handler function registration with routes
- Built-in HTTP response creation
- Multiple servers with separate handler sets

**🔄 What's In Progress:**
- Automatic function calling when routes are matched
- Request object parsing and passing to handlers
- Response object processing from handler returns
- Route parameter extraction (e.g., `/users/{id}`)

## How to Define Handler Functions

### Basic Pattern

```jing
// Define a handler function
fn my_handler(request) {
    // Process the request
    print("Handler called with: " + request);
    
    // Create response  
    let response_data = "Hello World!";
    let response = http_response(200, "text/plain", response_data);
    return response;
}
```

### Handler Function Signature

Handler functions should follow this pattern:
- **Name**: Any valid Jing function name
- **Parameter**: Single `request` parameter (will contain request details)
- **Return**: HTTP response created with `http_response(status, content_type, body)`

### Complete Example

```jing
// 1. Define handler functions
fn get_users(request) {
    let users = "[{\"id\": 1, \"name\": \"Alice\"}]";
    return http_response(200, "application/json", users);
}

fn create_user(request) {
    // TODO: Parse request body to get user data
    let new_user = "{\"id\": 123, \"status\": \"created\"}";
    return http_response(201, "application/json", new_user);
}

// 2. Start server
let api_server = start_http_server(8080);

// 3. Register handlers
http_register_handler(api_server, "GET", "/users", "get_users");
http_register_handler(api_server, "POST", "/users", "create_user");
```

## Request Object (Planned)

When fully implemented, the `request` parameter will contain:

```jing
// Example request object structure (planned)
{
    "method": "GET",
    "path": "/users/123", 
    "headers": {
        "Content-Type": "application/json",
        "User-Agent": "..."
    },
    "body": "request body content",
    "params": {
        "id": "123"  // extracted from route like /users/{id}
    },
    "query": {
        "limit": "10",  // from ?limit=10
        "offset": "0"
    }
}
```

## Response Creation

Use the `http_response()` function to create responses:

```jing
// Basic response
let response = http_response(200, "text/plain", "Hello World");

// JSON response
let json_data = "{\"message\": \"Success\"}";
let response = http_response(200, "application/json", json_data);

// Error response
let error = "{\"error\": \"User not found\"}";
let response = http_response(404, "application/json", error);

// Custom headers (planned feature)
let response = http_response(200, "text/html", "<h1>Hello</h1>", {
    "Cache-Control": "no-cache",
    "Custom-Header": "value"
});
```

## Route Patterns (Planned)

Future support for dynamic routes:

```jing
// Static routes (current)
http_register_handler(server, "GET", "/users", "get_users");

// Dynamic routes (planned)  
http_register_handler(server, "GET", "/users/{id}", "get_user_by_id");
http_register_handler(server, "POST", "/users/{id}/posts/{post_id}", "get_user_post");

// Wildcard routes (planned)
http_register_handler(server, "GET", "/static/*", "serve_static_files");
```

## Error Handling

```jing
fn safe_handler(request) {
    // Always include error handling
    if (!validate_request(request)) {
        return http_response(400, "application/json", "{\"error\": \"Bad request\"}");
    }
    
    // Process request...
    
    return http_response(200, "application/json", result);
}
```

## Testing Your Handlers

### Current Testing (Built-in Routes)

```bash
# Test built-in routes that work now
curl http://127.0.0.1:8080/           # Home page
curl http://127.0.0.1:8080/status     # Server status  
curl -X POST http://127.0.0.1:8080/echo -d "test"  # Echo service
```

### Future Testing (Custom Handlers)

```bash
# When custom handlers are fully implemented
curl http://127.0.0.1:8080/users                    # GET users
curl -X POST http://127.0.0.1:8080/users -d '{...}' # Create user
curl http://127.0.0.1:8080/users/123                # Get specific user
```

## Best Practices

1. **Always validate input**: Check request data before processing
2. **Use appropriate status codes**: 200 for success, 404 for not found, etc.
3. **Set correct Content-Type**: Use "application/json" for JSON, "text/plain" for text
4. **Handle errors gracefully**: Return proper error responses
5. **Keep handlers focused**: Each handler should do one thing well

## Examples

See the following example files:
- `examples/simple_handlers.jing` - Basic handler pattern
- `examples/handler_functions_demo.jing` - Complete API example  
- `examples/modern_api_demo.jing` - Modern framework-style API

## Implementation Roadmap

**Phase 1** (Current): Handler registration and server management ✅
**Phase 2** (Next): Function calling and request/response processing 🔄
**Phase 3** (Future): Route parameters and advanced features 📅
**Phase 4** (Future): Middleware support and advanced routing 📅
