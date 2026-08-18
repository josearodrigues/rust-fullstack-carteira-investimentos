# wallet_live

Aplicacao web em Rust para gerenciar uma carteira de ativos. O projeto permite autenticar usuarios, listar ativos, registrar compras, consultar o historico de cada ativo comprado e administrar o cadastro de ativos em uma area protegida.

## O que o projeto faz

- Exibe uma tela de login simples para usuarios.
- Permite autenticar ou cadastrar um usuario.
- Mostra os ativos disponiveis e os ativos comprados.
- Registra novas compras e calcula o resultado de cada movimentacao.
- Disponibiliza uma area administrativa para login, logout, listagem, criacao, atualizacao e exclusao de ativos.
- Implementa a área de portfolio com rotas /portfolio, compra e venda de ativos, além de testes automatizados.
- Impede a exclusao de um ativo quando ele ja possui historico de compras.

## Como executar a aplicacao

1. Suba o banco de dados:
   ```bash
   docker compose -f compose.yml up -d
   ```

2. Configure as variaveis de ambiente em um arquivo `.env`:
   ```env
   DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres
   ADMIN_SECRET_KEY=seu-token-admin
   ```

3. Rode as migracoes:
   ```bash
   sqlx migrate run
   ```

4. Inicie a aplicacao:
   ```bash
   cargo run
   ```

5. Acesse:
   - `http://localhost:3000/login`
   - `http://localhost:3000/assets`
   - `http://localhost:3000/admin/login`

## Tecnologias usadas

- Rust
- Axum
- SQLx
- PostgreSQL
- Askama
- Tokio
- JWT Simple
- Password Auth
- Tailwind CSS

## Melhoria implementada

Implementei a area administrativa para assets com login, logout, protecao por cookie de admin, CRUD completo e validacao de exclusao quando existe historico de compras.

## Como testar minha versao

```bash
cargo test
```

Os testes usam `sqlx::test` e dependem de um PostgreSQL disponivel. Antes de rodar a suite, suba o banco com `docker compose -f compose.yml up -d` e mantenha as migracoes aplicadas.

As fixtures de teste ficam em `src/routes/fixtures` e `src/handlers/fixtures`, separadas por modulo.

Se quiser validar o fluxo manualmente:

1. Rode a aplicacao.
2. Acesse a tela de login.
3. Entre com um usuario.
4. Verifique a pagina de ativos.
5. Teste o registro de compra.
6. Acesse a area administrativa com o token de admin.
7. Teste criar, atualizar e excluir ativos.

## O que eu aprendi

- A integrar Rust com Postgres usando SQLx.
- A estruturar rotas, extratores e erros com Axum.
- A renderizar telas server-side com Askama.
- A organizar autenticacao com cookie e token.
- A manter o projeto simples, mas com fluxo completo de ponta a ponta.
- A separar a area publica da area administrativa com regras proprias.
