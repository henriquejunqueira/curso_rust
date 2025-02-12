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
