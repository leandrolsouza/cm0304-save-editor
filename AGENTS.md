# Instruções para agentes

## Projeto e referências

Editor local para Windows x64 de saves do CM0304 4.1.5. O aplicativo usa Rust, Tauri 2 e TypeScript; a implementação JavaScript é uma referência de diagnóstico e paridade, não o backend do desktop.

- Leia `README.md` e `docs/DEVELOPMENT.md` para entender os fluxos e comandos.
- Consulte `docs/NEXT.md` e `ROADMAP.md` para continuidade e escopo. Esses arquivos contêm registros históricos: confira o estado atual no código e nas evidências antes de repetir pendências antigas.
- Consulte `research/FORMAT.md` antes de alterar o formato binário; `reports/` registra as validações e seus limites.
- Escreva a documentação pública e as notas de release em inglês. A interface usa inglês por padrão e mantém a tradução em português do Brasil. Preserve os nomes técnicos, as chaves de tradução e o idioma original das evidências históricas.

## Organização

- `crates/cm0304-core`: leitura, conversão, pesquisa e transações de edição.
- `crates/cm0304-cli`: inspeção, exportação, planos e comandos de laboratório.
- `apps/desktop`: frontend e integração Tauri; possui instruções próprias.
- `src/`, `tests/`, `tools/`: referência JavaScript, regressões e ferramentas de desenvolvimento.
- `game/`: instalação e amostras locais não versionadas. Não distribuir saves, executáveis ou bancos do jogo.

## Invariantes de segurança

- Preserve o save de origem e recuse destinos existentes. Grave apenas uma nova cópia validada, com releitura e hashes conforme o fluxo existente.
- Preserve bytes e blocos desconhecidos. Não generalize offsets, variantes ou compatibilidade sem evidência.
- Valide o plano inteiro antes de gravar. Preserve revisão, efeitos derivados, desfazer/refazer e detecção de alteração externa da origem.
- Não aproxime atributos sem representação exata nem ajuste CA automaticamente para viabilizar um alvo. Mudanças de CA devem respeitar a política explícita do usuário.
- Diferencie campo ausente, formato incompatível e valor zero. Não invente valores para preencher dados desconhecidos.
- Preserve alterações locais preexistentes. Não faça limpeza de saves, relatórios ou arquivos não rastreados como parte de uma tarefa comum.

## Verificação

Execute os comandos na raiz e escolha os que cobrem a mudança:

| Área | Comando |
| --- | --- |
| Referência JavaScript | `npm test` |
| Rust core e CLI (membros padrão) | `npm run test:rust` |
| Workspace incluindo Tauri | `cargo test --offline --workspace` |
| Lint Rust | `cargo clippy --offline --workspace --all-targets -- -D warnings` |
| Compilação da interface | `npm run build:ui` |
| Regressão da interface com IPC simulado | `npm run test:ui` |
| Compilação desktop | `npm run build:desktop` |
| Comparação Rust/JavaScript | `npm run verify:parity` |

Node.js 22+ é necessário para as ferramentas. Use `npm ci` quando precisar instalar as dependências do lockfile. Os builds Rust usuais são offline; se faltar cache, informe a dependência em vez de alterar versões para contornar o ambiente.

Testes com saves reais dependem de `game/user data/games`, não incluído no repositório. Se faltarem amostras, informe a limitação; não remova as asserções nem transforme falhas em sucesso silencioso. Testes de UI requerem Edge; Chrome pode ser selecionado por `CM0304_UI_BROWSER=chrome`.

Para alterações de leitura/conversão/escrita compartilhadas com JavaScript, verifique a paridade aplicável. Para documentação isolada, revise conteúdo, caminhos e comandos; não é necessário compilar o aplicativo.

Não edite artefatos gerados em `target/`, `node_modules/`, `apps/desktop/dist/` ou `apps/desktop/src-tauri/gen/`. Atualize README, documentação ou roadmap quando o comportamento ou escopo mudar. Relate os checks executados e diferencie testes automatizados de validação nativa no Tauri e no jogo.
