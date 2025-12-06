// ===========================================================================
// A-lang REST API Example with MySQL Database
// ===========================================================================
// Complete backend API server demonstrating:
// - Express-like HTTP server
// - MySQL database integration
// - RESTful CRUD operations
// - JSON request/response handling
// - Error handling and validation
// - Authentication middleware
// - WebSocket real-time updates
// ===========================================================================

print("╔════════════════════════════════════════════════════════════╗");
print("║     A-lang REST API Server - MySQL Backend Example        ║");
print("║     Full-stack backend with Express-like features          ║");
print("╚════════════════════════════════════════════════════════════╝");
print("");

// ===========================================================================
// DATABASE CONFIGURATION
// ===========================================================================

let dbConfig = {
    "host": "localhost",
    "port": 3306,
    "user": "root",
    "password": "password",
    "database": "alang_api",
    "poolMin": 2,
    "poolMax": 10
};

print("📊 Database Configuration:");
print("   Host: " + dbConfig.host + ":" + dbConfig.port);
print("   Database: " + dbConfig.database);
print("");

// ===========================================================================
// DATABASE CONNECTION
// ===========================================================================

// Connect to MySQL database
fn connectDatabase(config) {
    print("🔌 Connecting to MySQL database...");

    // In real implementation, this would use the database module
    // let db = Database.connect(config);

    // Simulated connection
    let db = {
        "config": config,
        "connected": true,

        // Query method
        "query": fn(sql) {
            print("   SQL: " + sql);
            return {
                "rows": [],
                "affectedRows": 0,
                "insertId": 0
            };
        },

        // Insert method
        "insert": fn(table, data) {
            print("   INSERT INTO " + table);
            return 1; // Return inserted ID
        },

        // Update method
        "update": fn(table, data, where) {
            print("   UPDATE " + table + " WHERE " + where);
            return 1; // Return affected rows
        },

        // Delete method
        "delete": fn(table, where) {
            print("   DELETE FROM " + table + " WHERE " + where);
            return 1; // Return affected rows
        },

        // Select method
        "select": fn(table, columns, where) {
            print("   SELECT " + join(columns, ", ") + " FROM " + table);
            if where != nil {
                print("   WHERE " + where);
            }

            // Return sample data
            return {
                "rows": [
                    {
                        "id": 1,
                        "name": "Alice",
                        "email": "alice@example.com",
                        "created_at": "2024-01-01 10:00:00"
                    },
                    {
                        "id": 2,
                        "name": "Bob",
                        "email": "bob@example.com",
                        "created_at": "2024-01-02 11:00:00"
                    }
                ]
            };
        },

        // Find by ID
        "findById": fn(table, id) {
            return {
                "id": id,
                "name": "Sample User",
                "email": "user@example.com",
                "created_at": timestamp()
            };
        },

        // Transaction support
        "transaction": fn(callback) {
            print("   BEGIN TRANSACTION");
            let result = callback(this);
            print("   COMMIT");
            return result;
        }
    };

    print("   ✓ Database connected successfully");
    print("");

    return db;
}

// ===========================================================================
// DATABASE SCHEMA SETUP
// ===========================================================================

fn setupDatabase(db) {
    print("📋 Setting up database schema...");

    // Create users table
    let createUsersTable = "
        CREATE TABLE IF NOT EXISTS users (
            id INT AUTO_INCREMENT PRIMARY KEY,
            name VARCHAR(100) NOT NULL,
            email VARCHAR(100) UNIQUE NOT NULL,
            password VARCHAR(255) NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
        )
    ";

    db.query(createUsersTable);
    print("   ✓ Users table created");

    // Create posts table
    let createPostsTable = "
        CREATE TABLE IF NOT EXISTS posts (
            id INT AUTO_INCREMENT PRIMARY KEY,
            user_id INT NOT NULL,
            title VARCHAR(200) NOT NULL,
            content TEXT,
            published BOOLEAN DEFAULT FALSE,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )
    ";

    db.query(createPostsTable);
    print("   ✓ Posts table created");

    print("");
}

// ===========================================================================
// API ROUTES - USERS
// ===========================================================================

// GET /api/users - Get all users
fn getAllUsers(req, db) {
    print("📡 GET /api/users");

    let result = db.select("users", ["id", "name", "email", "created_at"], nil);

    return {
        "status": 200,
        "body": stringifyJSON({
            "success": true,
            "data": result.rows,
            "count": len(result.rows)
        })
    };
}

// GET /api/users/:id - Get user by ID
fn getUserById(req, db) {
    let userId = req.params.id;
    print("📡 GET /api/users/" + userId);

    let user = db.findById("users", int(userId));

    if user == nil {
        return {
            "status": 404,
            "body": stringifyJSON({
                "success": false,
                "error": "User not found"
            })
        };
    }

    return {
        "status": 200,
        "body": stringifyJSON({
            "success": true,
            "data": user
        })
    };
}

// POST /api/users - Create new user
fn createUser(req, db) {
    print("📡 POST /api/users");

    let body = parseJSON(req.body);

    // Validation
    if body.name == nil || body.email == nil {
        return {
            "status": 400,
            "body": stringifyJSON({
                "success": false,
                "error": "Name and email are required"
            })
        };
    }

    // Check if email exists
    let existing = db.select("users", ["id"], "email = '" + body.email + "'");
    if len(existing.rows) > 0 {
        return {
            "status": 409,
            "body": stringifyJSON({
                "success": false,
                "error": "Email already exists"
            })
        };
    }

    // Insert user
    let userId = db.insert("users", {
        "name": body.name,
        "email": body.email,
        "password": hashPassword(body.password)
    });

    return {
        "status": 201,
        "body": stringifyJSON({
            "success": true,
            "data": {
                "id": userId,
                "name": body.name,
                "email": body.email
            },
            "message": "User created successfully"
        })
    };
}

// PUT /api/users/:id - Update user
fn updateUser(req, db) {
    let userId = req.params.id;
    print("📡 PUT /api/users/" + userId);

    let body = parseJSON(req.body);

    // Check if user exists
    let user = db.findById("users", int(userId));
    if user == nil {
        return {
            "status": 404,
            "body": stringifyJSON({
                "success": false,
                "error": "User not found"
            })
        };
    }

    // Update user
    let updateData = {};
    if body.name != nil {
        updateData.name = body.name;
    }
    if body.email != nil {
        updateData.email = body.email;
    }

    db.update("users", updateData, "id = " + userId);

    return {
        "status": 200,
        "body": stringifyJSON({
            "success": true,
            "message": "User updated successfully"
        })
    };
}

// DELETE /api/users/:id - Delete user
fn deleteUser(req, db) {
    let userId = req.params.id;
    print("📡 DELETE /api/users/" + userId);

    // Check if user exists
    let user = db.findById("users", int(userId));
    if user == nil {
        return {
            "status": 404,
            "body": stringifyJSON({
                "success": false,
                "error": "User not found"
            })
        };
    }

    // Delete user
    db.delete("users", "id = " + userId);

    return {
        "status": 200,
        "body": stringifyJSON({
            "success": true,
            "message": "User deleted successfully"
        })
    };
}

// ===========================================================================
// API ROUTES - POSTS
// ===========================================================================

// GET /api/posts - Get all posts
fn getAllPosts(req, db) {
    print("📡 GET /api/posts");

    let query = "
        SELECT p.*, u.name as author_name, u.email as author_email
        FROM posts p
        JOIN users u ON p.user_id = u.id
        ORDER BY p.created_at DESC
    ";

    let result = db.query(query);

    return {
        "status": 200,
        "body": stringifyJSON({
            "success": true,
            "data": result.rows,
            "count": len(result.rows)
        })
    };
}

// POST /api/posts - Create new post
fn createPost(req, db) {
    print("📡 POST /api/posts");

    let body = parseJSON(req.body);

    // Validation
    if body.title == nil || body.user_id == nil {
        return {
            "status": 400,
            "body": stringifyJSON({
                "success": false,
                "error": "Title and user_id are required"
            })
        };
    }

    // Insert post
    let postId = db.insert("posts", {
        "user_id": body.user_id,
        "title": body.title,
        "content": body.content,
        "published": body.published
    });

    return {
        "status": 201,
        "body": stringifyJSON({
            "success": true,
            "data": {
                "id": postId,
                "title": body.title
            },
            "message": "Post created successfully"
        })
    };
}

// ===========================================================================
// AUTHENTICATION MIDDLEWARE
// ===========================================================================

fn authMiddleware(req) {
    print("🔐 Auth middleware checking token...");

    let authHeader = req.headers["Authorization"];

    if authHeader == nil {
        return {
            "authorized": false,
            "error": "No authorization token provided"
        };
    }

    // Extract token (Bearer <token>)
    let token = split(authHeader, " ")[1];

    // Validate token (simplified)
    if token == nil || len(token) < 10 {
        return {
            "authorized": false,
            "error": "Invalid token"
        };
    }

    print("   ✓ Token validated");

    return {
        "authorized": true,
        "userId": 1
    };
}

// ===========================================================================
// UTILITY FUNCTIONS
// ===========================================================================

fn hashPassword(password) {
    // In real implementation, use bcrypt or similar
    // For demo, just encode to base64
    return toBase64(password);
}

fn generateToken(userId) {
    // In real implementation, use JWT
    // For demo, just create a simple token
    let token = toBase64(str(userId) + "_" + str(timestamp()));
    return token;
}

fn validateEmail(email) {
    // Simple email validation
    return len(email) > 5 && email.contains("@");
}

// ===========================================================================
// HTTP SERVER SETUP
// ===========================================================================

fn createApiServer(db) {
    print("🚀 Creating API server...");

    let server = {
        "db": db,
        "routes": {},

        // Register a route
        "route": fn(method, path, handler) {
            let key = method + ":" + path;
            this.routes[key] = handler;
            print("   ✓ Registered " + method + " " + path);
        },

        // Handle incoming request
        "handleRequest": fn(req) {
            let key = req.method + ":" + req.path;

            if this.routes[key] != nil {
                return this.routes[key](req, this.db);
            } else {
                return {
                    "status": 404,
                    "body": stringifyJSON({
                        "success": false,
                        "error": "Route not found"
                    })
                };
            }
        },

        // Start listening
        "listen": fn(port) {
            print("");
            print("═══════════════════════════════════════════════════════════");
            print("🚀 Server running on http://localhost:" + port);
            print("═══════════════════════════════════════════════════════════");
            print("");
            print("Available endpoints:");
            print("  GET    /api/users           - Get all users");
            print("  GET    /api/users/:id       - Get user by ID");
            print("  POST   /api/users           - Create new user");
            print("  PUT    /api/users/:id       - Update user");
            print("  DELETE /api/users/:id       - Delete user");
            print("  GET    /api/posts           - Get all posts");
            print("  POST   /api/posts           - Create new post");
            print("  POST   /api/auth/login      - User login");
            print("");
            print("WebSocket endpoint:");
            print("  ws://localhost:" + port + "/ws");
            print("");
        }
    };

    return server;
}

// ===========================================================================
// WEBSOCKET REAL-TIME UPDATES
// ===========================================================================

fn setupWebSocket(port) {
    print("🔌 Setting up WebSocket server...");

    let wsServer = {
        "clients": [],

        "broadcast": fn(message) {
            print("   📡 Broadcasting to " + len(this.clients) + " clients");
            for client in this.clients {
                // Send message to each client
                print("      → Client " + client.id + ": " + message);
            }
        },

        "onConnection": fn(clientId) {
            print("   ✓ New WebSocket client connected: " + clientId);
            push(this.clients, {"id": clientId});
            this.broadcast("User " + clientId + " joined");
        },

        "onMessage": fn(clientId, message) {
            print("   📨 Message from " + clientId + ": " + message);
            this.broadcast(clientId + ": " + message);
        }
    };

    print("   ✓ WebSocket server ready on port " + port);
    return wsServer;
}

// ===========================================================================
// MAIN APPLICATION
// ===========================================================================

fn main() {
    print("🎯 Initializing A-lang REST API...");
    print("");

    // Connect to database
    let db = connectDatabase(dbConfig);

    // Setup database schema
    setupDatabase(db);

    // Create API server
    let app = createApiServer(db);

    // Register routes
    app.route("GET", "/api/users", getAllUsers);
    app.route("GET", "/api/users/:id", getUserById);
    app.route("POST", "/api/users", createUser);
    app.route("PUT", "/api/users/:id", updateUser);
    app.route("DELETE", "/api/users/:id", deleteUser);
    app.route("GET", "/api/posts", getAllPosts);
    app.route("POST", "/api/posts", createPost);

    print("");

    // Setup WebSocket
    let wsServer = setupWebSocket(3001);

    // Start server
    app.listen(3000);

    // ===========================================================================
    // DEMO: Simulate API Requests
    // ===========================================================================

    print("📋 Running API Demo...");
    print("");

    // 1. Get all users
    print("1️⃣  Testing GET /api/users");
    let req1 = {
        "method": "GET",
        "path": "/api/users",
        "headers": {},
        "body": "",
        "params": {}
    };
    let response1 = app.handleRequest(req1);
    print("   Status: " + response1.status);
    print("   Body: " + response1.body);
    print("");

    // 2. Create new user
    print("2️⃣  Testing POST /api/users");
    let newUser = {
        "name": "Charlie",
        "email": "charlie@example.com",
        "password": "secret123"
    };
    let req2 = {
        "method": "POST",
        "path": "/api/users",
        "headers": {"Content-Type": "application/json"},
        "body": stringifyJSON(newUser),
        "params": {}
    };
    let response2 = app.handleRequest(req2);
    print("   Status: " + response2.status);
    print("   Body: " + response2.body);
    print("");

    // 3. Get user by ID
    print("3️⃣  Testing GET /api/users/1");
    let req3 = {
        "method": "GET",
        "path": "/api/users/:id",
        "headers": {},
        "body": "",
        "params": {"id": "1"}
    };
    let response3 = app.handleRequest(req3);
    print("   Status: " + response3.status);
    print("   Body: " + response3.body);
    print("");

    // 4. Create a post
    print("4️⃣  Testing POST /api/posts");
    let newPost = {
        "user_id": 1,
        "title": "My First Post with A-lang",
        "content": "This is awesome! Building APIs with A-lang is easy.",
        "published": true
    };
    let req4 = {
        "method": "POST",
        "path": "/api/posts",
        "headers": {"Content-Type": "application/json"},
        "body": stringifyJSON(newPost),
        "params": {}
    };
    let response4 = app.handleRequest(req4);
    print("   Status: " + response4.status);
    print("   Body: " + response4.body);
    print("");

    // 5. WebSocket simulation
    print("5️⃣  Testing WebSocket");
    wsServer.onConnection("client_001");
    wsServer.onMessage("client_001", "Hello from WebSocket!");
    print("");

    print("═══════════════════════════════════════════════════════════");
    print("✅ API Demo completed successfully!");
    print("═══════════════════════════════════════════════════════════");
    print("");

    print("📝 Summary:");
    print("   • Database: MySQL connected");
    print("   • HTTP Server: Running on port 3000");
    print("   • WebSocket: Running on port 3001");
    print("   • Routes: 7 endpoints registered");
    print("   • Features: CRUD, Auth, Real-time");
    print("");

    print("🎉 A-lang is ready for backend development!");
}

// Run the application
main();

// ===========================================================================
// END OF EXAMPLE
// ===========================================================================
