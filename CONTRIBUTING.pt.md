# Contribuindo para A-lang

Obrigado pelo seu interesse em contribuir com A-lang! 🎉

## 📋 Índice

- [Código de Conduta](#código-de-conduta)
- [Primeiros Passos](#primeiros-passos)
- [Como Contribuir](#como-contribuir)
- [Configuração de Desenvolvimento](#configuração-de-desenvolvimento)
- [Testes](#testes)
- [Processo de Pull Request](#processo-de-pull-request)
- [Padrões de Código](#padrões-de-código)

## Código de Conduta

Este projeto adere ao nosso [Código de Conduta](CODE_OF_CONDUCT.pt.md). Ao participar, espera-se que você cumpra este código.

## Primeiros Passos

1. Faça um fork do repositório
2. Clone seu fork: `git clone https://github.com/SEU_USUARIO/a-lang.git`
3. Crie uma branch: `git checkout -b feature/sua-feature`
4. Faça suas alterações
5. Teste suas alterações
6. Envie um pull request

## Como Contribuir

### 🐛 Reportando Bugs

- Use GitHub Issues
- Inclua versão do A-lang (`alang --version`)
- Forneça código mínimo de reprodução
- Descreva comportamento esperado vs atual
- Inclua mensagens de erro

### 💡 Sugerindo Features

- Abra uma GitHub Discussion primeiro
- Explique o caso de uso
- Forneça exemplos
- Considere compatibilidade retroativa

### 📝 Melhorando Documentação

- Corrija erros de digitação e texto confuso
- Adicione exemplos
- Melhore comentários no código
- Traduza documentação

### 🔧 Contribuições de Código

Áreas onde precisamos de ajuda:
- Melhorias FFI (mais assinaturas de tipo)
- Funções de biblioteca padrão
- Otimizações de performance
- Suporte a plataformas (FFI Windows)
- Correção de bugs

## Configuração de Desenvolvimento

### Pré-requisitos

- Rust 1.70 ou mais recente
- Git
- Um editor de código (VS Code recomendado)

### Compilar do Código Fonte

```bash
# Clonar
git clone https://github.com/yourusername/a-lang.git
cd a-lang

# Compilar
cargo build

# Executar testes
cargo test

# Executar REPL
cargo run

# Executar exemplo
cargo run examples/hello.al
```

## Testes

### Executando Testes

```bash
# Todos os testes
cargo test

# Teste específico
cargo test nome_do_teste

# Com saída
cargo test -- --nocapture

# Testes de integração
cargo test --test '*'
```

### Adicionando Testes

- Testes unitários: Adicione no mesmo arquivo do código
- Testes de integração: Adicione no diretório `tests/`
- Testes de exemplos: Garanta que exemplos em `examples/` funcionem

### Cobertura de Testes

Almejamos 90%+ de cobertura de testes. Por favor adicione testes para:
- Novas features
- Correções de bugs
- Casos extremos

## Processo de Pull Request

1. **Atualizar Testes**: Adicione ou atualize testes para suas mudanças
2. **Atualizar Documentação**: Atualize README, comentários ou docs conforme necessário
3. **Executar Testes**: Garanta que todos os testes passem (`cargo test`)
4. **Formatar Código**: Execute `cargo fmt`
5. **Lint**: Execute `cargo clippy`
6. **Escrever Mensagens de Commit Claras**: Use formato de commits convencionais
7. **Enviar PR**: Preencha o template de PR completamente

### Formato de Mensagem de Commit

```
tipo(escopo): assunto

corpo (opcional)

rodapé (opcional)
```

Tipos:
- `feat`: Nova feature
- `fix`: Correção de bug
- `docs`: Documentação
- `style`: Formatação
- `refactor`: Reestruturação de código
- `test`: Adição de testes
- `chore`: Manutenção

Exemplos:
```
feat(ffi): adiciona suporte para tipos struct
fix(parser): trata literais de array vazios
docs(readme): atualiza instruções de instalação
```

## Padrões de Código

### Código Rust

- Siga o guia de estilo Rust
- Use `cargo fmt` para formatação
- Corrija todos avisos do `cargo clippy`
- Adicione comentários de documentação (`///`) para APIs públicas
- Mantenha funções pequenas e focadas
- Use nomes de variáveis descritivos

### Código A-lang (Exemplos)

- Use sintaxe estilo JavaScript
- Inclua comentários para lógica complexa
- Siga convenções de nomenclatura:
  - Variáveis: `camelCase`
  - Funções: `camelCase`
  - Constantes: `MAIUSCULAS`

### Documentação

- Use linguagem clara e concisa
- Inclua exemplos de código
- Atualize sumário
- Verifique ortografia/gramática

## Estrutura do Projeto

```
a-lang/
├── src/
│   ├── main.rs           # Ponto de entrada REPL
│   ├── lib.rs            # Raiz da biblioteca
│   ├── lexer/            # Tokenizador
│   ├── parser/           # Parser AST
│   ├── interpreter/      # Motor de execução
│   ├── reactive/         # Sistema reativo
│   ├── time_travel/      # Depurador time-travel
│   ├── stdlib/           # Biblioteca padrão
│   └── ...
├── examples/             # Scripts de exemplo
├── tests/                # Testes de integração
└── docs/                 # Documentação
```

## Processo de Revisão

1. **Verificações Automáticas**: CI executa testes e linters
2. **Revisão de Código**: Mantenedores revisam seu código
3. **Feedback**: Atenda comentários da revisão
4. **Aprovação**: Necessária aprovação de pelo menos um mantenedor
5. **Merge**: Faremos merge do seu PR!

## Obtendo Ajuda

- **Perguntas**: Abra uma GitHub Discussion
- **Chat**: Junte-se à nossa comunidade (link TBD)
- **Email**: dev@alang.dev

## Reconhecimento

Contribuidores serão:
- Adicionados ao CONTRIBUTORS.md
- Mencionados nas notas de release
- Listados no website (em breve)

## Licença

Ao contribuir, você concorda que suas contribuições serão licenciadas sob:
- Licença MIT (LICENSE-MIT)
- Licença Apache 2.0 (LICENSE-APACHE)

---

Obrigado por contribuir com A-lang! 🚀
