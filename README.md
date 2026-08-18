# 💼 Wallet Live — Carteira de Investimentos em Rust

> Aplicação web fullstack desenvolvida em **Rust** para gerenciamento de uma carteira de investimentos.
>
> O projeto faz parte do **Santander Bootcamp 2026 — Rust AI Developer** e evoluiu de um fluxo básico de compras para uma aplicação com autenticação, histórico de transações, compra e venda de ativos, persistência em PostgreSQL e área administrativa protegida.

[![Rust](https://img.shields.io/badge/Rust-2024-orange?logo=rust)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/Axum-0.8-blue)](https://github.com/tokio-rs/axum)
[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-18-blue?logo=postgresql)](https://www.postgresql.org/)
[![SQLx](https://img.shields.io/badge/SQLx-0.9-blueviolet)](https://github.com/launchbadge/sqlx)
[![Askama](https://img.shields.io/badge/Askama-templates-green)](https://github.com/rinja-rs/askama)

---

## 🚀 Visão geral

A **Wallet Live** permite autenticar usuários, consultar os ativos disponíveis, acompanhar a carteira e registrar movimentações de compra e venda.

A aplicação também possui uma área administrativa para manutenção do cadastro de ativos.

### Funcionalidades atuais

- 🔐 autenticação de usuários;
- 🍪 sessão por cookie HTTP-only;
- 📈 listagem dos ativos disponíveis;
- 💰 registro de **compra (BUY)** de ativos;
- 💸 registro de **venda (SELL)** de ativos;
- 📊 atualização da quantidade mantida na carteira;
- 🧾 histórico de movimentações por ativo;
- 🕒 data/hora da operação em formato legível;
- 🛡️ validação para impedir venda superior à quantidade disponível;
- 🔑 autenticação administrativa;
- 🧰 CRUD administrativo de ativos;
- 🚫 proteção contra exclusão de ativo que possua histórico;
- 🧪 testes automatizados com `sqlx::test`;
- 🐘 PostgreSQL executado localmente com Docker Compose.

---

## 🆕 Destaque da versão: compra e venda

A principal evolução desta branch é transformar a antiga operação de compra em uma **movimentação de carteira**.

Cada operação agora possui um tipo explícito:

```text
BUY  → aumenta a quantidade mantida
SELL → reduz a quantidade mantida
```

O tipo é persistido no banco por meio do enum PostgreSQL `asset_operation`, com os valores `BUY` e `SELL`. fileciteturn35file0L2-L2

### Regra de venda

Uma venda somente é aceita quando o usuário possui quantidade suficiente do ativo. Caso contrário, a API retorna `400 Bad Request` com o erro `Insufficient Quantity`. fileciteturn26file0L2-L2 fileciteturn34file0L2-L2

Exemplo conceitual:

```text
Carteira
Bitcoin: 0.50

SELL 0.60 BTC
      ↓
❌ operação recusada
Insufficient Quantity

SELL 0.30 BTC
      ↓
✅ operação aceita
Bitcoin: 0.20
```

O teste de integração da operação cobre exatamente esse fluxo: tentativa de venda sem saldo, compra de 0,5, tentativa de venda de 0,6 e venda válida de 0,3, terminando com 0,2 unidades na carteira. fileciteturn26file0L2-L2

---

## 🧾 Histórico de transações

O histórico deixou de representar apenas compras e passou a representar **operações da carteira**.

Cada registro contém:

| Campo | Descrição |
|---|---|
| `operation_type` | `BUY` ou `SELL` |
| `occurred_at` | data/hora da operação |
| `unit_value` | valor unitário informado |
| `quantity_bought` | quantidade movimentada |
| `value_delta` | variação calculada para a movimentação |

O modelo `TransactionHistory` formaliza essa estrutura e serializa a data/hora em ISO 8601. fileciteturn27file0L2-L2

Na interface, o usuário pode expandir cada ativo para visualizar seu histórico, incluindo **tipo da transação**, quantidade, valor unitário, data e variação. fileciteturn29file0L2-L2

---

## 🧭 Fluxo da aplicação

```text
                    ┌─────────────────┐
                    │     Browser     │
                    └────────┬────────┘
                             │
              ┌──────────────┴──────────────┐
              │                             │
        Área do usuário              Área administrativa
              │                             │
              ▼                             ▼
        Login / sessão              Login de administrador
              │                             │
              ▼                             ▼
        /assets                    /admin/assets
              │
       ┌──────┴──────┐
       │             │
      BUY           SELL
       │             │
       ▼             ▼
    aumenta       valida saldo
    quantidade        │
                      ▼
                   diminui
                   quantidade
              │
              ▼
        Histórico
              │
              ▼
         PostgreSQL
```

A implementação do fluxo público está concentrada no handler de portfolio e utiliza os repositories de assets e de ativos pertencentes ao usuário. fileciteturn26file0L2-L2

> **Nota:** apesar de a implementação estar organizada em `portfolio.rs`, a rota HTTP atualmente utilizada pela interface é `/assets`. O módulo de rotas registra `GET /assets` e `POST /assets`. fileciteturn28file0L2-L2

---

## 📡 Rotas

### Usuário

| Método | Rota | Função |
|---|---|---|
| `GET` | `/` | Entrada da aplicação |
| `GET` | `/login` | Exibe a tela de login |
| `POST` | `/login` | Autentica ou cadastra usuário |
| `GET` | `/logout` | Encerra a sessão |
| `GET` | `/assets` | Exibe carteira, ativos e histórico |
| `POST` | `/assets` | Registra uma operação `BUY` ou `SELL` |

O módulo de login registra as rotas `/`, `/login` e `/logout`. fileciteturn32file0L2-L2

### Administração

| Método | Rota | Função |
|---|---|---|
| `GET` | `/admin/login` | Tela de login administrativo |
| `POST` | `/admin/login` | Autenticação administrativa |
| `GET` | `/admin/logout` | Encerra sessão administrativa |
| `GET` | `/admin/assets` | Lista ativos |
| `POST` | `/admin/assets` | Cadastra ativo |
| `POST` | `/admin/assets/{id}` | Atualiza ativo |
| `POST` | `/admin/assets/{id}/delete` | Exclui ativo quando permitido |

---

## 🛡️ Regras de negócio

### Compra

Uma operação `BUY` registra a quantidade adquirida e aumenta a posição do usuário no ativo.

### Venda

Uma operação `SELL` primeiro verifica a quantidade atualmente mantida pelo usuário. Se a quantidade solicitada for maior que a posição disponível, a operação é rejeitada. fileciteturn26file0L2-L2

### Exclusão administrativa

Um ativo com histórico não pode ser excluído. O erro correspondente é convertido para `409 Conflict`, preservando a integridade histórica da carteira. fileciteturn34file0L2-L2

```text
Ativo inexistente       → 404 Not Found
Venda sem quantidade    → 400 Bad Request
Ativo com histórico     → 409 Conflict
Operação válida         → processamento normal
```

---

## 🏗️ Arquitetura

```mermaid
flowchart TD
    Browser[🌐 Browser] --> Login[🔐 Login]
    Browser --> Assets[📈 /assets]
    Browser --> Admin[🛡️ /admin]

    Login --> Axum[⚙️ Axum]
    Assets --> Axum
    Admin --> Axum

    Axum --> Handlers[🎯 Handlers]
    Handlers --> Auth[🔑 Auth]
    Handlers --> Repositories[🗄️ Repositories]
    Handlers --> Templates[🖼️ Askama]

    Repositories --> SQLx[SQLx]
    SQLx --> PostgreSQL[(🐘 PostgreSQL)]
```

A estrutura atual separa autenticação, rotas, handlers, modelos, repositories e templates, permitindo que a funcionalidade de portfolio evolua sem concentrar toda a lógica em um único módulo.

---

## 📂 Estrutura do projeto

```text
.
├── migrations/
│   ├── ...
│   └── 20260818125714_add_operation_type_to_owned_assets.*.sql
├── src/
│   ├── auth/
│   │   ├── admin.rs
│   │   └── user.rs
│   ├── handlers/
│   │   ├── admin.rs
│   │   ├── assets.rs
│   │   ├── login.rs
│   │   └── portfolio.rs
│   ├── models/
│   │   ├── asset.rs
│   │   ├── owned_asset.rs
│   │   └── transaction_history.rs
│   ├── repositories/
│   │   ├── assets.rs
│   │   ├── owned_assets.rs
│   │   └── users.rs
│   ├── routes/
│   │   ├── admin.rs
│   │   ├── assets.rs
│   │   ├── login.rs
│   │   └── portfolio.rs
│   ├── app.rs
│   ├── error.rs
│   └── main.rs
├── templates/
│   ├── admin_assets.html
│   ├── admin_login.html
│   ├── assets.html
│   └── login.html
├── .env
├── .gitignore
├── Cargo.toml
├── Cargo.lock
├── CHANGELOG.md
├── CODE_OF_CONDUCT.md
├── CONTRIBUTING.md
├── LICENSE-APACHE
├── LICENSE-MIT
├── README.md
├── SECURITY.md
└── SUPPORT.md
```

> A estrutura acima representa os módulos relevantes da aplicação. Arquivos auxiliares e demais migrações podem existir além dos itens destacados.

---

## 🛠️ Tecnologias

- **Rust 2024** — linguagem principal;
- **Axum 0.8** — HTTP e roteamento;
- **Tokio** — runtime assíncrono;
- **SQLx 0.9** — acesso ao PostgreSQL e testes de integração;
- **PostgreSQL 18.6 Alpine** — banco de dados local;
- **Askama 0.16** — templates server-side;
- **axum-extra** — cookies;
- **JWT Simple** — autenticação;
- **password-auth** — autenticação por senha;
- **Serde / Serde JSON** — serialização;
- **dotenvy** — variáveis de ambiente;
- **thiserror** — tratamento tipado de erros;
- **tracing / tracing-subscriber** — observabilidade básica;
- **Insta** — suporte a testes/snapshots;
- **Docker Compose** — ambiente local.

As versões e dependências declaradas atualmente estão no `Cargo.toml`. fileciteturn20file0L2-L2

---

## 📦 Pré-requisitos

- Rust com suporte à Edition 2024;
- Cargo;
- Docker e Docker Compose;
- SQLx CLI para executar migrações manualmente.

Instalação do SQLx CLI:

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

---

## ▶️ Como executar

### 1. Clone o projeto

```bash
git clone https://github.com/josearodrigues/rust-fullstack-carteira-investimentos.git
cd rust-fullstack-carteira-investimentos
```

Se estiver trabalhando especificamente nesta feature:

```bash
git checkout feat/portfolio-buy-sell
```

### 2. Configure o ambiente

Crie um `.env` local com:

```env
DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres
ADMIN_SECRET_KEY=seu-token-admin
```

> Não publique credenciais reais no repositório. O `.env` existente na branch contém configuração de desenvolvimento e deve ser tratado como arquivo sensível.

### 3. Suba o PostgreSQL

```bash
docker compose -f compose.yml up -d
```

O Compose atual utiliza PostgreSQL `18.6-alpine3.24` e um volume persistente para `/var/lib/postgresql`. fileciteturn25file0L2-L2

### 4. Execute as migrações

```bash
sqlx migrate run
```

A feature de compra/venda adiciona o tipo PostgreSQL `asset_operation` e a coluna `operation_type` em `owned_assets`. fileciteturn35file0L2-L2

### 5. Inicie a aplicação

```bash
cargo run
```

### 6. Acesse

- Login: `http://localhost:3000/login`
- Carteira: `http://localhost:3000/assets`
- Administração: `http://localhost:3000/admin/login`

---

## 🧪 Testes

Execute:

```bash
cargo test
```

Os testes que exercitam persistência utilizam `sqlx::test`, portanto o PostgreSQL precisa estar disponível. A funcionalidade de compra/venda possui teste para validar venda sem quantidade suficiente, compra, venda parcial e quantidade final da posição. fileciteturn26file0L2-L2

### Checklist recomendado

```bash
cargo fmt -- --check
cargo check
cargo test
```

---

## 🐳 Docker e PostgreSQL

Subir:

```bash
docker compose -f compose.yml up -d
```

Verificar:

```bash
docker compose -f compose.yml ps
docker compose -f compose.yml logs db
```

Parar:

```bash
docker compose -f compose.yml down
```

Para recriar o banco local do zero:

```bash
docker compose -f compose.yml down -v
docker compose -f compose.yml up -d
sqlx migrate run
```

> ⚠️ `down -v` remove o volume do PostgreSQL e, consequentemente, os dados locais persistidos.

---

## 🧯 Troubleshooting

### `Connection refused`

Confirme se o container está ativo:

```bash
docker compose -f compose.yml ps
docker compose -f compose.yml exec db pg_isready -U postgres
```

### Migração não executada

Verifique o `DATABASE_URL` e execute:

```bash
sqlx migrate run
```

### Venda recusada

Se a aplicação retornar:

```text
Insufficient Quantity
```

verifique a quantidade atualmente mantida do ativo. A implementação bloqueia uma venda que exceda essa quantidade. fileciteturn26file0L2-L2

---

## 📚 Evolução do projeto

### Etapa anterior — Administração de Assets

A aplicação ganhou uma área administrativa protegida para criação, consulta, atualização e exclusão de ativos, com proteção contra remoção de ativos que possuem histórico.

### Etapa atual — Portfolio Buy/Sell

A evolução desta branch amplia o conceito de compra para **operações de carteira**:

- criação do enum `BUY` / `SELL`;
- persistência do tipo da operação;
- novo modelo `TransactionHistory`;
- compra e venda na mesma interface;
- validação de quantidade disponível para venda;
- atualização da posição após operações;
- histórico com tipo de transação;
- testes automatizados dos cenários de compra e venda;
- reorganização de handlers e rotas para acomodar o fluxo de portfolio.

O compare entre `main` e esta branch mostra, além dos arquivos de documentação da raiz, mudanças concentradas justamente nesses componentes de portfolio, histórico, repositórios e migração do banco.

---

## 🎓 O que este projeto demonstra

Este projeto exercita conceitos importantes de Rust e desenvolvimento backend/fullstack:

- programação assíncrona com Tokio;
- construção de aplicações web com Axum;
- extractors e estado compartilhado;
- autenticação e cookies;
- JWT e hashing de senhas;
- templates server-side com Askama;
- persistência relacional com PostgreSQL;
- migrações e testes com SQLx;
- modelagem de operações de domínio;
- tratamento explícito de erros HTTP;
- testes de integração;
- separação entre rotas, handlers, modelos e repositories;
- Docker para ambiente de desenvolvimento.

---

## 🗺️ Próximos passos sugeridos

- [ ] adicionar validações de entrada para valores e quantidades não positivos;
- [ ] evitar `f64` para cálculos monetários críticos, avaliando representação decimal apropriada;
- [ ] reforçar testes de autorização e cenários negativos;
- [ ] adicionar CI com `cargo fmt`, `cargo check` e `cargo test`;
- [ ] melhorar a gestão de sessão e cookies para produção;
- [ ] adicionar proteção CSRF aos formulários autenticados;
- [ ] evoluir a autenticação administrativa para usuários/roles persistidos;
- [ ] adicionar métricas e observabilidade estruturada;
- [ ] preparar deploy com PostgreSQL gerenciado.

---

## 📄 Documentação e colaboração

A raiz do projeto também contém documentos de apoio para colaboração, segurança, suporte e histórico de mudanças:

- `CHANGELOG.md` — histórico de alterações;
- `CONTRIBUTING.md` — orientações para contribuição;
- `SECURITY.md` — política de segurança;
- `SUPPORT.md` — suporte;
- `CODE_OF_CONDUCT.md` — código de conduta.

Esses arquivos acompanham a evolução do projeto e devem permanecer alinhados com a natureza da aplicação Wallet Live.

---

## 📄 Licença

Projeto desenvolvido para fins educacionais durante o **Santander Bootcamp 2026 — Rust AI Developer**.

Os arquivos `LICENSE-MIT` e `LICENSE-APACHE` fazem parte do repositório.

---

<p align="center">
  <strong>Rust + Axum + PostgreSQL + Askama</strong><br>
  Da compra à venda: uma carteira de investimentos evoluindo com Rust. 🦀📈
</p>
