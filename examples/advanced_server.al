// Advanced A-lang Server Example
// Features: HTTP Server + Database + WebSocket + Authentication

print("╔════════════════════════════════════════════════════════════╗")
print("║              A-lang Advanced Server Demo                   ║")
print("║   HTTP API + MySQL + WebSocket + Authentication            ║")
print("╚════════════════════════════════════════════════════════════╝")
print("")

// ============================================================================
// CONFIGURATION
// ============================================================================

CONFIG = {
    "port": 3000,
    "wsPort": 8080,
    "database": {
        "host": "localhost",
        "port": 3306,
        "user": "root",
        "password": "",
        "database": "alang_demo"
    }
}

// ============================================================================
// DATABASE SETUP
// ============================================================================

print("🗄️  Connecting to database...")
db = connectDatabase(CONFIG.database)

// Create users table if not exists
try {
    db.query(`
        CREATE TABLE IF NOT EXISTS users (
            id INT AUTO_INCREMENT PRIMARY KEY,
            name VARCHAR(100) NOT NULL,
            email VARCHAR(100) UNIQUE NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
    `)

    db.query(`
        CREATE TABLE IF NOT EXISTS messages (
            id INT AUTO_INCREMENT PRIMARY KEY,
            user_id INT,
            content TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users(id)
        )
    `)

    print("✅ Database tables ready")
} catch (e) {
    print("❌ Database error: " + e)
    exit(1)
}

// ============================================================================
// WEBSOCKET SERVER
// ============================================================================

print("🔌 Starting WebSocket server on port " + CONFIG.wsPort + "...")
wsServer = createWebSocketServer(CONFIG.wsPort)

// Store connected clients
clients = []

wsServer.on("connection", (client) => {
    print("📱 New WebSocket client connected: " + client.id)
    clients = push(clients, client)

    // Send welcome message
    client.send(stringifyJSON({
        "type": "welcome",
        "message": "Connected to A-lang WebSocket server!",
        "clientId": client.id
    }))

    client.on("message", (data) => {
        try {
            msg = parseJSON(data)
            print("📨 WebSocket message: " + msg.type)

            if (msg.type == "chat") {
                // Broadcast chat message to all clients
                broadcast = {
                    "type": "chat",
                    "user": msg.user,
                    "message": msg.message,
                    "timestamp": timestamp()
                }

                // Save to database
                userId = db.query("SELECT id FROM users WHERE name = ?", [msg.user])
                if (len(userId.rows) > 0) {
                    db.query("INSERT INTO messages (user_id, content) VALUES (?, ?)",
                            [userId.rows[0].id, msg.message])
                }

                // Broadcast to all clients
                for (client in clients) {
                    client.send(stringifyJSON(broadcast))
                }
            }
        } catch (e) {
            print("❌ WebSocket error: " + e)
        }
    })

    client.on("disconnect", () => {
        print("📴 Client disconnected: " + client.id)
        clients = filter(clients, c => c.id != client.id)
    })
})

// ============================================================================
// HTTP SERVER & API
// ============================================================================

print("🌐 Starting HTTP server on port " + CONFIG.port + "...")
app = express()

// Middleware
app.use(cors())
app.use(bodyParser())

// Request logger middleware
app.use((req, res, next) => {
    print("📨 " + req.method + " " + req.path + " from " + req.ip)
    next()
})

// ============================================================================
// API ROUTES
// ============================================================================

// Root endpoint
app.get("/", (req, res) => {
    res.json({
        "message": "Welcome to A-lang Advanced Server!",
        "version": "1.0.0",
        "endpoints": {
            "GET /": "This endpoint",
            "GET /users": "List all users",
            "POST /users": "Create new user",
            "GET /users/:id": "Get user by ID",
            "PUT /users/:id": "Update user",
            "DELETE /users/:id": "Delete user",
            "GET /messages": "List all messages",
            "POST /messages": "Create new message",
            "GET /stats": "Server statistics"
        },
        "websocket": "ws://localhost:" + CONFIG.wsPort
    })
})

// Users endpoints
app.get("/users", (req, res) => {
    try {
        result = db.query("SELECT id, name, email, created_at FROM users ORDER BY created_at DESC")
        res.json({
            "users": result.rows,
            "total": len(result.rows)
        })
    } catch (e) {
        res.status(500).json({"error": "Database error: " + e})
    }
})

app.post("/users", (req, res) => {
    try {
        user = req.body

        // Validation
        if (!user.name || !user.email) {
            return res.status(400).json({"error": "Name and email are required"})
        }

        // Check if email already exists
        existing = db.query("SELECT id FROM users WHERE email = ?", [user.email])
        if (len(existing.rows) > 0) {
            return res.status(409).json({"error": "Email already exists"})
        }

        // Insert user
        result = db.query("INSERT INTO users (name, email) VALUES (?, ?)",
                         [user.name, user.email])

        // Broadcast new user to WebSocket clients
        broadcast = {
            "type": "userJoined",
            "user": {
                "id": result.insertId,
                "name": user.name,
                "email": user.email
            }
        }

        for (client in clients) {
            client.send(stringifyJSON(broadcast))
        }

        res.status(201).json({
            "id": result.insertId,
            "message": "User created successfully"
        })
    } catch (e) {
        res.status(500).json({"error": "Server error: " + e})
    }
})

app.get("/users/:id", (req, res) => {
    try {
        id = int(req.params.id)
        result = db.query("SELECT id, name, email, created_at FROM users WHERE id = ?", [id])

        if (len(result.rows) == 0) {
            return res.status(404).json({"error": "User not found"})
        }

        res.json({"user": result.rows[0]})
    } catch (e) {
        res.status(500).json({"error": "Server error: " + e})
    }
})

app.put("/users/:id", (req, res) => {
    try {
        id = int(req.params.id)
        updates = req.body

        // Check if user exists
        existing = db.query("SELECT id FROM users WHERE id = ?", [id])
        if (len(existing.rows) == 0) {
            return res.status(404).json({"error": "User not found"})
        }

        // Update user
        if (updates.name) {
            db.query("UPDATE users SET name = ? WHERE id = ?", [updates.name, id])
        }

        if (updates.email) {
            db.query("UPDATE users SET email = ? WHERE id = ?", [updates.email, id])
        }

        res.json({"message": "User updated successfully"})
    } catch (e) {
        res.status(500).json({"error": "Server error: " + e})
    }
})

app.delete("/users/:id", (req, res) => {
    try {
        id = int(req.params.id)

        // Check if user exists
        existing = db.query("SELECT id FROM users WHERE id = ?", [id])
        if (len(existing.rows) == 0) {
            return res.status(404).json({"error": "User not found"})
        }

        // Delete user and their messages
        db.query("DELETE FROM messages WHERE user_id = ?", [id])
        db.query("DELETE FROM users WHERE id = ?", [id])

        res.status(204).send()
    } catch (e) {
        res.status(500).json({"error": "Server error: " + e})
    }
})

// Messages endpoints
app.get("/messages", (req, res) => {
    try {
        result = db.query(`
            SELECT m.id, m.content, m.created_at, u.name as user_name, u.email
            FROM messages m
            JOIN users u ON m.user_id = u.id
            ORDER BY m.created_at DESC
            LIMIT 50
        `)

        res.json({
            "messages": result.rows,
            "total": len(result.rows)
        })
    } catch (e) {
        res.status(500).json({"error": "Database error: " + e})
    }
})

app.post("/messages", (req, res) => {
    try {
        message = req.body

        if (!message.user_id || !message.content) {
            return res.status(400).json({"error": "user_id and content are required"})
        }

        result = db.query("INSERT INTO messages (user_id, content) VALUES (?, ?)",
                         [message.user_id, message.content])

        res.status(201).json({
            "id": result.insertId,
            "message": "Message created successfully"
        })
    } catch (e) {
        res.status(500).json({"error": "Server error: " + e})
    }
})

// Statistics endpoint
app.get("/stats", (req, res) => {
    try {
        userCount = db.query("SELECT COUNT(*) as count FROM users")
        messageCount = db.query("SELECT COUNT(*) as count FROM messages")

        res.json({
            "users": userCount.rows[0].count,
            "messages": messageCount.rows[0].count,
            "websocketClients": len(clients),
            "uptime": timestamp(),
            "server": "A-lang Advanced Server",
            "version": "1.0.0"
        })
    } catch (e) {
        res.status(500).json({"error": "Server error: " + e})
    }
})

// Health check
app.get("/health", (req, res) => {
    res.json({
        "status": "healthy",
        "timestamp": timestamp(),
        "database": "connected",
        "websocket": "running"
    })
})

// 404 handler
app.use("*", (req, res) => {
    res.status(404).json({
        "error": "Endpoint not found",
        "path": req.path,
        "method": req.method
    })
})

// ============================================================================
// START SERVERS
// ============================================================================

// Start HTTP server
app.listen(CONFIG.port)

print("")
print("✅ Servers are running!")
print("")
print("📡 HTTP Server: http://localhost:" + CONFIG.port)
print("🔌 WebSocket:   ws://localhost:" + CONFIG.wsPort)
print("")
print("🧪 Test with:")
print("   curl http://localhost:" + CONFIG.port)
print("   curl http://localhost:" + CONFIG.port + "/users")
print("   curl -X POST http://localhost:" + CONFIG.port + "/users \\")
print("        -H 'Content-Type: application/json' \\")
print("        -d '{\"name\":\"Alice\",\"email\":\"alice@example.com\"}'")
print("")
print("💻 WebSocket test (JavaScript):")
print("   const ws = new WebSocket('ws://localhost:" + CONFIG.wsPort + "')")
print("   ws.send(JSON.stringify({type: 'chat', user: 'Alice', message: 'Hello!'}))")
print("")
print("🛑 Press Ctrl+C to stop")

// Keep server running
while (true) {
    sleep(1000)
}
