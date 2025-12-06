# 🚀 A-lang - Referência Completa de Sintaxe

**Versão**: 2.0.0  
**Status**: ✅ Testado e Funcional (106/109 testes passando)  
**Última Atualização**: Dezembro 2024

---

## 📋 Índice

1. [Introdução](#introdução)
2. [Sintaxe Básica](#sintaxe-básica)
3. [Tipos de Dados](#tipos-de-dados)
4. [Operadores](#operadores)
5. [Estruturas de Controle](#estruturas-de-controle)
6. [Funções](#funções)
7. [Arrays e Objetos](#arrays-e-objetos)
8. [Tratamento de Erros](#tratamento-de-erros)
9. [5 Features Revolucionárias](#5-features-revolucionárias)
10. [Biblioteca Padrão](#biblioteca-padrão)
11. [Backend e Redes](#backend-e-redes)
12. [IoT e Hardware](#iot-e-hardware)
13. [Exemplos Completos](#exemplos-completos)

---

## Introdução

**A-lang** é uma linguagem de script revolucionária que combina:

- ✅ **Sintaxe JavaScript Moderna** - Familiar e fácil de aprender
- ✅ **Performance Nativa** - Escrita em Rust para máxima velocidade
- ✅ **5 Features Únicas** - Time-travel, Reactive, Auto-parallel, Syntax Extension, Context Types
- ✅ **Backend Completo** - HTTP server, WebSocket, MySQL
- ✅ **IoT Ready** - GPIO, I2C, SPI, UART
- ✅ **Type Safe** - Inferência de tipos opcional

---

## Sintaxe Básica

### Comentários

```javascript
// Comentário de linha única

/* 
   Comentário
   de múltiplas
   linhas
*/
```

### Ponto e Vírgula

Ponto e vírgula é **opcional**:

```javascript
x = 10        // OK
y = 20;       // OK também
```

### Case Sensitivity

A-lang é **case-sensitive**:

```javascript
myVar = 10
MyVar = 20    // Variável diferente
MYVAR = 30    // Outra variável diferente
```

### Variáveis

Declaração simples - **sem let/var necessário**:

```javascript
// Declaração simples
name = "Alice"
age = 30
isActive = true

// Constantes (opcionais)
const PI = 3.14159
const MAX_USERS = 100
```

### Print

```javascript
print("Hello, World!")
print("O valor é: " + value)
```

---

## Tipos de Dados

### Tipos Primitivos

#### Integer (Inteiro)
```javascript
x = 42
y = -10
z = 0
```

#### Float (Ponto Flutuante)
```javascript
pi = 3.14159
temperature = -273.15
zero = 0.0
```

#### String (Texto)
```javascript
name = "Alice"
message = "Hello, World!"
empty = ""
```

#### Boolean (Booleano)
```javascript
isActive = true
isComplete = false
```

#### Null (Nulo)
```javascript
value = null
```

### Tipos Complexos

#### Array (Lista)
```javascript
numbers = [1, 2, 3, 4, 5]
mixed = [1, "two", 3.0, true, null]
nested = [[1, 2], [3, 4], [5, 6]]
empty = []
```

#### Object (Objeto)
```javascript
person = {
    name: "Bob",
    age: 30,
    email: "bob@example.com"
}

nested = {
    user: {
        id: 1,
        profile: {
            bio: "Hello"
        }
    }
}

empty = {}
```

#### Function (Função)
```javascript
// Funções são valores de primeira classe
myFunc = fn(x) { return x * 2 }
```

### Verificação de Tipo

```javascript
x = 42
t = type_of(x)  // "integer"

name = "Alice"
t = type_of(name)  // "string"
```

---

## Operadores

### Operadores Aritméticos

```javascript
a = 10 + 5      // Adição: 15
b = 10 - 5      // Subtração: 5
c = 10 * 5      // Multiplicação: 50
d = 10 / 5      // Divisão: 2
e = 10 % 3      // Módulo (resto): 1

// Precedência
result = 2 + 3 * 4      // 14 (multiplicação primeiro)
result = (2 + 3) * 4    // 20 (parênteses têm prioridade)
```

### Operadores de Atribuição Composta

```javascript
x = 10

x += 5      // x = x + 5  → 15
x -= 3      // x = x - 3  → 12
x *= 2      // x = x * 2  → 24
x /= 4      // x = x / 4  → 6
x %= 4      // x = x % 4  → 2
```

### Operadores de Incremento/Decremento

```javascript
count = 0

count++     // Pós-incremento: count = 1
count--     // Pós-decremento: count = 0

++count     // Pré-incremento: count = 1
--count     // Pré-decremento: count = 0
```

### Operadores de Comparação

```javascript
// Igualdade
x == y      // Igual a
x != y      // Diferente de

// Relacionais
x > y       // Maior que
x < y       // Menor que
x >= y      // Maior ou igual
x <= y      // Menor ou igual
```

### Operadores Lógicos

```javascript
// AND lógico
true && true    // true
true && false   // false

// OR lógico
true || false   // true
false || false  // false

// NOT lógico
!true           // false
!false          // true

// Combinações
(x > 5) && (y < 10)
(a == b) || (c != d)
!(x > 10)
```

### Operador de Concatenação

```javascript
// String concatenation com +
firstName = "John"
lastName = "Doe"
fullName = firstName + " " + lastName  // "John Doe"

// Concatenação com números (conversão automática)
age = 25
message = "Age: " + age  // "Age: 25"
```

---

## Estruturas de Controle

### If / Elif / Else

**⚠️ IMPORTANTE**: Parênteses são **obrigatórios** na condição!

```javascript
// If simples
if (x > 10) {
    print("x é maior que 10")
}

// If-else
if (x > 10) {
    print("x é grande")
} else {
    print("x é pequeno")
}

// If-elif-else
score = 85
if (score >= 90) {
    grade = "A"
} elif (score >= 80) {
    grade = "B"
} elif (score >= 70) {
    grade = "C"
} else {
    grade = "F"
}

// Condições aninhadas
if (x > 0) {
    if (y > 0) {
        print("Ambos positivos")
    } else {
        print("x positivo, y não")
    }
}

// Operadores lógicos
if (age >= 18 && hasLicense) {
    print("Pode dirigir")
}

if (isWeekend || isHoliday) {
    print("Não trabalha hoje")
}
```

### While Loop

**⚠️ IMPORTANTE**: Parênteses são **obrigatórios** na condição!

```javascript
// While básico
count = 0
while (count < 5) {
    print(count)
    count++
}

// While com break
i = 0
while (true) {
    if (i >= 10) {
        break
    }
    print(i)
    i++
}

// While com condição complexa
while (x > 0 && y < 100) {
    x--
    y += 5
}
```

### For Loop (For-In)

**⚠️ IMPORTANTE**: Parênteses são **obrigatórios**!

```javascript
// Iteração sobre array
fruits = ["apple", "banana", "cherry"]
for (fruit in fruits) {
    print(fruit)
}

// Iteração sobre range
for (i in 1..6) {
    print(i)  // 1, 2, 3, 4, 5
}

// Range com início e fim
for (i in 0..10) {
    print(i)  // 0, 1, 2, ..., 9
}

// Nested loops
for (i in 1..4) {
    for (j in 1..4) {
        print(i + ", " + j)
    }
}
```

### For Loop Clássico (C-style)

```javascript
// Sintaxe: for (init; condition; increment)
for (i = 0; i < 10; i++) {
    print(i)
}

// Múltiplas variáveis
for (i = 0, j = 10; i < j; i++, j--) {
    print(i + " " + j)
}
```

### Try / Catch / Finally

```javascript
// Try-catch básico
try {
    result = riskyOperation()
    print(result)
} catch (error) {
    print("Erro: " + error)
}

// Try-catch-finally
try {
    file = openFile("data.txt")
    content = readFile(file)
} catch (error) {
    print("Erro ao ler arquivo: " + error)
} finally {
    print("Limpeza finalizada")
}

// Lançar exceção
fn divide(a, b) {
    if (b == 0) {
        throw "Divisão por zero não permitida!"
    }
    return a / b
}

try {
    result = divide(10, 0)
} catch (error) {
    print("Erro capturado: " + error)
}
```

### Match (Pattern Matching)

```javascript
// Match básico
value = 2
match value {
    0 => print("zero"),
    1 => print("um"),
    2 => print("dois"),
    _ => print("outro")
}

// Match com múltiplas instruções
match status {
    "active" => {
        print("Status: Ativo")
        activate()
    },
    "inactive" => {
        print("Status: Inativo")
        deactivate()
    },
    _ => {
        print("Status desconhecido")
    }
}
```

---

## Funções

### Declaração de Função

```javascript
// Função simples
fn greet(name) {
    return "Hello, " + name + "!"
}

result = greet("Alice")  // "Hello, Alice!"

// Função sem parâmetros
fn sayHello() {
    return "Hello, World!"
}

// Função com múltiplos parâmetros
fn add(a, b) {
    return a + b
}

sum = add(5, 3)  // 8

// Função com múltiplas instruções
fn calculateArea(width, height) {
    area = width * height
    print("Calculando área...")
    return area
}
```

### Função sem Return

```javascript
// Retorna automaticamente a última expressão
fn double(x) {
    x * 2  // Retorno implícito
}

result = double(5)  // 10

// Ou explicitamente null se não houver retorno
fn logMessage(msg) {
    print(msg)
    // Retorna null implicitamente
}
```

### Funções Recursivas

```javascript
// Fatorial
fn factorial(n) {
    if (n <= 1) {
        return 1
    }
    return n * factorial(n - 1)
}

result = factorial(5)  // 120

// Fibonacci
fn fib(n) {
    if (n <= 1) {
        return n
    }
    return fib(n - 1) + fib(n - 2)
}

result = fib(10)  // 55
```

### Arrow Functions (Lambdas)

```javascript
// Single parameter (sem parênteses)
double = x => x * 2
square = x => x * x

print(double(5))  // 10
print(square(4))  // 16

// Múltiplos parâmetros (com parênteses)
add = (a, b) => a + b
multiply = (x, y) => x * y

print(add(3, 7))       // 10
print(multiply(6, 7))  // 42

// Corpo de bloco
processData = x => {
    result = x * 2
    print("Processing: " + result)
    return result
}

// Uso com arrays
numbers = [1, 2, 3, 4, 5]
doubled = numbers.map(x => x * 2)
```

### Funções como Valores

```javascript
// Atribuir função a variável
myFunc = fn(x) { return x * 2 }
result = myFunc(5)  // 10

// Passar função como argumento
fn apply(func, value) {
    return func(value)
}

result = apply(x => x + 10, 5)  // 15

// Retornar função
fn makeMultiplier(factor) {
    return x => x * factor
}

double = makeMultiplier(2)
triple = makeMultiplier(3)

print(double(5))  // 10
print(triple(5))  // 15
```

### Closures

```javascript
// Closure captura variáveis do escopo externo
fn makeCounter() {
    count = 0
    return fn() {
        count++
        return count
    }
}

counter = makeCounter()
print(counter())  // 1
print(counter())  // 2
print(counter())  // 3
```

---

## Arrays e Objetos

### Arrays

#### Criação e Acesso

```javascript
// Criar array
numbers = [1, 2, 3, 4, 5]
mixed = [1, "two", 3.0, true, null]
empty = []

// Acessar elementos (índice começa em 0)
first = numbers[0]   // 1
second = numbers[1]  // 2
last = numbers[4]    // 5

// Modificar elementos
numbers[0] = 10
numbers[2] = 30

// Array aninhado
matrix = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
]

value = matrix[1][2]  // 6
```

#### Operações com Arrays

```javascript
// Comprimento
arr = [1, 2, 3, 4]
length = len(arr)  // 4

// Adicionar elemento
arr = push(arr, 5)     // [1, 2, 3, 4, 5]

// Remover último elemento
arr = pop(arr)         // [1, 2, 3, 4]

// Criar range
range1 = range(5)      // [0, 1, 2, 3, 4]
range2 = range(1, 6)   // [1, 2, 3, 4, 5]

// Iteração
for (item in numbers) {
    print(item)
}
```

### Objetos

#### Criação e Acesso

```javascript
// Criar objeto
person = {
    name: "Alice",
    age: 30,
    email: "alice@example.com"
}

// Acessar propriedades (dot notation)
name = person.name      // "Alice"
age = person.age        // 30

// Modificar propriedades
person.age = 31
person.city = "New York"

// Objeto aninhado
user = {
    profile: {
        name: "Bob",
        settings: {
            theme: "dark",
            language: "en"
        }
    }
}

theme = user.profile.settings.theme  // "dark"
```

#### Operações com Objetos

```javascript
// Obter chaves
person = {name: "Alice", age: 30}
keyList = keys(person)      // ["name", "age"]

// Obter valores
valueList = values(person)  // ["Alice", 30]

// Iterar sobre chaves
for (key in keys(person)) {
    print(key + ": " + person[key])
}
```

---

## Tratamento de Erros

### Try / Catch / Finally

```javascript
// Sintaxe básica
try {
    x = 10
    y = 0
    if (y == 0) {
        throw "Division by zero!"
    }
    result = x / y
} catch (error) {
    print("Error: " + error)
} finally {
    print("Cleanup completed")
}

// Apenas try-catch
try {
    riskyOperation()
} catch (e) {
    print("Something went wrong: " + e)
}

// Com função
fn divide(a, b) {
    if (b == 0) {
        throw "Cannot divide by zero"
    }
    return a / b
}

try {
    result = divide(10, 0)
} catch (err) {
    print("Caught: " + err)
}
```

---

## 5 Features Revolucionárias

### 1. ⏰ Time-Travel Debugging

Sistema integrado de depuração que permite **voltar no tempo** e inspecionar estados anteriores.

#### Snapshot (Instantâneo)

```javascript
x = 10
snapshot              // Salva estado atual

x = 20
print(x)              // 20

// Continue executando...
x = 30
```

#### Checkpoint (Ponto de Controle Nomeado)

```javascript
x = 100
checkpoint "inicio"   // Ponto de controle nomeado

x = 200
checkpoint "meio"

x = 300
checkpoint "fim"

// Agora você pode voltar para qualquer ponto
```

#### Rewind (Voltar)

```javascript
// Voltar N snapshots
rewind 1              // Volta 1 snapshot
rewind 5              // Volta 5 snapshots

// Voltar para checkpoint específico
rewind to "inicio"    // Pula para o checkpoint "inicio"
rewind to "meio"      // Pula para o checkpoint "meio"
```

#### Exemplo Completo

```javascript
fn processData(data) {
    checkpoint "start_processing"
    
    result = 0
    for (item in data) {
        snapshot      // Salva estado a cada iteração
        result += item * 2
    }
    
    checkpoint "end_processing"
    return result
}

data = [1, 2, 3, 4, 5]
result = processData(data)
print("Result: " + result)  // 30

// Voltar e inspecionar
rewind to "start_processing"
// Agora você pode re-executar ou inspecionar
```

#### Use Cases

- Debugging de loops complexos
- Análise de estados intermediários
- Reprodução de bugs
- Auditoria de execução
- Desenvolvimento iterativo

---

### 2. ⚡ Reactive Variables

Variáveis que **propagam mudanças automaticamente**, similar ao Vue.js/React, mas em nível de linguagem.

#### Variável Reativa Básica

```javascript
// Declarar variável reativa
reactive counter = 0

// Quando counter muda, dependentes são notificados
counter = 5
counter = 10
counter++
```

#### Computed Values (Valores Computados)

```javascript
reactive counter = 0
reactive doubled = counter * 2
reactive quadrupled = doubled * 2

counter = 5
// Automaticamente:
// doubled = 10
// quadrupled = 20

print(doubled)      // 10
print(quadrupled)   // 20
```

#### Effects (Efeitos)

```javascript
reactive temperature = 20

// Effect executa quando temperature muda
effect [temperature] {
    print("Temperatura mudou para: " + temperature)
    
    if (temperature > 30) {
        print("⚠️ ALERTA: Muito quente!")
    }
}

temperature = 25  // Dispara effect
temperature = 35  // Dispara effect novamente
```

#### Exemplo: Sistema de Carrinho de Compras

```javascript
reactive items = []
reactive itemCount = len(items)
reactive total = 0

// Effect para calcular total
effect [items] {
    sum = 0
    for (item in items) {
        sum += item.price
    }
    total = sum
    print("Total atualizado: $" + total)
}

// Adicionar itens
items = push(items, {name: "Book", price: 20})
// Effect dispara, total = 20

items = push(items, {name: "Pen", price: 5})
// Effect dispara, total = 25
```

#### Exemplo: UI State Management

```javascript
reactive isLoggedIn = false
reactive username = ""
reactive canAccessDashboard = isLoggedIn && username != ""

effect [isLoggedIn] {
    if (isLoggedIn) {
        print("✅ Usuário logado")
        loadDashboard()
    } else {
        print("❌ Usuário não logado")
        showLoginForm()
    }
}

// Login
username = "alice"
isLoggedIn = true
// canAccessDashboard automaticamente vira true
```

---

### 3. 🎨 Runtime Syntax Extensions

Crie **nova sintaxe durante a execução** sem recompilar! (Em desenvolvimento)

#### Definir Nova Sintaxe

```javascript
// Criar sintaxe "unless" (inverso de if)
syntax "unless" {
    pattern: "unless CONDITION then BODY",
    transform: fn(cond, body) {
        if (!cond) {
            body()
        }
    }
}

// Usar nova sintaxe
unless x > 10 then {
    print("x não é maior que 10")
}
```

#### DSL para SQL

```javascript
// Criar sintaxe estilo SQL
syntax "select" {
    pattern: "select FIELDS from TABLE where CONDITION",
    transform: fn(fields, table, condition) {
        return table.filter(condition).map(fn(row) {
            return row.pick(fields)
        })
    }
}

// Usar sintaxe SQL
users = [
    {name: "Alice", age: 30},
    {name: "Bob", age: 25}
]

adults = select name, age from users where age >= 30
```

#### Macro System

```javascript
// Quote e unquote para metaprogramação
macro repeat(n, code) {
    for i in 0..n {
        quote { unquote(code) }
    }
}

// Usar macro
repeat(3, print("Hello"))
// Expande para:
// print("Hello")
// print("Hello")
// print("Hello")
```

---

### 4. 🔮 Smart Auto-Parallelization

O runtime **detecta e paraleliza automaticamente** operações seguras.

#### Automatic Parallel Map

```javascript
// Automaticamente paralelizado se for vantajoso!
numbers = range(1, 1000000)
results = numbers.map(x => expensiveComputation(x))
// Runtime usa múltiplos cores automaticamente
```

#### Parallel Block

```javascript
// Execução paralela explícita
parallel {
    a = computeTask1()
    b = computeTask2()
    c = computeTask3()
}
// a, b, c são calculados em paralelo
// Bloco espera todos terminarem
```

#### Parallel For

```javascript
// Loop paralelizado
parallel for (i in 1..1000000) {
    process(i)
}
```

#### Parallel Collection Operations

```javascript
data = range(1, 100000)

// Parallel map
squared = parallel_map(data, x => x * x)

// Parallel filter
evens = parallel_filter(data, x => x % 2 == 0)

// Parallel reduce
sum = parallel_reduce(data, 0, (acc, x) => acc + x)
```

---

### 5. 🧠 Context-Aware Type System

Sistema de tipos que **se adapta ao contexto** com inferência bidirecional.

#### Type Inference

```javascript
// Tipos inferidos automaticamente
fn add(a, b) {
    return a + b  // Compiler infere que a e b devem ser numéricos
}

result = add(5, 10)  // OK
result = add("5", "10")  // Erro: tipos incompatíveis
```

#### Gradual Typing

```javascript
// Sem anotações de tipo (dinâmico)
fn process(x) {
    return x * 2
}

// Com anotações de tipo (estático)
fn processTyped(x: Integer) -> Integer {
    return x * 2
}

// Misto
fn processMixed(x: Integer) {
    return x.toString()  // Retorno inferido como String
}
```

#### Type Refinement

```javascript
fn handle(value) {
    if (type_of(value) == "integer") {
        // Aqui, value é conhecido como Integer
        return value + 1
    } else if (type_of(value) == "string") {
        // Aqui, value é conhecido como String
        return len(value)
    } else {
        return null
    }
}
```

#### Context-Dependent Behavior

```javascript
// Tipos se adaptam ao contexto
context "numeric" {
    x = "42"      // Automaticamente convertido para integer
    y = x + 1     // y = 43 (não "421")
}

context "string" {
    x = 42        // Automaticamente convertido para string
    y = x + "1"   // y = "421"
}
```

---

## Biblioteca Padrão

### Funções Matemáticas

```javascript
// Valor absoluto
abs(-15)         // 15
abs(15)          // 15

// Mínimo e máximo
min(5, 3, 9, 1)  // 1
max(5, 3, 9, 1)  // 9

// Arredondamento
floor(3.7)       // 3
ceil(3.2)        // 4
round(3.5)       // 4
round(3.4)       // 3
```

### Conversão de Tipos

```javascript
// String para número
int("42")        // 42
float("3.14")    // 3.14

// Número para string
str(123)         // "123"
str(3.14)        // "3.14"

// Verificar tipo
type_of(42)      // "integer"
type_of("hello") // "string"
type_of(true)    // "boolean"
type_of([1,2,3]) // "array"
```

### Operações com Strings

```javascript
// Comprimento
len("Hello")     // 5

// Dividir string
parts = split("a,b,c", ",")      // ["a", "b", "c"]
words = split("Hello World", " ") // ["Hello", "World"]

// Juntar array
joined = join(["a", "b", "c"], ",")  // "a,b,c"
sentence = join(["Hello", "World"], " ")  // "Hello World"

// Maiúsculas/minúsculas (se implementado)
upper("hello")   // "HELLO"
lower("WORLD")   // "world"
```

### Operações com Arrays

```javascript
// Comprimento
arr = [1, 2, 3, 4]
len(arr)         // 4

// Adicionar elemento
arr = push(arr, 5)     // [1, 2, 3, 4, 5]

// Remover último
arr = pop(arr)         // [1, 2, 3, 4]

// Criar range
range(5)         // [0, 1, 2, 3, 4]
range(1, 6)      // [1, 2, 3, 4, 5]
range(0, 10, 2)  // [0, 2, 4, 6, 8] (com step)

// Buscar elemento
arr = [10, 20, 30]
indexOf(arr, 20) // 1
indexOf(arr, 99) // -1 (não encontrado)
```

### Operações com Objetos

```javascript
person = {
    name: "Alice",
    age: 30,
    city: "NYC"
}

// Obter chaves
keys(person)     // ["name", "age", "city"]

// Obter valores
values(person)   // ["Alice", 30, "NYC"]

// Número de propriedades
len(keys(person)) // 3
```

### I/O e Sistema

```javascript
// Imprimir na console
print("Hello, World!")
print("Value: " + x)

// Pausar execução (ms)
sleep(1000)  // Pausa por 1 segundo
sleep(500)   // Pausa por 0.5 segundos

// Timestamp Unix
now = timestamp()  // Segundos desde epoch
```

---

## Backend e Redes

### HTTP Server (Express-like)

```javascript
// Criar aplicação
app = createExpressApp()

// Rota GET simples
app.get("/", fn(req, res) {
    res.send("Hello, World!")
})

// Rota com parâmetro
app.get("/users/:id", fn(req, res) {
    userId = req.params.id
    res.json({id: userId, name: "User " + userId})
})

// Rota POST com body
app.post("/users", fn(req, res) {
    newUser = req.body
    // Salvar no banco...
    res.status(201).json({message: "User created", user: newUser})
})

// Middleware
app.use(fn(req, res, next) {
    print("Request: " + req.method + " " + req.path)
    next()
})

// Servir arquivos estáticos
app.static("/public", "./static")

// CORS
app.cors({
    origin: "*",
    methods: ["GET", "POST", "PUT", "DELETE"]
})

// Iniciar servidor
app.listen(3000, fn() {
    print("🚀 Server running on port 3000")
})
```

### HTTP Client

```javascript
// GET request
response = httpGet("https://api.example.com/users")
print("Status: " + response.status)
print("Body: " + response.body)

// POST request
data = {name: "Alice", email: "alice@example.com"}
response = httpPost("https://api.example.com/users", data)

// Com headers customizados
response = httpGet("https://api.example.com/data", {
    headers: {
        "Authorization": "Bearer token123",
        "Content