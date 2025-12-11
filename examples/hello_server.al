// Hello Server - Minimal A-lang HTTP Server Example

print("🚀 Starting Hello Server...")

// Create server
server = createServer()

// Simple hello route
server.get("/", (req, res) => {
    res.text("Hello, World!")
})

// Start server
server.listen(3000)
print("✅ Server running on http://localhost:3000")
