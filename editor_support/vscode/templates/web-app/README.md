# Nova Web Application

A simple web server example in Nova demonstrating object-oriented programming and HTTP routing concepts.

## Files

- `main.nova` - Main application with WebServer class and route handlers

## Features Demonstrated

- **Classes and Objects**: `WebServer` class with constructor and methods
- **Object Methods**: `get()`, `start()`, `handleRequests()`
- **Import System**: Importing HTTP and filesystem modules
- **Routing**: Simple route registration and handling
- **String Interpolation**: Dynamic response generation
- **For-in Loops**: Iterating over routes and arrays
- **Function Definitions**: Handler functions for different routes

## Running

To run this web application:

```bash
nova main.nova
```

## Architecture

```
WebServer
├── constructor(port) - Initialize server with port
├── get(path, handler) - Register GET route
├── start() - Start the server
└── handleRequests() - Process incoming requests
```

## Routes

- `/` - Home page
- `/api/users` - Users API endpoint
- `/about` - About page

## Next Steps

- Add POST, PUT, DELETE methods
- Implement middleware system
- Add request/response objects
- Connect to a real database
- Add authentication and sessions