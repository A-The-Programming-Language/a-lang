// Simplest A-lang HTTP Server
print("Starting simple server...")

// Create server
server = createServer()

// Add route
server.get("/", (req, res) => {
    res.text("Hello from A-lang!")
})

// Start server
server.listen(3000)
print("Server running on port 3000")
