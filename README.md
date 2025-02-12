# Repositório com projetos e exercícios do curso de Rust da HCode

### Comandos utilizados

- Compilando arquivo rust .rs: `$ rust ./main.rs`
- Executando arquivo rust .rs: `$ ./main`

- Criando projeto utilizando o gerenciador de pacotes cargo: `$ cargo new nome_projeto`
- Executando o projeto com cargo: `$ cargo run`
- Cria uma versão release do projeto: `$ cargo run --release`
- Exibe ajuda: `$ cargo run --help`

- Criando projeto utilizando cargo sem o comando new: `$ cargo init`

### Anotações

#### Variáveis

- Rust, uma linguagem de programação focada em segurança e desempenho, possui um sistema de gerenciamento de memória inovador e robusto. O entendimento das variáveis e da mutabilidade é fundamental para trabalha eficientemente com Rust.
- Em Rust, as variáveis são declaradas utilizando a palavra-chave let. O tipo da variável pode ser explicitamente indicado ou inferido pelo compilador.

#### Mutabilidade

- A mutabilidade em Rust é explicitamente controlada. Por padrão, as variáveis são imutáveis, o que significa que não podem ser modificadas após a sua primeira atribuição.
- Para tornar uma variável mutável, utiliza-se a palavra-chave mut. O controle rigoroso sobre a mutabilidade em Rust contribui para a segurança do código. A prevenção contra mutações não autorizadas ajuda a evitar erros comuns, como race conditions e data traces.

#### Propriedade Única (Ownership)

- Rust introduz o conceito de propriedade única para gerenciar a alocação de memória. Cada valor tem exatamente um "proprietário". Quando o proprietário sai de escopo, o valor é liberado automaticamente.

```rust
let texto = String::from("hello");
let texto2 = texto;
```

#### Referências e Empréstimos

- Para referenciar valores sem transferir a propriedade, Rust utiliza referências. Elas podem ser mutáveis ou imutáveis.

```rust
let mut valor = 50;
let referencia = &valor; // Referência imutável
let referencia_mut = &mut valor;
```

#### Rust Standard Library

- A Rust Standard Library (Biblioteca Padrão de Rust), muitas vezes referida como std, é um conjunto de módulos e tipos que acompanham a linguagem Rust.
- Ela oferece funcionalidades essenciais para o desenvolvimento de software em Rust, fornecendo implementações para tipos comuns, operações de E/S (Entrada/Saída), manipulação de strings, concorrência, coleções, entre outras utilidades.

#### std

- A std é uma parte fundamental da experiência de desenvolvimento em Rust e é automaticamente incluída em todos os programas Rust por padrão.

#### Principais Características

- Tipos Básicos: A std define tipos básicos, como bool, char, i32, u64, f32, etc.
- Coleções: Oferece implementações de coleções como vetores (Vec), fatias (&[T]), hashes (HashMap), conjuntos (HashSet), listas duplamente vinculadas (LinkedList), entre outros.
- Operações de E/S: Fornece funcionalidades para entrada e saída, incluindo leitura de arquivos, manipulação de diretórios, e operações básicas de E/S.
- Concorrência: Oferece suporte a programação concorrente com tipos e mecanismos como std::thread, std::sync, std::mpsc (canaleta de vários produtores, único consumidor), etc.
- Manipulação de Strings: Inclui funcionalidades para manipulação e formatação de strings, incluindo tipos como String, str, e módulos como std::fmt.
- Gerenciamento de Memória: Fornece abstrações para gerenciamento de memória, incluindo tipos como Box, Vec, Rc, Arc, etc.
- Tempo e Data: Inclui tipos e funcionalidades para manipulação de tempo e data, como std::time e std::chrono.
- Matemática: Oferece funcionalidades matemáticas, incluindo std::f64, std::f32, std::num, etc.
- Padrões de Comportamento: Define traços (traits) e implementações padrão para muitos comportamentos comuns, como std::cmp, std::ops, std::iter, entre outros.
