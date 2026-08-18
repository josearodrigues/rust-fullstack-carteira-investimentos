# Contribuindo

Obrigado pelo interesse em contribuir com a **Wallet Live**!

Contribuições são bem-vindas, especialmente correções, testes, melhorias de documentação e novas funcionalidades relacionadas ao projeto.

## Como contribuir

1. Faça um fork do projeto.
2. Crie uma branch para sua alteração:

```bash
git checkout -b minha-feature
```

3. Faça as alterações.
4. Execute as verificações locais:

```bash
cargo fmt -- --check
cargo check
cargo test
```

5. Faça um commit descrevendo a alteração.
6. Envie a branch:

```bash
git push origin minha-feature
```

7. Abra um Pull Request.

## Padrões do projeto

Procure manter:

- código Rust idiomático e simples;
- responsabilidades separadas entre rotas, handlers, models e repositories;
- regras de negócio protegidas por testes;
- mensagens de erro claras;
- documentação atualizada quando uma funcionalidade mudar o comportamento da aplicação.

## Banco de dados

Alterações no schema devem incluir a migração correspondente (`.up.sql` e `.down.sql`) e testes adequados quando aplicável.

## Novas funcionalidades

Ao adicionar uma funcionalidade, procure atualizar:

- `README.md`;
- `CHANGELOG.md`;
- testes;
- documentação adicional da raiz, quando necessário.

Não inclua segredos, credenciais ou arquivos de ambiente pessoais em commits.
