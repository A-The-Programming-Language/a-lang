// Basic A-lang HTTP Server
// Using only confirmed syntax from examples

print("Starting A-lang server...")

// Simple variables (no let/const needed)
port = 3000
message = "Hello from A-lang!"

// Function to handle requests
fn handleRequest(req, res) {
    print("Request received: " + req.path)
    return "Hello, World!"
}

// Print server info
print("Server will run on port: " + port)
print("Message: " + message)

// Simple response function
fn simpleResponse() {
    return "Hello from A-lang server!"
}

// Basic server logic (simplified)
print("Setting up routes...")

// Simulate server behavior
routes = {
    "/": "Hello, World!",
    "/health": "OK",
    "/about": "A-lang Basic Server v1.0"
}

print("Available routes:")
for (route in keys(routes)) {
    print("  " + route + " -> " + routes[route])
}

// Simulate starting server
print("Server starting on port " + port + "...")
print("Server is ready!")

// Basic HTTP-like responses
fn getResponse(path) {
    if (path == "/") {
        return "Hello, World!"
    } elif (path == "/health") {
        return "Server is healthy"
    } elif (path == "/about") {
        return "A-lang Basic Server - Simple and fast!"
    } else {
        return "404 - Not Found"
    }
}

// Test responses
testPaths = ["/", "/health", "/about", "/unknown"]
print("")
print("Testing responses:")
for (path in testPaths) {
    response = getResponse(path)
    print("  GET " + path + " -> " + response)
}

print("")
print("✅ Basic server logic working!")
print("📡 In a full implementation, this would listen on port " + port)
