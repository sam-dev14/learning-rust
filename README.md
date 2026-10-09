# Estudos com Rust

## Conceitos básicos

### 1. Rust

* É uma linguagem de programação compilada de baixo nível.

### 2. Compilação manual

Para compilar e executar um arquivo Rust manualmente, utilize os seguintes comandos:

```bash
rustc "nome_do_arquivo.rs"
./main
```

* `rustc`: compila o arquivo Rust e gera um binário executável.
* `./main`: executa o binário gerado.

### 3. Cargo

É o gerenciador de pacotes do Rust e também uma ferramenta de build, responsável por compilar e gerenciar projetos.

#### Criar projetos com Cargo

Para criar um novo projeto, utilize:

```bash
cargo new "nome_do_repositorio"
```

#### Compilar e executar automaticamente

Para compilar e executar o código, utilize:

```bash
cargo run
```

Se você alterar o código, basta executar `cargo run` novamente. O Cargo recompila o que for necessário e executa o programa.

Outra opção é utilizar:

```bash
cargo build
```

Esse comando compila o projeto, mas não executa o programa. Depois, você pode executar o binário gerado manualmente.

#### Cargo new

O comando `cargo new` cria um novo diretório para o projeto com a estrutura inicial do Cargo.

Os principais arquivos e diretórios são:

* **`Cargo.toml`**: arquivo de configuração do projeto. Contém informações como nome, versão, edição do Rust e dependências externas.
* **`src/`**: diretório onde fica o código-fonte.
* **`src/main.rs`**: arquivo principal de um programa executável em Rust.

Por padrão, o Cargo também inicializa um repositório Git para o projeto, caso não seja desativado ou já exista um repositório apropriado.

#### Cargo init

O comando `cargo init` inicializa um projeto Cargo no diretório atual, aproveitando um diretório que já existe.

```bash
cargo init
```

#### Cargo check

Verifica se o projeto apresenta erros de compilação sem gerar o executável final. É útil para verificar o código rapidamente durante o desenvolvimento.

```bash
cargo check
```

#### Cargo build --release

Compila o projeto com otimizações voltadas para desempenho em produção.

```bash
cargo build --release
```

O executável otimizado é gerado no diretório `target/release/`.
