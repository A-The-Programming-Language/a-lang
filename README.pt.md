<div align="center">
  <img src="Assets/tumb.png" alt="A-lang Banner" width="100%" />
</div>

<br/>

# 🚀 A-lang - A Linguagem Reativa com Viagem no Tempo

[![Versão](https://img.shields.io/badge/versão-1.0--preview-blue.svg)](https://github.com/A-The-Programming-Language/a-lang/releases)
[![Licença](https://img.shields.io/badge/licença-MIT-green.svg)](LICENSE)
[![Plataforma](https://img.shields.io/badge/plataforma-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey.svg)]()
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

**A-lang** é uma linguagem de script revolucionária que une o melhor do design moderno de linguagens com recursos inovadores como depuração de viagem no tempo e variáveis reativas. Construída em Rust para performance e segurança.

[🌐 English](README.md) | [📚 Documentação](DOCUMENTATION.md) | [🎯 Exemplos](examples/)

---

## ✨ Recursos Principais

### 🌟 5 Fatores WOW

#### ⏰ 1. Depuração com Viagem no Tempo
Depuração integrada com viagem no tempo permite rebobinar a execução, inspecionar estados históricos e reproduzir a execução do código de qualquer ponto. **Sem depurador externo necessário!**

```javascript
let x = 10;
snapshot;           // Tira um snapshot
x = x + 5;
rewind 1;           // Volta um snapshot
print(x);           // Imprime: 10
```

**Recursos:** `snapshot`, `rewind`, `checkpoint` - Controle total do histórico de execução integrado na linguagem!

---

#### ⚡ 2. Variáveis Reativas
Variáveis são reativas por padrão - mudanças se propagam automaticamente pelo programa como em frameworks frontend modernos, mas em nível de linguagem.

```javascript
reactive counter <- 0;
computed double <- counter * 2;
counter <- 5;       // double automaticamente vira 10
```

**Recursos:**
- Variáveis reativas (`reactive x <- value`)
- Valores computados (`computed y <- expression`)
- Efeitos (`effect { ... }`)
- Expressões watch
- Rastreamento automático de dependências

---

#### 🎨 3. Extensões de Sintaxe em Runtime
Crie nova sintaxe durante a execução! Defina DSLs dinamicamente sem recompilar.

```javascript
syntax "unless" {
    pattern: "unless COND then BODY",
    transform: |cond, body| if !cond then body
}

unless x > 10 then print("x é pequeno")
```

**Recursos:**
- Quote (código como dados)
- Unquote (avaliar em contexto quote)
- Sistema de macros em runtime
- Criação de DSLs customizadas sem recompilar

---

#### 🔮 4. Auto-Paralelização Inteligente
O runtime detecta e paraleliza operações seguras automaticamente sem intervenção manual.

```javascript
parallel {
    let results = [1, 2, 3, 4, 5].map(|x| heavy_computation(x));
}
// Executa automaticamente em múltiplas threads!
```

**Recursos:**
- Paralelização automática de operações seguras
- Blocos paralelos
- Execução concorrente
- Blocos atômicos (sincronizados)
- Map e filter paralelos

---

#### 🧠 5. Sistema de Tipos Consciente de Contexto
Tipos se adaptam ao contexto de uso com inferência de tipos bidirecional, oferecendo a flexibilidade de tipagem dinâmica com a segurança de tipagem estática.

```javascript
fn process(x) {  // Tipo inferido do uso
    x + 1        // x deve ser numérico
}

let result = process(42);  // Funciona!
```

**Recursos:**
- Inferência de tipos bidirecional
- Adaptação de tipos ao contexto
- Restrições de tipos
- Tipagem flexível porém segura

---

## 🚀 Início Rápido

### Instalação

#### Ubuntu/Debian
```bash
wget https://github.com/A-The-Programming-Language/a-lang/releases/download/v1.0-preview/alang_1.0-preview_amd64.deb
sudo dpkg -i alang_1.0-preview_amd64.deb
```

#### Linux (Portátil)
```bash
wget https://github.com/A-The-Programming-Language/a-lang/releases/download/v1.0-preview/alang-1.0-preview-linux-x64.tar.gz
tar -xzf alang-1.0-preview-linux-x64.tar.gz
export PATH=$PATH:$(pwd)/alang-1.0-preview-linux-x64
```

#### macOS
```bash
# Em breve via Homebrew
brew install a-lang
```

#### Windows
Baixe e execute [A-lang-1.0-preview-Setup.exe](https://github.com/A-The-Programming-Language/a-lang/releases)

---

### Olá Mundo

```javascript
print("Olá, Mundo!")
```

Execute:
```bash
alang hello.al
```

Ou use o REPL:
```bash
alang
> print("Olá, Mundo!")
Olá, Mundo!
```

---

## 💡 Exemplos

### Calculadora Interativa
```javascript
num1 = float(input("Primeiro número: "))
num2 = float(input("Segundo número: "))

print("Soma: " + str(num1 + num2))
print("Produto: " + str(num1 * num2))
```

### FFI - Chame Funções C
```javascript
// Carregar biblioteca matemática
ffiLoadLibrary("libm", "/lib/x86_64-linux-gnu/libm.so.6")

// Raiz quadrada
ffiRegisterFunction("sqrt", "double", ["double"])
print(ffiCall("libm", "sqrt", [16.0]))  // 4.0

// Potência
ffiRegisterFunction("pow", "double", ["double", "double"])
print(ffiCall("libm", "pow", [2.0, 8.0]))  // 256.0
```

### Depuração com Viagem no Tempo
```javascript
total = 0
for (i in range(5)) {
    total += i
    snapshot("passo_" + str(i))
}
print("Final: " + str(total))

// Voltar e inspecionar
rewind("passo_2")
print("No passo 2, total era: " + str(total))
```

### Contador Reativo
```javascript
reactive counter = 0

computed doubled = () => counter * 2
computed squared = () => counter * counter

effect () => {
    print("Contador: " + str(counter))
    print("Dobrado: " + str(doubled))
    print("Quadrado: " + str(squared))
}

counter = 5
// Automaticamente imprime:
// Contador: 5
// Dobrado: 10
// Quadrado: 25
```

### API REST
```javascript
app = createExpressApp()

app.get("/api/hello", fn(req, res) {
    res.json({"message": "Olá, Mundo!"})
})

app.post("/api/echo", fn(req, res) {
    body = req.body
    res.json(body)
})

print("Servidor rodando em http://localhost:3000")
app.listen(3000)
```

---

## 📚 Recursos da Linguagem

### Sintaxe Moderna Estilo JavaScript
```javascript
// Variáveis
name = "Alice"
age = 30
active = true

// Funções
fn greet(name) {
    return "Olá, " + name + "!"
}

// Funções seta
double = (x) => x * 2
add = (a, b) => a + b

// Controle de fluxo
if (age >= 18) {
    print("Adulto")
} elif (age >= 13) {
    print("Adolescente")
} else {
    print("Criança")
}

// Loops
for (i in range(10)) {
    print(i)
}

while (count < 100) {
    count++
}

// Arrays
numbers = [1, 2, 3, 4, 5]
print(numbers[0])

// Objetos
person = {
    name: "Alice",
    age: 30,
    city: "SP"
}
print(person.name)
```

### Funções Integradas (80+)
```javascript
// Matemática
abs(-5), min(1,2,3), max(1,2,3)
floor(3.7), ceil(3.2), round(3.5)

// Strings
len("olá"), split("a,b,c", ","), join(["a","b"], ",")
toUpperCase("olá"), toLowerCase("OLÁ")

// Arrays
push(arr, item), pop(arr), slice(arr, 0, 5)
indexOf(arr, item), includes(arr, item)

// Conversão de tipos
int("42"), float("3.14"), str(123)

// I/O
input("Prompt: "), print("Saída")
readFile("dados.txt"), writeFile("dados.txt", content)

// Sistema
sleep(1000), timestamp(), exit(0)
```

---

## 🎯 Casos de Uso

### ✅ Scripts e Automação
Substitua scripts Bash/Python com sintaxe moderna e melhor depuração.

### ✅ Desenvolvimento Backend
Construa APIs REST, servidores WebSocket e microserviços.

### ✅ Sistemas IoT e Embarcados
Controle hardware com suporte GPIO, I2C, SPI e UART.

### ✅ Programação de Sistema
Acesse bibliotecas C diretamente via FFI para operações de baixo nível.

### ✅ Processamento de Dados
Processe arquivos, APIs e bancos de dados com pipelines reativos.

### ✅ Ferramentas Interativas
Construa CLIs e TUIs com entrada integrada e formatação rica.

---

## 🏗️ Arquitetura

```
Arquitetura A-lang
├── Lexer (Logos) - Tokenização
├── Parser (Chumsky) - Geração de AST
├── Interpretador (Rust) - Motor de execução
├── Depurador de Viagem no Tempo - Gerenciamento de snapshots
├── Sistema Reativo - Rastreamento de dependências
├── Camada FFI (libloading) - Interop com C
├── Biblioteca Padrão - 80+ funções
├── Backend (Axum/Hyper) - HTTP/WebSocket
└── Módulo IoT - Abstração de hardware
```

**Construído em Rust** para:
- Segurança de memória
- Abstrações de custo zero
- Performance extremamente rápida
- Concorrência sem medo

---

## 📊 Performance

- **Tempo de Inicialização**: ~50ms
- **Execução**: 1M+ operações/seg
- **Memória**: ~10MB base + tamanho do script
- **Overhead FFI**: ~50-100ns por chamada

---

## 🌍 Suporte de Plataforma

| Plataforma | Status | Recursos |
|----------|--------|----------|
| **Linux (x86_64)** | ✅ Completo | Todos os recursos incluindo FFI |
| **macOS (Intel/ARM)** | ✅ Completo | Todos os recursos incluindo FFI |
| **Windows 10/11** | ⚠️ Parcial | Tudo exceto FFI (em breve) |
| **Raspberry Pi** | 🔜 Em breve | Recursos IoT otimizados |

---

## 📦 O Que Está Incluído

### Exemplos (15+)
- `hello.al` - Olá mundo
- `input_demo.al` - Exemplos de entrada do usuário ⭐ NOVO
- `ffi_demo.al` - Chamadas de função FFI C ⭐ NOVO
- `reactive_counter.al` - Variáveis reativas
- `rest_api_example.al` - Servidor HTTP
- `iot_complete_example.al` - Recursos IoT
- E mais...

### Biblioteca Padrão
- **Matemática**: abs, min, max, floor, ceil, round
- **String**: len, split, join, replace, trim
- **Array**: push, pop, slice, indexOf, includes
- **I/O de Arquivo**: readFile, writeFile, appendFile
- **Rede**: httpGet, httpPost, fetch
- **Sistema**: exec, getEnv, timestamp
- **Banco de Dados**: Suporte MySQL
- **IoT**: GPIO, I2C, SPI, UART

---

## 🛣️ Roadmap

### v1.1 (Q1 2025)
- ✅ Suporte FFI para Windows
- ✅ Builds ARM/Raspberry Pi
- ✅ Gerenciadores de pacote (brew, apt)
- ✅ Mais assinaturas de tipo FFI
- ✅ Melhorias de performance

### v1.2 (Q2 2025)
- 🔮 Sintaxe async/await
- 🔮 Sistema de módulos
- 🔮 Expansão da biblioteca padrão
- 🔮 Melhores mensagens de erro

### v2.0 (Q3 2025)
- 🔮 Language Server Protocol (LSP)
- 🔮 Integrações IDE (VS Code, Vim)
- 🔮 Protocolo de depurador
- 🔮 Estabilidade para produção

---

## 🤝 Contribuindo

Aceitamos contribuições! Veja como:

1. Faça um fork do repositório
2. Crie uma branch de feature
3. Faça suas alterações
4. Adicione testes
5. Envie um pull request

Veja [CONTRIBUTING.md](CONTRIBUTING.md) para detalhes.

---

## 📝 Licença

Licença MIT - veja o arquivo [LICENSE](LICENSE).

---

## 🌟 Por Que A-lang?

### ✅ **Experiência do Desenvolvedor**
- Sintaxe familiar estilo JavaScript
- Depuração poderosa com viagem no tempo
- REPL interativo
- Mensagens de erro ricas

### ✅ **Recursos Modernos**
- Programação reativa integrada
- FFI para integração com C
- Capacidades full-stack
- Pronto para IoT

### ✅ **Performance**
- Execução powered by Rust
- Compilação nativa
- Abstrações de custo zero
- Inicialização rápida

### ✅ **Versatilidade**
- De scripting a programação de sistema
- De backends web a dispositivos IoT
- De processamento de dados a automação
- Tudo entre esses extremos

---

## 💬 Comunidade

- **GitHub**: [github.com/A-The-Programming-Language/a-lang](https://github.com/A-The-Programming-Language/a-lang)
- **Discussões**: [GitHub Discussions](https://github.com/A-The-Programming-Language/a-lang/discussions)
- **Issues**: [Reportar bugs](https://github.com/A-The-Programming-Language/a-lang/issues)
- **Twitter**: [@alang_dev](https://twitter.com/alang_dev)

---

## 🙏 Agradecimentos

Construído com essas tecnologias incríveis:
- **Rust** - Linguagem de programação de sistemas
- **Logos** - Gerador de lexer
- **Chumsky** - Combinador de parser
- **Tokio** - Runtime assíncrono
- **Axum** - Framework web
- **libloading** - Carregamento de biblioteca dinâmica

---

## 📖 Aprenda Mais

- [📚 Documentação](DOCUMENTATION.md) - Referência completa da linguagem
- [🎯 Exemplos](examples/) - 15+ exemplos funcionando
- [🔧 Compilando do Código Fonte](BUILD.md) - Compile você mesmo
- [🗺️ Roadmap](ROADMAP.md) - Planos futuros

---

## 🎉 Comece Agora!

```bash
# Instalar
sudo dpkg -i alang_1.0-preview_amd64.deb

# Rodar REPL
alang

# Experimentar exemplos
alang examples/input_demo.al
alang examples/ffi_demo.al
alang examples/reactive_counter.al

# Escrever seu primeiro script
echo 'print("Olá da A-lang!")' > hello.al
alang hello.al
```

---

**Construído com ❤️ pela equipe A-lang**

*"O futuro do scripting está aqui, e ele pode viajar no tempo."*

---

**Versão**: 1.0-preview | **Lançado**: Dezembro 2024 | **Licença**: MIT