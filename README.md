# rust-broadcast-buffer

## Preparação do ambiente

Instale as dependencias e os executáveis do Rust e Cargo pelo [link](https://www.rust-lang.org/pt-BR/learn/get-started).

## Execução

Ao clonar o repositório, basta utilizar o comadno *cargo run* com os parâmetros descritos para iniciar a tarefa.

````bash
cargo run T P C I
````
Onde:
- **T** é a **C**apacidade (número de posições) do Buffer;
- **P** número de **P**rodutores;
- **C** número de **C**onsumidores;
- **I** número máxima de **I**nserções feito por cada produtor;

ex: 
````bash
cargo run 16 4 3 2
````

## Testes de Unidade

É possível validar as propriedades do Buffer a seguir utilizando o comando de testes:

```bash
cargo test
```

O resumo do comportamento do buffer pode ser descrito pelas regras de negócio descritas a seguir:
- Cada Buffer é inicializado com um número de Produtores e Consumidores fixos;
- O Buffer é construido através de uma fila (FIFO) de dados;
- Cada produtor observa uma fila (*nxt_free*) que disponibiliza a próxima posição disponível para escrita, garantindo a ordenação;
- Cada posição é ocupada por uma estrutura de dado com um contador (*bdata.falta_ler*) que sinaliza quantos consumidores faltam ler o dado na posição;
- Quando um produtor aloca um dado na fila o contador (*falta_ler*) é setado para o número de consumidores;
- Quando o contador chega a zero a posiçao é desalocada e inserida na fila de posições (*nxt_free*);
- Cada consumidor possui uma fila (*nxt_data[meu_id]*) preenchida com as posições em ordem de leitura;
- Quando um consumidor realiza uma leitura a posição sai da sua fila (*nxt_data[meu_id]*);
- Os produtores adicionam as posições a cada deposito em cada componente (*nxt_data[meu_id]*) para leitura;



