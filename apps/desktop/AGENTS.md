# Desktop Tauri e TypeScript

## Estrutura e contratos

- Edite o frontend em `ui/main.ts`, `ui/index.html` e `ui/style.css`. `dist/` é gerado por `npm run build:ui`.
- O frontend usa TypeScript e DOM/CSS diretamente. Siga essa estrutura antes de adicionar frameworks ou dependências.
- `src-tauri/src/main.rs` mantém os comandos IPC e a sessão do desktop. Regras de formato e edição pertencem a `cm0304-core`.
- Ao alterar um comando, confira nomes, argumentos, serialização, tipos TypeScript e respostas simuladas em `tools/check-staff-ui.playwright.js`.
- Preserve limites de importação/exportação de planos, verificação da origem por hash e compatibilidade das estruturas persistidas ou documente explicitamente qualquer mudança.

## Fluxo do usuário

- Preserve rascunho, prévia, revisão, efeitos derivados, desfazer/refazer e gravação em nova cópia.
- Ao abrir outro save, evite reutilizar seleção, cache ou rascunho da sessão anterior. Preserve as proteções contra ações concorrentes e fechamento durante gravação.
- Mostre apenas operações elegíveis para os registros carregados. Explique indisponibilidade e erros nos dois idiomas da interface, distinguindo ausência de dados de formato não suportado. Use as chaves em português e traduza o texto em `ui/en.ts`.
- Mantenha unidades e vínculos claros: valores financeiros usam libras semanais quando aplicável; contrato proprietário e empréstimo são vínculos distintos.
- Preserve navegação por teclado, rótulos dos controles, estados desabilitados e mensagens de progresso/erro nos fluxos alterados.
- Mantenha permissões Tauri restritas ao necessário. Não acrescente acesso amplo ao filesystem para contornar um problema localizado.

## Verificação

Execute na raiz `npm run build:ui` e, para mudanças de comportamento, `npm run test:ui`. A suíte usa Edge headless e IPC simulado; `CM0304_UI_BROWSER=chrome` seleciona Chrome.

Se alterar Rust/IPC, confira também `cargo test --offline --workspace` e `npm run build:desktop`. Os testes simulados não comprovam diálogos do Windows, comportamento do WebView2, escrita real ou aceitação pelo CM0304. Registre separadamente o que foi conferido no aplicativo nativo e no jogo.

Empacotamento segue `docs/DEVELOPMENT.md`. Não incremente versões nem gere instaladores por padrão em tarefas sem escopo de release.
