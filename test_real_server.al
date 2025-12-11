print("Creating server...")
server = createServer()

server.get("/", (req, res) => {
    res.json({message: "Hello"})
})

server.get("/api/users", (req, res) => {
    res.json({users: []})
})

server.listen(8080)
