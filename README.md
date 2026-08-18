# wallet_live

Aplicacao web em Rust para gerenciar uma carteira de ativos. O projeto permite autenticar usuarios, listar ativos, registrar compras e consultar o historico de cada ativo comprado.

## O que o projeto faz

- Exibe uma tela de login simples.
- Permite autenticar ou cadastrar um usuario.
- Mostra os ativos disponiveis e os ativos comprados.
- Registra novas compras e calcula o resultado de cada movimentacao.
- Disponibiliza rotas administrativas para criar, atualizar e listar ativos.

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

Implementei a protecao do acesso admin com `Bearer token` vindo do ambiente e corrigi o fluxo de historico de compras, login e mensagens de erro para deixar o app funcional e mais consistente.

## Como testar minha versao

```bash
cargo test
```

Se quiser validar o fluxo manualmente:

1. Rode a aplicacao.
2. Acesse a tela de login.
3. Entre com um usuario.
4. Verifique a pagina de ativos.
5. Teste o registro de compra e a area administrativa.

## O que eu aprendi

- A integrar Rust com Postgres usando SQLx.
- A estruturar rotas, extratores e erros com Axum.
- A renderizar telas server-side com Askama.
- A organizar autenticacao com cookie e token.
- A manter o projeto simples, mas com fluxo completo de ponta a ponta.
