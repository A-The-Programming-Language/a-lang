server = createServer()

server.get("/test", (req, res) => {
    print("Handler called!")
    res.json({message: "Hello from A-lang"})
})

print("Starting...")
server.listen(880)
