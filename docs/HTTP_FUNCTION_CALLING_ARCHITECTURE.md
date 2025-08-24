# HTTP Handler Function Calling Architecture Challenge

## Current Architecture Problem

```
┌───────────────────┐       ┌───────────────────────────┐
│   Jing VM         │       │    HTTP Server            │
│   (Main Thread)   │       │    (Separate Threads)     │
│                   │       │                           │
│  ┌───────────────┐│       │  ┌─────────────────────┐  │
│  │ Functions     ││       │  │ handle_request()    │  │  
│  │               ││       │  │                     │  │
│  │ fn get_users  ││   ?   │  │ andler_name =       │  │
│  │ fn create_user│◄───────┼─►│ "get_users"         │  │
│  │ fn delete_user││       │  │                     │  │
│  │               ││       │  │ // How to call      │  │
│  └───────────────┘│       │  │ // get_users(req)?  │  │
│                   │       │  └─────────────────────┘  │
└───────────────────┘       └───────────────────────────┘
      ↑                                    ↑
      │                                    │
   Functions                          Function Names
   Available                          (Strings Only)
```

## The Missing Link

The HTTP server has:
- ✅ Function names as strings: `"get_users"`
- ❌ No way to execute those functions
- ❌ No access to VM environment

## Possible Solutions

### Solution 1: Function Registry (Recommended)
Store function references/closures that can be called from HTTP threads:

```rust
// Instead of HashMap<String, String>
static HANDLERS: OnceLock<Mutex<HashMap<String, Box<dyn Fn(String) -> String + Send + Sync>>>> = ...

// Registration would store callable function
handlers.insert(key, Box::new(move |request| {
    // Call actual Jing function somehow
}));
```

### Solution 2: Message Passing
HTTP threads send requests to VM thread via channels:

```rust
// HTTP thread sends message
let (sender, receiver) = channel();
sender.send(FunctionCall { name: "get_users", request: req }).unwrap();

// VM thread processes function calls
loop {
    if let Ok(call) = receiver.try_recv() {
        let result = vm.call_function(call.name, call.request);
        // Send result back...
    }
}
```

### Solution 3: Shared VM Instance
Create a thread-safe VM that can be accessed from HTTP threads:

```rust
static SHARED_VM: OnceLock<Arc<Mutex<VM>>> = OnceLock::new();

// HTTP thread calls function
let vm = SHARED_VM.get().unwrap();
let mut vm_guard = vm.lock().unwrap();
let result = vm_guard.call_function_by_name("get_users", request);
```

## Current Implementation Status

```rust
// src/builtins/http.rs line ~491
if let Some(handler_name) = custom_handler {
    // TODO: This is where function calling needs to happen
    // handler_name = "get_users" (string)
    // Need to somehow call the actual Jing function
    
    let response_body = format!(
        "Custom handler registered for {} {} (handler calling not yet implemented)",
        method, path
    );
    return Ok(Response::new(Full::new(Bytes::from(response_body))));
}
```

The placeholder response shows that function lookup works, but function **execution** is not implemented.

## Next Steps

1. Choose an architecture solution
2. Implement function calling mechanism  
3. Handle request/response serialization
4. Add error handling for function calls
5. Test with real handler functions
