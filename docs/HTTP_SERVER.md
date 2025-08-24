# HTTP Server Guide

## Overview

Jing provides built-in HTTP server capabilities that allow you to create web servers directly in your Jing programs. The HTTP server supports custom route handlers, multiple concurrent servers, and built-in endpoints.

## Quick Start

```jing
// Start a server and get its handle
let server = start_http_server(8080);
print(server); // "server_8080"

// The server is now running with built-in endpoints:
// GET  /        - Welcome page
// GET  /status  - Server status (JSON)
// POST /echo    - Echo service
```

Test your server:
```bash
curl http://127.0.0.1:8080/
curl http://127.0.0.1:8080/status
curl -X POST http://127.0.0.1:8080/echo -d "Hello World"
```

## Core Functions

### `start_http_server(port)`
Starts an HTTP server on the specified port (8000-9999) and returns a server handle.

```jing
let api_server = start_http_server(8080);
let admin_server = start_http_server(9000);
```

**Returns**: Server handle string (e.g., "server_8080")
**Note**: If a server is already running on the port, returns the existing handle.

### `stop_http_server(server_handle)`
Stops the HTTP server using its handle.

```jing
let server = start_http_server(8080);
let result = stop_http_server(server);
print(result); // "HTTP server server_8080 stopped"
```

### `list_http_servers()`
Lists all currently running HTTP servers.

```jing
let servers = list_http_servers();
print(servers);
// Output: "Running HTTP servers:\n  server_8080: running (port 8080)"
```

## Custom Route Handlers

### `http_register_handler(server_handle, method, path, handler_function)`
Registers a Jing function to handle specific HTTP routes.

```jing
let server = start_http_server(8080);

// Register route handlers
http_register_handler(server, "GET", "/users", "get_users");
http_register_handler(server, "POST", "/users", "create_user");
http_register_handler(server, "GET", "/users/profile", "get_user_profile");
```

**Parameters**:
- `server_handle`: Handle returned by `start_http_server()`
- `method`: HTTP method (GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS)
- `path`: URL path (e.g., "/users", "/api/data")
- `handler_function`: Name of Jing function to call (as string)

### Handler Function Pattern

```jing
// Define your handler functions
fn get_users(request) {
    let users = "[{\"id\": 1, \"name\": \"Alice\"}]";
    return http_response(200, "application/json", users);
}

fn create_user(request) {
    // TODO: Parse request body when request object is implemented
    let response = "{\"id\": 123, \"status\": \"created\"}";
    return http_response(201, "application/json", response);
}

// Start server and register handlers
let server = start_http_server(8080);
http_register_handler(server, "GET", "/users", "get_users");
http_register_handler(server, "POST", "/users", "create_user");
```

**Current Status**: Handler registration works, but function calling is not yet implemented. Custom routes currently return placeholder responses.

## HTTP Responses

### `http_response(status, content_type, body)`
Creates formatted HTTP response strings for use in handler functions.

```jing
// JSON response
let json_resp = http_response(200, "application/json", "{\"message\": \"OK\"}");

// HTML response
let html_resp = http_response(200, "text/html", "<h1>Hello World</h1>");

// Error response
let error_resp = http_response(404, "text/plain", "Page not found");

// Other content types
let xml_resp = http_response(200, "application/xml", "<root>data</root>");
```

**Parameters**:
- `status`: HTTP status code (200-599)
- `content_type`: MIME type string
- `body`: Response body content

## Multiple Servers Example

```jing
// Start multiple servers for different purposes
let api_server = start_http_server(8080);
let admin_server = start_http_server(9000);
let static_server = start_http_server(8888);

// Register different handlers for each server
http_register_handler(api_server, "GET", "/api/users", "api_get_users");
http_register_handler(api_server, "POST", "/api/users", "api_create_user");

http_register_handler(admin_server, "GET", "/admin/stats", "admin_get_stats");
http_register_handler(admin_server, "POST", "/admin/reset", "admin_reset");

http_register_handler(static_server, "GET", "/files", "serve_static_files");

// Each server operates independently
print(list_http_servers());

// Stop servers when done
stop_http_server(api_server);
stop_http_server(admin_server);
stop_http_server(static_server);
```

## Built-in Endpoints

All servers automatically provide these endpoints:

### GET /
Returns welcome page with server information and available endpoints.

### GET /status
Returns JSON status information:
```json
{
  "status": "ok",
  "server": "Jing HTTP Server",
  "timestamp": 1640995200
}
```

### POST /echo
Echo service that returns the request body as JSON:
```json
{
  "echo": "your request body",
  "method": "POST", 
  "path": "/echo"
}
```

## Testing Your Servers

### Built-in Endpoints
```bash
# Welcome page
curl http://127.0.0.1:8080/

# Server status
curl http://127.0.0.1:8080/status

# Echo service
curl -X POST http://127.0.0.1:8080/echo \
     -H "Content-Type: text/plain" \
     -d "Hello from client"
```

### Custom Handlers (when implemented)
```bash
# Test your custom routes
curl http://127.0.0.1:8080/users
curl -X POST http://127.0.0.1:8080/users -d '{"name":"Bob"}'
curl http://127.0.0.1:8080/users/profile
```

## Common Patterns

### REST API Pattern
```jing
let api = start_http_server(8080);

// RESTful routes
http_register_handler(api, "GET", "/users", "list_users");
http_register_handler(api, "POST", "/users", "create_user");
http_register_handler(api, "GET", "/users/123", "get_user");
http_register_handler(api, "PUT", "/users/123", "update_user");
http_register_handler(api, "DELETE", "/users/123", "delete_user");
```

### Error Handling Pattern
```jing
fn safe_handler(request) {
    // Validate request
    if (!is_valid_request(request)) {
        return http_response(400, "application/json", 
                           "{\"error\": \"Invalid request\"}");
    }
    
    // Process request
    let result = process_request(request);
    
    // Return success response
    return http_response(200, "application/json", result);
}
```

### Multi-Server Architecture
```jing
// Separate concerns across different ports
let public_api = start_http_server(8080);  // Public API
let admin_api = start_http_server(8081);   // Admin interface
let webhooks = start_http_server(8082);    // Webhook receivers

// Different authentication/handlers for each
http_register_handler(public_api, "GET", "/", "public_home");
http_register_handler(admin_api, "GET", "/", "admin_dashboard"); 
http_register_handler(webhooks, "POST", "/github", "handle_github_webhook");
```

## Implementation Status

### ✅ What Works Now
- Starting/stopping HTTP servers on multiple ports
- Server handle management
- Built-in endpoints (/, /status, /echo)
- Handler registration with routes
- HTTP response formatting
- Multiple concurrent servers

### 🔄 In Progress  
- Custom handler function calling
- Request object parsing and passing to handlers
- Response object processing from handlers

### 📋 Future Features
- Route parameters (e.g., `/users/{id}`)
- Query parameter parsing
- Request body parsing (JSON, form data)
- Custom headers
- Middleware support
- Static file serving
- WebSocket support

## Error Handling

Common errors and solutions:

**Port already in use**: If you try to start a server on a port that's already in use by another process, you'll get an error. Use a different port or stop the existing server.

**Invalid port range**: Ports must be between 8000-9999. This restriction helps avoid conflicts with system services.

**Handler not found**: When custom handler calling is implemented, ensure your handler function names match exactly what you registered.

**Invalid HTTP methods**: Only GET, POST, PUT, DELETE, PATCH, HEAD, and OPTIONS are supported.

## Best Practices

1. **Use meaningful handler names**: Choose descriptive function names that clearly indicate what the handler does.

2. **Handle errors gracefully**: Always return appropriate HTTP status codes and error messages.

3. **Separate concerns**: Use different servers for different types of functionality (API vs admin vs webhooks).

4. **Validate input**: Check request data before processing (when request parsing is implemented).

5. **Return consistent responses**: Use the same response format across similar endpoints.

6. **Set appropriate content types**: Use "application/json" for JSON, "text/html" for HTML, etc.

## Examples

See the `examples/` directory for complete working examples:
- `examples/simple_http_server.jing` - Basic server setup
- `examples/multiple_servers.jing` - Multi-server configuration  
- `examples/http_handlers.jing` - Route handler patterns
- `examples/modern_api_demo.jing` - REST API example
