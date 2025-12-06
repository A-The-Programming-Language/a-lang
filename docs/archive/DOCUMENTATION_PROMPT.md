# 📚 Prompt para Criação de Documentação A-lang (Estilo Angular.dev)

## Objetivo

Criar uma documentação web interativa, moderna e completa para a linguagem de programação **A-lang**, inspirada na excelência do site angular.dev, mas focada em uma linguagem de script revolucionária com 5 features únicas.

---

## Contexto da A-lang

A-lang é uma linguagem de script moderna escrita em Rust que combina:
- ✅ Sintaxe JavaScript familiar (sem let/var, parênteses obrigatórios em if/while/for)
- ✅ Performance nativa (Rust)
- ✅ 5 Features revolucionárias únicas: Time-Travel Debugging, Reactive Variables, Runtime Syntax Extensions, Smart Auto-Parallelization, Context-Aware Type System
- ✅ Backend completo (HTTP Server Express-like, WebSocket, MySQL)
- ✅ IoT Ready (GPIO, I2C, SPI, UART)
- ✅ Biblioteca padrão rica (Math, String, Array, System, Network, Binary)

**Status**: 106/109 testes passando, produção-ready

---

## Requisitos da Documentação

### 1. **Estrutura e Navegação**

Crie uma documentação web com:

- **Homepage Impactante**
  - Hero section com animação chamativa
  - Destaque para os 5 WOW factors
  - Quick start em 3 passos
  - Comparação com outras linguagens (tabela)
  - Casos de uso (IoT, Backend, Scripts, Automação)

- **Navegação Lateral** (Sidebar)
  - Introduction
    - O que é A-lang?
    - Por que A-lang?
    - Primeiros Passos
  - Fundamentos
    - Sintaxe Básica
    - Tipos de Dados
    - Operadores
    - Estruturas de Controle
    - Funções
    - Arrays e Objetos
  - Features Únicas (5 WOW)
    - Time-Travel Debugging
    - Reactive Variables
    - Runtime Syntax Extensions
    - Smart Auto-Parallelization
    - Context-Aware Type System
  - Biblioteca Padrão
    - Math
    - String & Array
    - System & I/O
    - Network & HTTP
    - Binary & Encoding
  - Backend
    - HTTP Server (Express-like)
    - WebSocket
    - MySQL Database
    - REST API Examples
  - IoT & Hardware
    - GPIO
    - I2C Communication
    - SPI Communication
    - UART Serial
    - Complete IoT Examples
  - Avançado
    - Tratamento de Erros
    - Closures
    - Pattern Matching
    - Performance Tips
  - API Reference (alfabética)
  - Examples & Recipes
  - FAQ

- **Barra de Busca Inteligente**
  - Busca rápida por função, conceito, exemplo
  - Sugestões instantâneas
  - Atalho de teclado (Ctrl+K ou Cmd+K)

### 2. **Design e UX**

**Elementos Visuais:**

- **Modo Claro/Escuro** (toggle no header)
- **Tipografia Limpa** (fonte sans-serif moderna, código em monospace)
- **Cores:**
  - Primary: Azul/Roxo moderno (#6366f1 ou similar)
  - Success: Verde (#10b981)
  - Warning: Amarelo (#f59e0b)
  - Danger: Vermelho (#ef4444)
  - Code background: Cinza claro/escuro conforme tema

- **Componentes Interativos:**
  - Code snippets com syntax highlighting (A-lang custom syntax)
  - Botão "Copy" em cada bloco de código
  - Tabs para múltiplos exemplos
  - Alertas/Callouts coloridos (Info, Warning, Tip, Important)
  - Animações sutis em hover
  - Scroll suave

- **Layout Responsivo:**
  - Desktop: Sidebar fixa à esquerda, conteúdo central, TOC à direita
  - Tablet: Sidebar colapsável
  - Mobile: Menu hamburger, navegação touch-friendly

### 3. **Conteúdo e Estrutura de Páginas**

Cada página deve ter:

**Cabeçalho da Página:**
```
# [Título do Tópico]
[Descrição breve em 1-2 linhas]
```

**Seções Organizadas:**
- Introdução/Overview
- Sintaxe/API
- Exemplos Práticos (do simples ao complexo)
- Use Cases Reais
- Tips & Best Practices
- Related Topics (links para páginas relacionadas)

**Elementos Especiais:**

- 🔥 **Live Code Editor** (opcional, mas incrível!)
  - Editor in-browser com execução A-lang
  - Output console
  - Exemplos pré-carregados

- 💡 **Callouts/Alerts:**
  ```
  ℹ️ INFO: Informação adicional útil
  ⚠️ WARNING: Atenção a este detalhe importante
  💡 TIP: Dica profissional
  ⚡ IMPORTANT: Parênteses são obrigatórios em if/while/for!
  ✅ SUCCESS: Exemplo de código que funciona
  ❌ ERROR: Código incorreto ou anti-pattern
  ```

- 📊 **Comparações Visuais:**
  - Tabelas comparando A-lang vs Python/JavaScript/Lua
  - Before/After code snippets
  - Performance benchmarks (gráficos)

- 🎯 **Interactive Examples:**
  - Sliders para ajustar valores
  - Buttons para executar código
  - Visualizações de Time-Travel Debugging
  - Grafo de Reactive Dependencies

### 4. **Páginas Especiais**

#### **Homepage**

```
[HERO SECTION - Full viewport]
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🚀 A-lang: The Revolutionary Scripting Language

Fast. Powerful. Unique.

The only language with Time-Travel Debugging, 
Reactive Variables, and Auto-Parallelization built-in.

[Button: Get Started] [Button: See Examples]
[Install: curl -sSf https://alang.dev/install.sh | sh]
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

[5 WOW FACTORS - Grid de Cards com ícones animados]
⏰ Time-Travel Debugging
⚡ Reactive Variables
🎨 Runtime Syntax Extensions
🔮 Smart Auto-Parallelization
🧠 Context-Aware Type System

[QUICK START - 3 Passos]
1. Install → 2. Write → 3. Run

[CODE EXAMPLE - Com syntax highlighting]
// Hello World in A-lang
name = "World"
print("Hello, " + name + "!")

[WHY A-LANG? - Feature Comparison Table]
| Feature              | Lua | Python | JS  | A-lang |
|---------------------|-----|--------|-----|--------|
| Performance         | ⭐⭐⭐⭐ | ⭐⭐  | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| Time-Travel Debug   | ❌   | ❌     | ❌   | ✅     |
| Reactive Variables  | ❌   | ❌     | ❌   | ✅     |
| IoT Support         | ⚠️   | ⚠️     | ⚠️   | ✅     |
| Backend Framework   | ⚠️   | ✅     | ✅   | ✅     |

[USE CASES - Cards com ícones]
🏠 IoT & Smart Home
🌐 Web Backend APIs
🤖 Automation Scripts
📊 Data Processing
🎮 Game Scripting

[COMMUNITY & ECOSYSTEM]
[GitHub Stars] [Discord] [Twitter] [Playground]

[FOOTER]
Made with ❤️ and Rust 🦀
```

#### **Getting Started (Quick Start)**

```
# Getting Started with A-lang

⚡ Get up and running in 3 minutes!

## Installation

[TABS: Linux | macOS | Windows | Docker]

### Linux/macOS
```bash
curl -sSf https://alang.dev/install.sh | sh
```

### Windows
```powershell
irm https://alang.dev/install.ps1 | iex
```

### Verify Installation
```bash
alang --version
# Expected: A-lang v2.0.0
```

## Your First Program

1. Create a file `hello.al`:
```javascript
name = "Developer"
print("Hello, " + name + "!")
```

2. Run it:
```bash
alang hello.al
```

3. See the output:
```
Hello, Developer!
```

## Next Steps

- 📖 [Learn the Syntax](./syntax-basics)
- 🎯 [Try Interactive Examples](./playground)
- 🚀 [Build Your First API](./backend/express-like)
- 💡 [Explore WOW Features](./features/time-travel)
```

#### **Time-Travel Debugging (Feature Showcase)**

```
# ⏰ Time-Travel Debugging

Go back in time and inspect your program's execution history!

## What is Time-Travel Debugging?

A-lang has **built-in time-travel debugging** that lets you:
- ✅ Save snapshots of your program state
- ✅ Rewind execution to any point
- ✅ Create named checkpoints
- ✅ Replay code from any snapshot
- ✅ Inspect historical states

❌ **No external debugger needed!**

## Quick Example

[INTERACTIVE DEMO - Animated visualization]

```javascript
x = 10
snapshot              // 💾 Save state

x = x + 5
print(x)              // Prints: 15

rewind 1              // ⏪ Travel back in time
print(x)              // Prints: 10 - we're back!
```

## Commands

| Command | Description | Example |
|---------|-------------|---------|
| `snapshot` | Take a snapshot of current state | `snapshot` |
| `checkpoint "name"` | Create named checkpoint | `checkpoint "start"` |
| `rewind N` | Go back N snapshots | `rewind 5` |
| `rewind to "name"` | Jump to named checkpoint | `rewind to "start"` |

## Real-World Use Cases

### 1. Debugging Loops
[CODE EXAMPLE]

### 2. Algorithm Visualization
[CODE EXAMPLE]

### 3. State Inspection
[CODE EXAMPLE]

## Best Practices

💡 TIP: Use named checkpoints for important points in your code
⚠️ WARNING: Too many snapshots can consume memory
✅ BEST: Strategic snapshot placement

## Performance

- Zero overhead when disabled
- Minimal impact with sparse snapshots
- Can be toggled at runtime

[BENCHMARK CHART]

## Learn More

- [Advanced Time-Travel Patterns](./advanced/time-travel)
- [API Reference: Time-Travel](./api/time-travel)
```

### 5. **Code Snippets - Syntax Highlighting**

**Regras de Syntax Highlighting para A-lang:**

```
Keywords (roxo/azul): fn, if, elif, else, while, for, in, try, catch, finally, throw, return, break, continue, match, const, reactive, snapshot, checkpoint, rewind, parallel, effect, syntax, context

Built-in Functions (amarelo/laranja): print, len, push, pop, keys, values, range, type_of, int, float, str, abs, min, max, floor, ceil, round, split, join, sleep, timestamp

Operators (cinza/branco): +, -, *, /, %, =, ==, !=, <, >, <=, >=, &&, ||, !, +=, -=, *=, /=, ++, --

Strings (verde): "texto", 'texto'

Numbers (laranja): 42, 3.14, -10

Comments (cinza claro/escuro): //, /* */

Functions (azul claro): greet, add, factorial

Arrow Functions (azul): =>, x => x * 2
```

### 6. **API Reference Structure**

Para cada função da stdlib, crie uma página com:

```
# abs()

Retorna o valor absoluto de um número.

## Syntax
```javascript
abs(x)
```

## Parameters

- **x** (Number): O número do qual calcular o valor absoluto

## Return Value

**Number**: O valor absoluto de `x`

## Examples

### Basic Usage
```javascript
result = abs(-15)
print(result)  // 15

result = abs(15)
print(result)  // 15
```

### With Variables
```javascript
temperature = -5
absolute = abs(temperature)
print(absolute)  // 5
```

### In Calculations
```javascript
difference = abs(10 - 25)
print(difference)  // 15
```

## See Also

- [min()](#min) - Find minimum value
- [max()](#max) - Find maximum value
- [Math Functions Overview](#math-functions)
```

### 7. **Examples & Recipes**

Crie uma galeria de exemplos práticos:

```
# Examples & Recipes

## By Category

### 🎯 Beginner
- Hello World
- Variables & Types
- Control Flow
- Functions Basics

### 🚀 Intermediate
- Array Manipulation
- Object Operations
- Error Handling
- Recursive Functions

### 💪 Advanced
- Time-Travel Debugging
- Reactive State Management
- Parallel Processing
- Custom Syntax Extensions

### 🌐 Backend
- REST API Server
- WebSocket Chat
- MySQL CRUD
- Authentication Middleware

### 🏠 IoT
- LED Blink
- Temperature Sensor
- I2C Communication
- Complete IoT Device

## Featured Example: Todo API

[FULL CODE WITH ANNOTATIONS]
```javascript
// Complete REST API for Todo Management
app = createExpressApp()
todos = []

app.get("/todos", fn(req, res) {
    res.json(todos)
})

app.post("/todos", fn(req, res) {
    todo = req.body
    todos = push(todos, todo)
    res.status(201).json(todo)
})

app.listen(3000)
```

[COPY BUTTON] [OPEN IN PLAYGROUND]
```

### 8. **FAQ Page**

```
# Frequently Asked Questions

## General

### What is A-lang?
A-lang is a revolutionary scripting language with unique features...

### Why should I use A-lang?
...

### Is A-lang production-ready?
Yes! 106/109 tests passing, used in production by...

## Syntax

### Why are parentheses required in if/while/for?
For consistency and clarity. This is JavaScript-style syntax...

### Do I need to use semicolons?
No, semicolons are optional...

### What's the difference between reactive and normal variables?
...

## Features

### How does Time-Travel Debugging work?
...

### Can I really create new syntax at runtime?
Yes! (Feature in development)...

### Is Auto-Parallelization safe?
Yes, the runtime only parallelizes proven-safe operations...

## Performance

### How fast is A-lang compared to Python/Lua?
[BENCHMARK CHARTS]

### Does Time-Travel have overhead?
Only when enabled, zero overhead when disabled...

## Ecosystem

### What libraries are available?
...

### Can I call C/C++ libraries?
Yes, via FFI...

### Is there VS Code support?
Coming soon! Track progress on GitHub...

## Troubleshooting

### My if statement doesn't work
⚠️ Remember: Parentheses are REQUIRED!
```
❌ if x > 10 {
✅ if (x > 10) {
```

## Community

### How can I contribute?
...

### Where can I get help?
- Discord: discord.gg/alang
- GitHub Issues: github.com/alang/issues
- Stack Overflow: #alang tag
```

---

## Funcionalidades Técnicas da Documentação

### **Search (Busca)**
- Full-text search
- Search por função/conceito/exemplo
- Fuzzy matching
- Keyboard shortcuts (Ctrl+K)
- Recent searches
- Popular searches

### **Navigation**
- Breadcrumbs
- Previous/Next page buttons
- "On This Page" sidebar (TOC)
- Smooth scroll to sections
- URL hash navigation

### **Code Features**
- Syntax highlighting (custom A-lang)
- Copy to clipboard button
- Line numbers (optional)
- Highlight specific lines
- Multi-tab code examples
- Diff view (before/after)

### **Personalization**
- Theme toggle (light/dark)
- Font size adjustment
- Code theme selection
- Bookmark favorite pages
- Progress tracking (completed tutorials)

### **Interactive Elements**
- Live code playground (opcional)
- Interactive diagrams
- Animated examples
- Tooltips on hover
- Expandable sections

### **SEO & Performance**
- Static site generation (SSG)
- Fast page loads
- Optimized images
- Sitemap.xml
- robots.txt
- Meta tags
- Open Graph tags
- Schema.org markup

---

## Stack Tecnológica Recomendada

### Framework Base
- **Next.js** (React) - Para SSG e routing
- **Docusaurus** - Framework específico para docs
- **VitePress** - Alternativa Vue-based
- **Astro** - Para performance máxima

### Componentes UI
- **Tailwind CSS** - Styling
- **Radix UI** / **HeadlessUI** - Componentes acessíveis
- **Framer Motion** - Animações

### Code Highlighting
- **Prism.js** ou **Shiki** - Com custom A-lang grammar

### Search
- **Algolia DocSearch** (free for open source)
- **Pagefind** (static search)

### Deploy
- **Vercel** / **Netlify** - Deploy automático
- **Cloudflare Pages** - Alternativa

---

## Estrutura de Arquivos Sugerida

```
docs/
├── src/
│   ├── pages/
│   │   ├── index.mdx                 # Homepage
│   │   ├── getting-started.mdx
│   │   ├── syntax/
│   │   │   ├── basics.mdx
│   │   │   ├── variables.mdx
│   │   │   ├── operators.mdx
│   │   │   └── ...
│   │   ├── features/
│   │   │   ├── time-travel.mdx
│   │   │   ├── reactive.mdx
│   │   │   └── ...
│   │   ├── stdlib/
│   │   │   ├── math.mdx
│   │   │   ├── string.mdx
│   │   │   └── ...
│   │   ├── backend/
│   │   │   ├── http-server.mdx
│   │   │   ├── websocket.mdx
│   │   │   └── ...
│   │   ├── iot/
│   │   │   ├── gpio.mdx
│   │   │   ├── i2c.mdx
│   │   │   └── ...
│   │   ├── api/
│   │   │   └── reference.mdx
│   │   ├── examples/
│   │   │   └── recipes.mdx
│   │   └── faq.mdx
│   ├── components/
│   │   ├── CodeBlock.tsx
│   │   ├── Alert.tsx
│   │   ├── Tabs.tsx
│   │   ├── LiveEditor.tsx (opcional)
│   │   └── ...
│   ├── styles/
│   │   ├── globals.css
│   │   └── syntax-theme.css
│   └── config/
│       ├── navigation.ts
│       └── prism-alang.ts
├── public/
│   ├── images/
│   ├── icons/
│   └── fonts/
├── package.json
├── next.config.js (ou similar)
└── README.md
```

---

## Exemplos de Componentes React

### CodeBlock Component
```tsx
<CodeBlock language="alang" showLineNumbers copyButton>
{`fn greet(name) {
    return "Hello, " + name + "!"
}

print(greet("World"))`}
</CodeBlock>
```

### Alert Component
```tsx
<Alert type="warning">
⚠️ IMPORTANT: Parênteses são obrigatórios em if/while/for!
</Alert>

<Alert type="tip">
💡 TIP: Use named checkpoints for important points
</Alert>
```

### Tabs Component
```tsx
<Tabs>
  <Tab label="JavaScript">
    ```javascript
    const x = 10;
    console.log(x);
    ```
  </Tab>
  <Tab label="A-lang">
    ```alang
    x = 10
    print(x)
    ```
  </Tab>
</Tabs>
```

### Comparison Component
```tsx
<Comparison>
  <Before title="Python">
    ```python
    def add(a, b):
        return a + b
    ```
  </Before>
  <After title="A-lang">
    ```alang
    fn add(a, b) {
        return a + b
    }
    ```
  </After>
</Comparison>
```

---

## Checklist de Funcionalidades

### Must-Have (MVP)
- [ ] Homepage impactante
- [ ] Getting Started completo
- [ ] Documentação de sintaxe básica
- [ ] Documentação das 5 features únicas
- [ ] API Reference completa
- [ ] Search funcional
- [ ] Navigation sidebar
- [ ] Dark/Light mode
- [ ] Code syntax highlighting
- [ ] Copy button em códigos
- [ ] Responsivo (mobile/tablet/desktop)
- [ ] SEO otimizado

### Nice-to-Have (V2)
- [ ] Live code playground
- [ ] Interactive examples
- [ ] Video tutorials
- [ ] User comments/discussions
- [ ] Version selector (v1.0, v2.0, etc)
- [ ] Multilingual (EN, PT, ES)
- [ ] PDF export
- [ ] Offline mode (PWA)
- [ ] AI-powered chatbot assistant
- [ ] Community contributions section

---

## Inspiração de Design

**Referências visuais (estilo dessas docs, mas adaptado para A-lang):**
- ✅ **Angular.dev** - Clean, moderno, bem organizado
- **React.dev** - Interativo, didático
- **Vue.js docs** - Simples e elegante
- **Rust book** - Profundo e bem estruturado
- **Tailwind CSS** - Visual impactante
- **Stripe docs** - API reference exemplar

**Cores e Branding A-lang:**
- Primary: Roxo/Azul vibrante (#6366f1)
- Accent: Laranja (#f59e0b) para WOW factors
- Success: Verde (#10b981)
- Background: Branco/Cinza muito claro (light mode), Dark navy (dark mode)
- Code: Tema One Dark ou similar

**Logo/Ícone A-lang:** (sugestão)
- Letra "A" estilizada com elementos de:
  - Relógio (time-travel)
  - Raio (performance)
  - Circuito (IoT)

---

## Tom e Voz da Documentação

**Tom:**
- 🎯 **Profissional mas acessível**
- 💡 **Educativo e encorajador**
- 🚀 **Entusiasmado sobre as features únicas**
- 🤝 **Amigável para iniciantes, respeitoso com experts**

**Voz:**
- Use "você" ao se dirigir ao leitor
- Seja direto e objetivo
- Evite jargões desnecessários
- Explique conceitos complexos com analogias
- Use emojis com moderação (✅ Sim, 🎉 mas não abuse 😅)

**Exemplos de bom estilo:**
✅ "A-lang makes debugging easy with built-in time-travel."
✅ "You can go back in time and inspect your program's state."
✅ "No external debugger needed!"

❌ "A-lang utilizes advanced temporal navigation paradigms."
❌ "One must configure the debugging apparatus appropriately."

---

## Métricas de Sucesso

Após lançamento, a documentação deve ter:

- ⚡ **Performance:** Lighthouse score > 95
- 📊 **Engagement:** Time on page > 3min
- 🔍 **Search:** < 3 cliques para qualquer informação
- 📱 **Mobile:** 100% funcional em mobile
- ♿ **Accessibility:** WCAG 2.1 AA compliant
- 🌍 **SEO:** Ranking top 3 para "A-lang documentation"

---

## Próximos Passos

1. ✅ **Escolher stack tecnológica** (Next.js + MDX recomendado)
2. ✅ **Setup projeto base** com estrutura de arquivos
3. ✅ **Implementar Homepage** com hero e WOW factors
4. ✅ **Criar componentes reutilizáveis** (CodeBlock, Alert, etc)
5. ✅ **Escrever conteúdo core** (Getting Started, Syntax, Features)
6. ✅ **Implementar search** (Algolia ou Pagefind)
7. ✅ **Configurar syntax highlighting** para A-lang
8. ✅ **Deploy em Vercel/Netlify**
9. ✅ **Coletar feedback** e iterar
10. ✅ **Adicionar features V2** (playground, etc)

---

## Conclusão

Esta documentação deve ser:
- 🎨 **Visualmente impressionante** (como Angular.dev)
- 📚 **Completa e organizada** (fácil de navegar)
- 💡 **Educativa e prática** (learn by doing)
- ⚡ **Rápida e responsiva** (performance top)
- 🚀 **Destacar as 5 features únicas** (nosso diferencial)

**Objetivo final:** Fazer qualquer desenvolvedor pensar "WOW, essa linguagem é incrível e a documentação é fantástica!" nos primeiros 5 minutos de leitura.

---

**Made with ❤️ for A-lang developers**