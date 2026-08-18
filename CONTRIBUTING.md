# Contribuindo

Obrigado pelo interesse em contribuir com este projeto!

Toda contribuição é bem-vinda, seja ela uma correção de bug, melhoria na documentação, novos testes ou implementação de funcionalidades.

## Como contribuir

1. Faça um Fork do projeto.
2. Crie uma branch para sua alteração.

```bash
git checkout -b minha-feature
```

3. Faça suas alterações.

4. Execute as verificações antes do commit.

```bash
cargo fmt

cargo clippy --all-targets --all-features

cargo test
```

5. Faça um commit.

```bash
git commit -m "Adiciona validação XYZ"
```

6. Envie sua branch.

```bash
git push origin minha-feature
```

7. Abra um Pull Request.

---

## Padrões do projeto

Este projeto procura seguir:

- Rust API Guidelines
- Rust Style Guide
- Idiomatic Rust
- Zero dependências quando possível

---

## Código

- Prefira código simples.
- Evite duplicação.
- Documente toda API pública.
- Inclua testes para novas funcionalidades.

---

## Documentação

Toda função pública deve possuir documentação (`rustdoc`).

Sempre que possível inclua exemplos executáveis.

---

Obrigado por contribuir!