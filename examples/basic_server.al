// Basic HTTP Server in A-lang
// Simple server that responds "Hello, World!" to all requests

print("Starting A-lang Basic HTTP Server...")
print("")

// Create HTTP server
server = createServer()

// Basic route - responds to all GET requests
server.get("/", (req, res) => {
    print("📨 Request received: " + req.method + " " + req.path)
    res.text("Hello, World!")
})

// Health check endpoint
server.get("/health", (req, res) => {
    print("💚 Health check requested")
    res.json({
        "status": "ok",
        "timestamp": timestamp(),
        "message": "A-lang server is running!"
    })
})

// About endpoint
server.get("/about", (req, res) => {
    print("ℹ️  About page requested")
    res.json({
        "name": "A-lang Basic Server",
        "version": "1.0.0",
        "language": "A-lang",
        "features": [
            "Simple syntax",
            "Built-in HTTP server",
            "Express-like routing",
            "JSON responses"
        ]
    })
})

// Catch-all route
server.get("*", (req, res) => {
    print("❓ Unknown route: " + req.path)
    res.status(404).json({
        "error": "Not Found",
        "message": "The requested endpoint does not exist",
        "path": req.path
    })
})

// Start server on port 3000
port = 3000
print("🚀 Starting server on port " + port + "...")
print("📡 Server endpoints:")
print("   GET  /        - Hello World")
print("   GET  /health  - Health check")
print("   GET  /about   - About info")
print("")
print("💡 Test with:")
print("   curl http://localhost:" + port)
print("   curl http://localhost:" + port + "/health")
print("   curl http://localhost:" + port + "/about")
print("")

// Listen for requests
server.listen(port)

// This will keep running until stopped with Ctrl+C
print("✅ Server is now running!")
print("🛑 Press Ctrl+C to stop the server")
