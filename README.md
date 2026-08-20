# 💼 Wallet Live — Carteira de Investimentos em Rust

> Aplicação web fullstack desenvolvida em **Rust** para gerenciamento de uma carteira de investimentos.
>
> Projeto desenvolvido durante o **Santander Bootcamp 2026 — Rust AI Developer**, evoluindo de um fluxo básico de compra de ativos para uma aplicação com autenticação, carteira, compra e venda, histórico de operações, dashboard, gráficos, área administrativa e persistência em PostgreSQL.

[![Rust](https://img.shields.io/badge/Rust-2024-orange?logo=rust)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/Axum-0.8-blue)](https://github.com/tokio-rs/axum)
[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-18-blue?logo=postgresql)](https://www.postgresql.org/)
[![SQLx](https://img.shields.io/badge/SQLx-0.9-blueviolet)](https://github.com/launchbadge/sqlx)
[![Askama](https://img.shields.io/badge/Askama-templates-green)](https://github.com/rinja-rs/askama)
[![Chart.js](https://img.shields.io/badge/Chart.js-charts-orange)](https://www.chartjs.org/)

---

## 📑 Índice

- [Visão geral](#-visão-geral)
- [Funcionalidades](#-funcionalidades)
- [Dashboard](#-dashboard)
- [Compra, venda e histórico](#-compra-venda-e-histórico)
- [Rotas](#-rotas)
- [Regras de negócio](#-regras-de-negócio)
- [Arquitetura](#-arquitetura)
- [Estrutura do projeto](#-estrutura-do-projeto)
- [Tecnologias](#-tecnologias)
- [Pré-requisitos](#-pré-requisitos)
- [Como executar](#-como-executar)
- [Testes](#-testes)
- [Docker e PostgreSQL](#-docker-e-postgresql)
- [Troubleshooting](#-troubleshooting)
- [Evolução do projeto](#-evolução-do-projeto)
- [O que este projeto demonstra](#-o-que-este-projeto-demonstra)

---

## 🚀 Visão geral

A **Wallet Live** permite autenticar usuários, consultar os ativos disponíveis, acompanhar a carteira e registrar operações de compra e venda.

A aplicação possui quatro áreas principais:

- **Login e autenticação** de usuários;
- **Carteira**, com compra, venda e histórico das operações;
- **Dashboard**, com resumo patrimonial, distribuição da carteira e evolução do patrimônio;
- **Administração**, protegida por autenticação administrativa, para manutenção dos ativos.

---

## ✨ Funcionalidades

### 👤 Usuário

- 🔐 autenticação de usuários;
- 🍪 sessão por cookie HTTP-only;
- 📝 cadastro automático quando o usuário ainda não existe;
- 📈 visualização dos ativos disponíveis;
- 💰 compra de ativos;
- 💸 venda de ativos;
- 🛡️ validação de quantidade disponível antes da venda;
- 🧾 histórico de operações por ativo;
- 🇧🇷 valores monetários em formato brasileiro.

### 📊 Dashboard

O dashboard apresenta:

- patrimônio atual;
- total investido;
- rentabilidade percentual;
- quantidade de ativos;
- quantidade de operações;
- tabela dos ativos mantidos;
- **distribuição da carteira** em gráfico de barras horizontal;
- **evolução do patrimônio** em gráfico de linha.

Os dados dos gráficos são calculados no backend em `src/handlers/dashboard.rs`, serializados em JSON e renderizados no frontend com **Chart.js**.

### 🛡️ Administração

- autenticação administrativa;
- cadastro, listagem, atualização e exclusão de ativos;
- proteção contra exclusão de ativos que possuem histórico.

---

## 💰 Compra, venda e histórico

Cada movimentação da carteira possui um tipo explícito:

```text
BUY  → aumenta a quantidade mantida
SELL → reduz a quantidade mantida
```

O tipo é persistido no PostgreSQL por meio do enum `asset_operation`.

### Regra de venda

Uma venda somente é aceita quando o usuário possui quantidade suficiente do ativo.

```text
Carteira
Bitcoin: 0,50

SELL 0,60 BTC
      ↓
❌ operação recusada
Insufficient Quantity

SELL 0,30 BTC
      ↓
✅ operação aceita
Bitcoin: 0,20
```

### Histórico

O modelo `TransactionHistory` contém:

| Campo | Descrição |
|---|---|
| `operation_type` | `BUY` ou `SELL` |
| `occurred_at` | data/hora da operação |
| `unit_value` | valor unitário informado |
| `quantity_bought` | quantidade movimentada |
| `value_delta` | variação calculada da operação |

Na interface, as operações são apresentadas em português como **COMPRA** e **VENDA**.

---

## 📡 Rotas

### Usuário

| Método | Rota | Função |
|---|---|---|
| `GET` | `/` | Entrada da aplicação e redirecionamento conforme autenticação |
| `GET` | `/login` | Exibe a tela de login |
| `POST` | `/login` | Autentica ou cadastra o usuário |
| `GET` | `/logout` | Encerra a sessão |
| `GET` | `/assets` | Exibe a carteira e seus ativos |
| `POST` | `/assets` | Registra uma compra/venda |
| `GET` | `/dashboard` | Exibe o dashboard da carteira |

### API de ativos

As rotas abaixo são montadas sob `/api`:

| Método | Rota | Função |
|---|---|---|
| `GET` | `/api/assets` | Lista ativos |
| `POST` | `/api/assets` | Cria ativo |
| `PATCH` | `/api/assets` | Atualiza ativo |
| `DELETE` | `/api/assets` | Exclui ativo |

### Administração web

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

Uma operação `SELL` reduz a posição do usuário. A aplicação bloqueia vendas superiores à quantidade atualmente disponível.

### Exclusão administrativa

Um ativo que possui histórico de operações não pode ser excluído, preservando a integridade histórica da carteira.

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
    Browser --> Wallet[💼 /assets]
    Browser --> Dashboard[📊 /dashboard]
    Browser --> Admin[🛡️ /admin]

    Login --> Axum[⚙️ Axum]
    Wallet --> Axum
    Dashboard --> Axum
    Admin --> Axum

    Axum --> Handlers[🎯 Handlers]
    Handlers --> Auth[🔑 Auth]
    Handlers --> Repositories[🗄️ Repositories]
    Handlers --> Templates[🖼️ Askama]

    Repositories --> SQLx[SQLx]
    SQLx --> PostgreSQL[(🐘 PostgreSQL)]
    Dashboard --> Charts[📊 Chart.js]
```

A aplicação separa responsabilidades entre **rotas**, **handlers**, **autenticação**, **models**, **repositories** e **templates**.

---

## 📂 Estrutura do projeto

```text
.
├── migrations/
├── src/
│   ├── auth/
│   │   ├── admin.rs
│   │   └── user.rs
│   ├── handlers/
│   │   ├── admin.rs
│   │   ├── assets.rs
│   │   ├── dashboard.rs
│   │   ├── login.rs
│   │   ├── wallet.rs
│   │   ├── fixtures/
│   │   └── snapshots/
│   ├── models/
│   │   ├── asset.rs
│   │   ├── owned_asset.rs
│   │   ├── portfolio_summary.rs
│   │   └── transaction_history.rs
│   ├── repositories/
│   │   ├── assets.rs
│   │   ├── owned_assets.rs
│   │   └── users.rs
│   ├── routes/
│   │   ├── admin.rs
│   │   ├── api.rs
│   │   ├── assets.rs
│   │   ├── dashboard.rs
│   │   ├── login.rs
│   │   ├── mod.rs
│   │   └── wallet.rs
│   ├── app.rs
│   ├── error.rs
│   └── main.rs
├── templates/
│   ├── admin_login.html
│   ├── assets.html
│   ├── dashboard.html
│   ├── login.html
│   └── wallet.html
├── compose.yml
├── Cargo.toml
├── Cargo.lock
└── README.md
```

> A estrutura acima destaca os módulos principais. Arquivos auxiliares, snapshots e migrações podem existir além dos itens apresentados.

---

## 🛠️ Tecnologias

- **Rust 2024** — linguagem principal;
- **Axum 0.8** — HTTP e roteamento;
- **Tokio** — runtime assíncrono;
- **SQLx 0.9** — acesso ao PostgreSQL, migrações e testes;
- **PostgreSQL 18** — banco de dados;
- **Askama** — templates server-side;
- **axum-extra** — cookies;
- **JWT Simple** — autenticação baseada em token;
- **password-auth** — hashing e verificação de senhas;
- **Serde / Serde JSON** — serialização;
- **dotenvy** — variáveis de ambiente;
- **thiserror** — tratamento tipado de erros;
- **tracing / tracing-subscriber** — observabilidade básica;
- **Chart.js** — gráficos do dashboard;
- **Docker Compose** — ambiente local;
- **Insta** — suporte a snapshots de testes.

---

## 📦 Pré-requisitos

- Rust com suporte à Edition 2024;
- Cargo;
- Docker e Docker Compose;
- SQLx CLI.

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
git checkout feat/dasboard
```

### 2. Configure o ambiente

Crie um `.env` local com:

```env
DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres
ADMIN_SECRET_KEY=seu-token-admin
```

> Não publique credenciais reais no repositório. O `.env` deve ser tratado como arquivo sensível.

### 3. Suba o PostgreSQL

```bash
docker compose -f compose.yml up -d
```

### 4. Execute as migrações

```bash
sqlx migrate run
```

As migrações incluem a estrutura necessária para registrar o tipo das operações de carteira (`BUY`/`SELL`).

### 5. Inicie a aplicação

```bash
cargo run
```

### 6. Acesse

- Login: `http://localhost:3000/login`
- Carteira: `http://localhost:3000/assets`
- Dashboard: `http://localhost:3000/dashboard`
- Administração: `http://localhost:3000/admin/login`

---

## 🧪 Testes

A suíte atual possui **27 testes automatizados**, distribuídos principalmente entre handlers, repositories e models.

Execute:

```bash
cargo test --all-features
```

### Cobertura atual

- autenticação e login;
- logout e cookies;
- cadastro automático de usuário;
- CRUD de ativos;
- operações administrativas;
- proteção de exclusão de ativos com histórico;
- compra e venda de ativos;
- validação de venda sem quantidade suficiente;
- persistência e leitura do histórico;
- cálculos de patrimônio e rentabilidade;
- renderização do dashboard;
- dados de distribuição da carteira;
- dados de evolução do patrimônio.

Os testes que precisam de banco utilizam `sqlx::test` e os fixtures ficam organizados junto aos handlers que os utilizam.

### Verificação de qualidade

```bash
cargo test --all-features
cargo clippy --all-targets --all-features -- -D warnings
```

Na revisão desta feature, os dois comandos passaram com sucesso: **27 testes passaram e o Clippy não apresentou warnings**.

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

Confirme se o PostgreSQL está ativo:

```bash
docker compose -f compose.yml ps
docker compose -f compose.yml exec db pg_isready -U postgres
```

### Migração não executada

Confirme o `DATABASE_URL` e execute:

```bash
sqlx migrate run
```

### Venda recusada

Se a aplicação retornar:

```text
Insufficient Quantity
```

verifique a quantidade atualmente mantida do ativo. A implementação bloqueia uma venda que exceda essa quantidade.

---

## 📈 Evolução do projeto

### Administração de Assets

A aplicação ganhou uma área administrativa protegida para criação, consulta, atualização e exclusão de ativos, com proteção contra remoção de ativos que possuem histórico.

### Portfolio Buy/Sell

O conceito de compra foi ampliado para representar operações completas de carteira:

- enum `BUY` / `SELL`;
- persistência do tipo da operação;
- modelo `TransactionHistory`;
- compra e venda na mesma interface;
- validação da quantidade disponível;
- atualização da posição após operações;
- histórico detalhado;
- testes dos cenários de compra e venda.

### Dashboard

A aplicação passou a possuir um dashboard dedicado, com resumo patrimonial, total investido, rentabilidade, quantidade de ativos e operações, distribuição percentual da carteira e evolução histórica do patrimônio.

Os gráficos passaram a utilizar dados reais calculados no backend, em vez de dados de exemplo no template.

Também houve reorganização da responsabilidade dos módulos: os testes foram mantidos próximos dos handlers e repositories correspondentes, e os testes de rota que duplicavam a cobertura dos handlers foram removidos.

---

## 🎓 O que este projeto demonstra

Este projeto exercita conceitos importantes de Rust e desenvolvimento backend/fullstack:

- programação assíncrona com Tokio;
- aplicações web com Axum;
- extractors e estado compartilhado;
- autenticação, cookies e JWT;
- hashing de senhas;
- templates server-side com Askama;
- persistência relacional com PostgreSQL;
- migrações e queries com SQLx;
- testes de integração com `sqlx::test`;
- organização de testes por responsabilidade;
- serialização com Serde;
- tratamento tipado de erros;
- cálculos de carteira e rentabilidade;
- preparação de dados para visualização;
- gráficos no frontend com Chart.js;
- Docker Compose para ambiente de desenvolvimento;
- qualidade de código com `cargo fmt`, `cargo test` e `cargo clippy`.

---

## 📚 Próximos passos possíveis

Algumas evoluções naturais para o projeto são:

- melhorar a responsividade do dashboard;
- adicionar filtros de período ao histórico;
- adicionar mais indicadores financeiros;
- criar testes adicionais para cenários de borda da carteira;
- adicionar testes de autenticação de ponta a ponta;
- preparar a aplicação para deploy em ambiente cloud;
- evoluir observabilidade e configuração para produção.

---

## 📄 Licença

Este projeto mantém as licenças **MIT** e **Apache-2.0**, conforme os arquivos de licença presentes no repositório.

---

**Santander Bootcamp 2026 — Rust AI Developer**  
Projeto desenvolvido como parte da jornada de aprendizado em Rust e desenvolvimento fullstack.
