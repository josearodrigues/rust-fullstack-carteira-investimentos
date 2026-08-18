# Changelog

Todas as mudanças relevantes deste projeto serão documentadas neste arquivo.

O formato é baseado em [Keep a Changelog](https://keepachangelog.com/) e este projeto segue o [Versionamento Semântico (SemVer)](https://semver.org/lang/pt-BR/).

## [Unreleased]

### Added

### Changed

### Fixed

## [0.2.0] - 2026-07-17

### Added

- Implementação da validação de CNPJ.
- Testes de integração para CNPJ.
- Exemplos de utilização do validador de CNPJ.
- Refatoração da lógica comum para o módulo interno `utils`.

### Changed

- Reorganização da estrutura interna da biblioteca para facilitar a inclusão de novos validadores.

## [0.1.2] - 2026-07-17

### Changed

- Renomeados exemplos e documentação para refletir o novo nome da crate.
- Melhorias no README.

## [0.1.1] - 2026-07-17

### Added

- Implementação da validação de CPF.
- Documentação completa da API (`rustdoc`).
- Testes de integração.
- Exemplo de utilização da biblioteca.
- Banner SVG para o README.
- Arquivos `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md` e `SUPPORT.md`.
- Licenciamento duplo (`MIT OR Apache-2.0`).
- Workflow de Integração Contínua (GitHub Actions).

### Changed

- Reestruturação completa do `README.md`.
- Melhorias na organização do projeto.
- Preparação para publicação no crates.io.