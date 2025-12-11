server = createServer()
server.use(cors())

server.get("/api/users", (req, res) => {
    users = db.query("SELECT * FROM users")
    res.json(users)
})

server.post("/api/users", (req, res) => {
    id = db.insert("users", req.body)
    res.status(201).json({id: id})
})

print("Server ready!")
server.listen(3000)
