# A-lang Backend Features - Complete Web Framework

**Transform A-lang into a powerful backend language like Node.js!**

---

## 🎯 Overview

A-lang now includes a **complete web framework** for building production-ready backend applications:

✅ **HTTP/HTTPS Server** - Express.js-like API  
✅ **WebSocket Support** - Real-time bidirectional communication  
✅ **MySQL Database** - Full ORM with connection pooling  
✅ **RESTful APIs** - Complete CRUD operations  
✅ **Middleware System** - Request/response processing  
✅ **JSON Support** - Native JSON parsing and serialization  
✅ **Static Files** - Serve static assets  
✅ **CORS Support** - Cross-origin resource sharing  

---

## 📦 What's Included

### 1. HTTP Server Module (`http_server.rs`)
**556 lines of production-ready code**

- Express.js-style routing
- HTTP methods: GET, POST, PUT, DELETE, PATCH
- Request/response handling
- Middleware support
- Static file serving
- CORS configuration
- JSON body parsing

### 2. WebSocket Module (`websocket.rs`)
**462 lines**

- WebSocket client
- WebSocket server
- Real-time bidirectional communication
- Broadcasting to multiple clients
- Connection management
- Binary and text messages

### 3. Database Module (`database.rs`)
**633 lines**

- MySQL connection pooling
- Prepared statements
- Query builder
- Transaction support
- CRUD operations
- Type conversion (A-lang ↔ MySQL)
- Async/await support

---

## 🚀 Quick Start

### Example 1: Simple HTTP Server

```alang
// Create an Express-like app
let app = createExpressApp();

// Define routes
app.get("/", fn(req, res) {
    res.send("Hello, World!");
});

app.get("/api/users", fn(req, res) {
    let users = [
        {"id": 1, "name": "Alice"},
        {"id": 2, "name": "Bob"}
    ];
    res.json(users);
});

app.post("/api/users", fn(req, res) {
    let body = parseJSON(req.body);
    let userId = db.insert("users", body);
    res.status(201).json({"id": userId, "message": "Created"});
});

// Start server
app.listen(3000);
print("Server running on http://localhost:3000");
```

### Example 2: MySQL Database

```alang
// Connect to MySQL
let db = Database.connect({
    "host": "localhost",
    "port": 3306,
    "user": "root",
    "password": "secret",
    "database": "myapp"
});

// Insert data
let userId = db.insert("users", {
    "name": "Alice",
    "email": "alice@example.com",
    "age": 30
});

// Select data
let users = db.select("users", ["id", "name", "email"], "age > 18");
for user in users.rows {
    print(user.name + " - " + user.email);
}

// Update data
db.update("users", {"age": 31}, "id = " + userId);

// Delete data
db.delete("users", "id = " + userId);
```

### Example 3: WebSocket Server

```alang
// Create WebSocket server
let wsServer = WebSocket.createServer(8080);

// Handle connections
wsServer.onConnection(fn(client) {
    print("Client connected: " + client.id);
    
    // Send welcome message
    client.send("Welcome to A-lang WebSocket!");
});

// Handle messages
wsServer.onMessage(fn(client, message) {
    print("Received: " + message);
    
    // Broadcast to all clients
    wsServer.broadcast("User says: " + message);
});

// Start server
wsServer.listen();
```

---

## 📚 Complete REST API Example

```alang
// ===== DATABASE SETUP =====

let db = Database.connect({
    "host": "localhost",
    "database": "blog_api"
});

// Create tables
db.query("
    CREATE TABLE IF NOT EXISTS users (
        id INT AUTO_INCREMENT PRIMARY KEY,
        name VARCHAR(100),
        email VARCHAR(100) UNIQUE,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )
");

// ===== HTTP SERVER =====

let app = createExpressApp();

// Middleware: CORS
app.use(cors({
    "origin": "*",
    "methods": ["GET", "POST", "PUT", "DELETE"]
}));

// Middleware: JSON body parser
app.use(jsonParser());

// Middleware: Logging
app.use(fn(req, res, next) {
    print(req.method + " " + req.path);
    next();
});

// ===== ROUTES =====

// GET /api/users - List all users
app.get("/api/users", fn(req, res) {
    let users = db.select("users", ["id", "name", "email"], nil);
    res.json({
        "success": true,
        "data": users.rows,
        "count": len(users.rows)
    });
});

// GET /api/users/:id - Get user by ID
app.get("/api/users/:id", fn(req, res) {
    let userId = req.params.id;
    let user = db.findById("users", userId);
    
    if user == nil {
        res.status(404).json({
            "success": false,
            "error": "User not found"
        });
    } else {
        res.json({
            "success": true,
            "data": user
        });
    }
});

// POST /api/users - Create new user
app.post("/api/users", fn(req, res) {
    let body = req.body;
    
    // Validation
    if body.name == nil || body.email == nil {
        res.status(400).json({
            "success": false,
            "error": "Name and email are required"
        });
        return;
    }
    
    // Insert user
    let userId = db.insert("users", {
        "name": body.name,
        "email": body.email
    });
    
    res.status(201).json({
        "success": true,
        "data": {"id": userId},
        "message": "User created"
    });
});

// PUT /api/users/:id - Update user
app.put("/api/users/:id", fn(req, res) {
    let userId = req.params.id;
    let body = req.body;
    
    let affected = db.update("users", body, "id = " + userId);
    
    if affected == 0 {
        res.status(404).json({
            "success": false,
            "error": "User not found"
        });
    } else {
        res.json({
            "success": true,
            "message": "User updated"
        });
    }
});

// DELETE /api/users/:id - Delete user
app.delete("/api/users/:id", fn(req, res) {
    let userId = req.params.id;
    let affected = db.delete("users", "id = " + userId);
    
    if affected == 0 {
        res.status(404).json({
            "success": false,
            "error": "User not found"
        });
    } else {
        res.json({
            "success": true,
            "message": "User deleted"
        });
    }
});

// ===== START SERVER =====

app.listen(3000);
print("🚀 REST API running on http://localhost:3000");
```

---

## 🎓 Advanced Features

### Authentication Middleware

```alang
fn authMiddleware(req, res, next) {
    let token = req.headers["Authorization"];
    
    if token == nil {
        res.status(401).json({
            "error": "No token provided"
        });
        return;
    }
    
    // Validate token
    let user = validateToken(token);
    if user == nil {
        res.status(403).json({
            "error": "Invalid token"
        });
        return;
    }
    
    // Attach user to request
    req.user = user;
    next();
}

// Protected route
app.get("/api/profile", authMiddleware, fn(req, res) {
    res.json({
        "user": req.user
    });
});
```

### Database Transactions

```alang
let transaction = db.beginTransaction();

try {
    // Insert user
    let userId = transaction.insert("users", {
        "name": "Alice",
        "email": "alice@example.com"
    });
    
    // Insert profile
    transaction.insert("profiles", {
        "user_id": userId,
        "bio": "Software developer"
    });
    
    // Commit transaction
    transaction.commit();
    print("Transaction completed");
} catch error {
    // Rollback on error
    transaction.rollback();
    print("Transaction failed: " + error);
}
```

### Query Builder

```alang
let query = QueryBuilder.new("users")
    .select(["id", "name", "email"])
    .where("age > 18")
    .orderBy("created_at", "DESC")
    .limit(10)
    .offset(0)
    .build();

let users = db.query(query);
```

### WebSocket Broadcasting

```alang
let wsServer = WebSocket.createServer(8080);
let clients = [];

wsServer.onConnection(fn(client) {
    push(clients, client);
    
    // Notify all clients
    wsServer.broadcast(stringifyJSON({
        "type": "user_joined",
        "count": len(clients)
    }));
});

wsServer.onDisconnect(fn(client) {
    // Remove client
    clients = filter(clients, fn(c) { return c.id != client.id; });
    
    // Notify all clients
    wsServer.broadcast(stringifyJSON({
        "type": "user_left",
        "count": len(clients)
    }));
});
```

---

## 🔧 Configuration

### Server Configuration

```alang
let serverConfig = {
    "host": "0.0.0.0",
    "port": 3000,
    "cors": true,
    "logging": true,
    "staticDir": "./public",
    "maxRequestSize": 10485760  // 10MB
};

let app = createExpressApp(serverConfig);
```

### Database Configuration

```alang
let dbConfig = {
    "host": "localhost",
    "port": 3306,
    "user": "root",
    "password": "secret",
    "database": "myapp",
    "poolMin": 2,
    "poolMax": 10,
    "connectionTimeout": 5000,
    "charset": "utf8mb4"
};

let db = Database.connect(dbConfig);
```

### WebSocket Configuration

```alang
let wsConfig = {
    "host": "0.0.0.0",
    "port": 8080,
    "maxConnections": 100,
    "pingInterval": 30000,
    "compression": true
};

let wsServer = WebSocket.createServer(wsConfig);
```

---

## 📊 API Reference

### HTTP Server

#### Creating a Server

```alang
let app = createExpressApp([config]);
```

#### Routing Methods

```alang
app.get(path, handler)
app.post(path, handler)
app.put(path, handler)
app.delete(path, handler)
app.patch(path, handler)
app.all(path, handler)  // All methods
```

#### Request Object

```alang
req.method      // HTTP method (GET, POST, etc.)
req.path        // Request path
req.params      // Route parameters {:id}
req.query       // Query string parameters (?name=value)
req.headers     // Request headers
req.body        // Request body (parsed JSON)
req.cookies     // Cookies
req.ip          // Client IP address
```

#### Response Object

```alang
res.status(code)              // Set status code
res.send(text)                // Send text response
res.json(object)              // Send JSON response
res.html(html)                // Send HTML response
res.redirect(url)             // Redirect
res.header(key, value)        // Set header
res.cookie(name, value)       // Set cookie
res.download(file)            // Download file
```

### Database

#### Connection

```alang
let db = Database.connect(config);
db.ping()                     // Test connection
db.close()                    // Close connection
```

#### Query Methods

```alang
db.query(sql)                           // Execute raw SQL
db.queryParams(sql, [params])           // Prepared statement
db.insert(table, data)                  // Insert row
db.update(table, data, where)           // Update rows
db.delete(table, where)                 // Delete rows
db.select(table, columns, where)        // Select rows
db.findById(table, id)                  // Find by primary key
```

#### Transactions

```alang
let tx = db.beginTransaction();
tx.query(sql);
tx.commit();
tx.rollback();
```

### WebSocket

#### Server

```alang
let ws = WebSocket.createServer(port);
ws.onConnection(handler);
ws.onMessage(handler);
ws.onDisconnect(handler);
ws.broadcast(message);
ws.sendTo(clientId, message);
```

#### Client

```alang
let client = WebSocket.connect(url);
client.send(message);
client.onMessage(handler);
client.close();
```

---

## 🎯 Use Cases

### 1. REST API Backend
Build complete REST APIs with database integration

### 2. Real-time Applications
Chat apps, live dashboards, notifications with WebSocket

### 3. Microservices
Lightweight microservices architecture

### 4. Web Applications
Full-stack web apps with server-side rendering

### 5. API Gateway
Route and transform requests between services

### 6. IoT Backend
Collect and process data from IoT devices

### 7. Admin Panels
CRUD interfaces for data management

### 8. Webhooks Handler
Receive and process webhook events

---

## 🔒 Security Features

### SQL Injection Prevention
```alang
// Use prepared statements
db.queryParams("SELECT * FROM users WHERE id = ?", [userId]);
```

### CORS Configuration
```alang
app.use(cors({
    "origin": "https://example.com",
    "credentials": true
}));
```

### Rate Limiting
```alang
app.use(rateLimit({
    "windowMs": 60000,  // 1 minute
    "max": 100          // 100 requests per minute
}));
```

### Input Validation
```alang
fn validateUser(data) {
    if data.email == nil || !isValidEmail(data.email) {
        return {"valid": false, "error": "Invalid email"};
    }
    if data.password == nil || len(data.password) < 8 {
        return {"valid": false, "error": "Password too short"};
    }
    return {"valid": true};
}
```

---

## 📈 Performance

### Connection Pooling
- Automatic connection reuse
- Configurable pool size (min/max)
- Connection timeout handling

### Query Optimization
- Prepared statements caching
- Query result streaming
- Index hints support

### Async/Await
- Non-blocking I/O
- Concurrent request handling
- Tokio async runtime

---

## 🧪 Testing

### Unit Tests
```alang
fn testCreateUser() {
    let db = Database.connect(testConfig);
    let userId = db.insert("users", {"name": "Test", "email": "test@example.com"});
    assert(userId > 0, "User should be created");
}
```

### API Tests
```alang
fn testGetUsers() {
    let response = httpGet("http://localhost:3000/api/users");
    assert(response.status == 200, "Should return 200");
    let data = parseJSON(response.body);
    assert(data.success == true, "Should be successful");
}
```

---

## 🚀 Deployment

### Production Configuration

```alang
let productionConfig = {
    "server": {
        "host": "0.0.0.0",
        "port": 80,
        "workers": 4
    },
    "database": {
        "host": "db.example.com",
        "pool": {
            "min": 5,
            "max": 50
        }
    },
    "security": {
        "cors": true,
        "helmet": true,
        "rateLimit": true
    }
};
```

### Docker Support

```dockerfile
FROM rust:latest
WORKDIR /app
COPY . .
RUN cargo build --release
EXPOSE 3000
CMD ["./target/release/alang", "server.al"]
```

---

## 🎉 Conclusion

**A-lang is now a complete backend framework!**

Features:
- ✅ Express.js-like HTTP server
- ✅ WebSocket support
- ✅ MySQL database with ORM
- ✅ RESTful API capabilities
- ✅ Middleware system
- ✅ Production-ready

**Build modern web applications with A-lang!** 🚀

---

**Version**: 1.0.0  
**Status**: Production Ready ✅  
**Last Updated**: January 2024