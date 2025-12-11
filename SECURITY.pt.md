# Política de Segurança

## Versões Suportadas

| Versão | Suportada          |
| ------- | ------------------ |
| 1.0-preview   | :white_check_mark: |
| < 1.0   | :x:                |

## Reportando uma Vulnerabilidade

Levamos a segurança do A-lang a sério. Se você acredita ter encontrado uma vulnerabilidade de segurança, por favor reporte-a conforme descrito abaixo.

### Onde Reportar

**Por favor NÃO reporte vulnerabilidades de segurança através de issues públicas no GitHub.**

Em vez disso, reporte-as por email para: **security@alang.dev**

### O Que Incluir

Por favor inclua as seguintes informações:
- Tipo de problema (ex: buffer overflow, SQL injection, cross-site scripting, etc.)
- Caminhos completos dos arquivos fonte relacionados ao problema
- A localização do código fonte afetado (tag/branch/commit ou URL direto)
- Qualquer configuração especial necessária para reproduzir o problema
- Instruções passo a passo para reproduzir o problema
- Prova de conceito ou código de exploit (se possível)
- Impacto do problema, incluindo como um atacante poderia explorá-lo

### O Que Esperar

- **Confirmação**: Dentro de 48 horas após envio
- **Avaliação Inicial**: Dentro de 1 semana
- **Atualizações de Status**: Toda semana até resolução
- **Resolução**: Divulgação coordenada após patch ser lançado

### Safe Harbor

Apoiamos safe harbor para pesquisadores de segurança que:
- Fazem esforço de boa fé para evitar violações de privacidade, destruição de dados e interrupção de serviço
- Apenas interagem com contas próprias ou com permissão explícita
- Não exploram problema de segurança além do necessário para demonstrá-lo
- Reportam vulnerabilidades prontamente
- Permitem tempo razoável para correção antes de divulgação pública

## Melhores Práticas de Segurança para Usuários

### FFI (Foreign Function Interface)

FFI é inerentemente inseguro. Ao usar FFI:

- ✅ Verifique se caminhos de biblioteca existem
- ✅ Carregue apenas bibliotecas confiáveis
- ✅ Valide todas as assinaturas de função
- ✅ Use try/catch para tratamento de erros
- ⚠️ Nunca passe dados não confiáveis para funções C
- ⚠️ Tenha cuidado com operações de ponteiro

### Validação de Entrada

Ao usar `input()`:

- ✅ Valide e sanitize entrada do usuário
- ✅ Use conversão de tipo (int, float) com cuidado
- ✅ Trate erros de conversão
- ⚠️ Nunca execute entrada não confiável como código

### Operações de Arquivo

- ✅ Valide caminhos de arquivo
- ✅ Use caminhos relativos quando possível
- ✅ Verifique permissões de arquivo
- ⚠️ Evite vulnerabilidades de path traversal
- ⚠️ Sanitize nomes de arquivo de entrada do usuário

### Operações de Rede

- ✅ Valide URLs
- ✅ Use HTTPS quando possível
- ✅ Sanitize dados antes de enviar
- ⚠️ Tenha cuidado com URLs controladas por usuário
- ⚠️ Valide respostas do servidor

## Considerações de Segurança Conhecidas

### Segurança FFI

FFI permite chamar funções C, o que contorna garantias de segurança do A-lang:

- **Segurança de Memória**: Sem verificação automática de limites
- **Segurança de Tipo**: Tipos incorretos podem causar crashes
- **Comportamento Indefinido**: Possível com assinaturas erradas

**Mitigação**: Use FFI apenas com bibliotecas confiáveis e entradas validadas.

### Sistema Reativo

O sistema reativo rastreia dependências:

- **Uso de Memória**: Grafos grandes de dependência podem consumir memória
- **Loops Infinitos**: Dependências circulares são detectadas mas podem impactar performance

**Mitigação**: Mantenha grafos reativos simples e evite aninhamento profundo.

### Depuração Time-Travel

Snapshots armazenam estado do programa:

- **Uso de Memória**: Pode crescer muito com muitos snapshots
- **Dados Sensíveis**: Snapshots podem conter informação sensível

**Mitigação**: Use limites de snapshot e limpe dados sensíveis quando necessário.

## Atualizações de Segurança

Atualizações de segurança serão:
- Lançadas o mais rápido possível após verificação
- Anunciadas em GitHub Security Advisories
- Documentadas em notas de release
- Retroportadas para versões suportadas quando viável

## Perguntas

Para perguntas sobre esta política de segurança, contate: security@alang.dev

---

**Última Atualização**: Dezembro 2024  
**Versão**: 1.0
