# Política de Segurança

## Reportando vulnerabilidades

Caso encontre uma vulnerabilidade de segurança, evite publicar detalhes sensíveis em uma Issue pública.

Abra uma comunicação privada pelo GitHub descrevendo o problema, o impacto e, quando possível, os passos para reprodução.

## Escopo

A Wallet Live é uma aplicação web de gerenciamento de carteira de investimentos, com autenticação, persistência em PostgreSQL e área administrativa.

A aplicação contém mecanismos de autenticação por token/cookie e regras de autorização para a área administrativa. O projeto está em desenvolvimento educacional e não deve ser considerado pronto para uso financeiro em produção.

## Dados sensíveis

- Não publique senhas, tokens administrativos ou outros segredos no repositório.
- Mantenha configurações locais em `.env`.
- Prefira variáveis de ambiente ou um gerenciador de segredos em ambientes reais.
- Se um segredo for exposto, considere-o comprometido e faça sua rotação.

## Produção

Antes de um deployment real, recomenda-se revisar especialmente:

- cookies `Secure` e `SameSite`;
- proteção CSRF;
- expiração e rotação de tokens;
- autenticação e autorização administrativa;
- validação de entrada;
- uso de tipos decimais apropriados para valores monetários;
- logs e exposição de informações sensíveis;
- permissões do banco de dados.
