// HTTP Server API Demo
// Demonstrates server creation, routing, and database integration

print("╔════════════════════════════════════════════════╗")
print("║     A-lang HTTP Server API Demo               ║")
print("║     Express-like Server Framework              ║")
print("╚════════════════════════════════════════════════╝")
print("")

print("🔧 Initializing server...")
server = createServer()

// Middleware
print("📦 Loading middleware...")
server.use(cors())
print("   ✓ CORS enabled")
print("")

// API Routes
print("🛣️  Registering routes...")

// GET /api/users - List all users
server.get("/api/users", (req, res) => {
    print("  📡 GET /api/users - Fetching users...")
    users = db.query("SELECT * FROM users")
    res.json(users)
})
print("   ✓ GET  /api/users")

// POST /api/users - Create new user
server.post("/api/users", (req, res) => {
    print("  📡 POST /api/users - Creating user...")
    id = db.insert("users", req.body)
    res.status(201).json({id: id, message: "User created"})
})
print("   ✓ POST /api/users")

// GET /api/users/:id - Get specific user
server.get("/api/users/:id", (req, res) => {
    print("  📡 GET /api/users/:id - Fetching user...")
    userId = req.params.id
    user = db.query("SELECT * FROM users WHERE id = " + userId)
    res.json(user)
})
print("   ✓ GET  /api/users/:id")

// PUT /api/users/:id - Update user
server.put("/api/users/:id", (req, res) => {
    print("  📡 PUT /api/users/:id - Updating user...")
    userId = int(req.params.id)
    success = db.update("users", userId, req.body)
    res.json({success: success, message: "User updated"})
})
print("   ✓ PUT  /api/users/:id")

// DELETE /api/users/:id - Delete user
server.delete("/api/users/:id", (req, res) => {
    print("  📡 DELETE /api/users/:id - Deleting user...")
    userId = int(req.params.id)
    success = db.delete("users", userId)
    res.json({success: success, message: "User deleted"})
})
print("   ✓ DELETE /api/users/:id")

// Health check endpoint
server.get("/api/health", (req, res) => {
    print("  💚 GET /api/health - Health check...")
    res.json({
        status: "healthy",
        timestamp: timestamp(),
        uptime: "running"
    })
})
print("   ✓ GET  /api/health")

// Root endpoint
server.get("/", (req, res) => {
    print("  🏠 GET / - Root endpoint...")
    res.json({
        name: "A-lang API Server",
        version: "1.0",
        endpoints: [
            "GET /api/users",
            "POST /api/users",
            "GET /api/users/:id",
            "PUT /api/users/:id",
            "DELETE /api/users/:id",
            "GET /api/health"
        ]
    })
})
print("   ✓ GET  /")

print("")
print("✅ All routes registered")
print("")

// Start server
port = 3000
print("🚀 Starting server on port " + str(port) + "...")
server.listen(port)

print("")
print("╔════════════════════════════════════════════════╗")
print("║     Server is running!                         ║")
print("║     Available endpoints:                       ║")
print("║     - http://localhost:3000/                   ║")
print("║     - http://localhost:3000/api/users          ║")
print("║     - http://localhost:3000/api/health         ║")
print("╚════════════════════════════════════════════════╝")
