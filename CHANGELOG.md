# Changelog

Todas as mudanças relevantes da **Wallet** são documentadas neste arquivo.

O formato segue uma adaptação do [Keep a Changelog](https://keepachangelog.com/) e o projeto utiliza [Versionamento Semântico (SemVer)](https://semver.org/lang/pt-BR/) como referência.

## [Unreleased]

### Added

- Suporte a operações de compra (`BUY`) e venda (`SELL`) de ativos.
- Histórico de transações com tipo de operação.
- Validação de quantidade disponível antes de uma venda.
- Testes automatizados para os fluxos de compra e venda.

### Changed

- A operação anteriormente tratada como compra passou a representar uma movimentação de carteira.
- `owned_assets` passou a armazenar `operation_type`.
- A interface de ativos passou a permitir selecionar o tipo da transação.
- A organização dos handlers, rotas e modelos foi ajustada para suportar o fluxo de portfolio.

### Fixed

- Bloqueio de vendas que excedam a quantidade mantida pelo usuário.

## [Previous]

### Administration

- Implementada área administrativa protegida para gerenciamento de assets.
- Adicionados login e logout administrativos.
- Implementado CRUD de assets.
- Impedida a exclusão de assets que possuem histórico de movimentações.
- Tentativas de exclusão bloqueadas retornam `409 Conflict`.
- Adicionados testes e fixtures para os fluxos administrativos.
