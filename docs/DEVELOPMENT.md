# Desenvolvimento Rust + Tauri

## Estrutura

- `crates/cm0304-core`: leitor binário, contêiner, banco, atributos e transações de edição.
- `crates/cm0304-cli`: inspeção, pesquisa, exportação e execução de planos JSON.
- `apps/desktop/src-tauri`: aplicativo Tauri e sessão de edição em memória.
- `apps/desktop/ui`: interface TypeScript/CSS, compilada para `dist`.
- `src`, `tests`, `tools`: prova de conceito JavaScript e verificadores de comparação.

## Comandos

```powershell
npm install
npm run build:desktop
./target/debug/cm0304-desktop.exe
cargo test
cargo clippy -p cm0304-core -p cm0304-cli --all-targets -- -D warnings
npm run verify:parity
```

Os comandos npm de build usam Cargo offline porque as dependências já estão disponíveis neste ambiente. Em uma máquina nova, execute `cargo fetch` com acesso à rede antes do build offline. O desenvolvimento exige Rust, ferramentas de compilação C++ do Windows, WebView2, Node.js e TypeScript. O executável não usa Node.js em produção; a distribuição e a instalação em máquina limpa ainda são etapas futuras.

Os testes com amostras reais usam os arquivos em `game/user data/games`. Eles não são incluídos no repositório. A falta das amostras causa falha nesses testes, em vez de sucesso sem executá-los.

## Planos de edição

```json
[
  { "uid": 2001642, "attribute": "passing", "value": 20 },
  { "uid": 9888, "attribute": "pace", "value": 19 }
]
```

```powershell
cargo run -p cm0304-cli -- edit "origem.cm4" "nova_copia.cm4" "plano.json"
```

Campos repetidos, não suportados, sem alteração efetiva ou não representáveis são recusados. A transação é validada integralmente antes da gravação. A origem e qualquer destino existente são preservados. O temporário é escrito no diretório de destino, sincronizado, relido e finalizado sem sobrescrita.

Na interface, alterações de um mesmo campo substituem a intenção anterior na sessão; o plano final contém uma intenção por UID/campo. Desfazer/refazer opera sobre esses planos. A cópia usa o snapshot aberto e recusa uma origem modificada externamente durante a sessão.

Para combinar campos e remoções estruturais, use `apply-plan` com um objeto:

```json
{
  "edits": [{ "uid": 356, "attribute": "state.morale", "value": 20 }],
  "healInjuries": [356],
  "clearSuspensions": [5640794]
}
```

```powershell
cargo run --offline -p cm0304-cli -- apply-plan "origem.cm4" "nova_copia.cm4" "transacao.json"
cargo run --offline -p cm0304-cli -- health "origem.cm4" "5640794"
cargo run --offline -p cm0304-cli -- health "origem.cm4" "uid:1873"
cargo run --offline -p cm0304-cli -- health-cases "origem.cm4"
```

`health` mostra os registros brutos de uma entidade; `health-cases` inventaria as variantes estruturais presentes, com contagens e até três exemplos por grupo. Os discriminadores brutos não representam automaticamente duração ou categoria. Consulte `reports/v0.5-suspensions.md` para os casos conferidos no jogo e as variantes ainda pendentes. Remover suspensão preserva os cartões; remover lesão preserva condição e ritmo.

As consultas `players`, `career` e `health` aceitam `uid:<número>` para igualdade exata. Sem o prefixo, a pesquisa é textual e pode retornar UIDs que apenas contenham os dígitos informados.

## Diagnóstico de staff

Para antecipar os efeitos de uma CA de staff, sem gravar o save:

```powershell
cargo run --offline -p cm0304-cli -- staff-ability "origem.cm4" 4950 100 preserveValues
cargo run --offline -p cm0304-cli -- staff-ability "origem.cm4" 4950 100 preserveRelative
```

A política é obrigatória. Preservar valores exige inversa exata; preservar bytes mostra atributos recalculados. Esta é uma prévia de diagnóstico, ainda sem escrita de CA. Exemplos e verificação em `reports/v0.6-staff-ability.md`.

```powershell
cargo run --offline -p cm0304-cli -- staff "origem.cm4" "Braga"
cargo run --offline -p cm0304-cli -- staff "origem.cm4" "uid:4950"
```

Leitura separada do modelo de jogadores, com vínculos de clube/seleção, atributos brutos e convertidos. Oito atributos profissionais usam a CA própria de staff; os demais são diretos. A conversão possui 24 observações de controle em `reports/v06-staff-conversion-observed.json`.

A escrita experimental no núcleo aceita `staffEdits` em `apply-plan`, com UID, `attribute` e `value` de 1 a 20. Os 11 campos permitidos estão em `staff_editor::EDITABLE`; CA/PA e personalidade ainda não têm escrita de staff. O plano de exemplo está em `reports/v06-staff-edit-plan.json`. O relatório separa `staffChanges` das alterações de jogador e mantém offsets da origem, antes de remoções estruturais. Campos desconhecidos, repetidos, referências compartilhadas e alvos não representáveis são recusados. A interface prepara essas edições por `stage_staff_edits`, com revisão e histórico compartilhados. A escrita permanece experimental até passar pelo ciclo de carregamento, conferência, evolução e recarga no jogo.

O frontend de consulta também pode ser verificado em navegador isolado com IPC controlado; comandos e limites da prova em `reports/v0.6-staff-ui.md`. Isso não substitui a conferência no aplicativo Tauri com save real.

## Referências de Tauri

- [Comandos Rust](https://v2.tauri.app/develop/calling-rust/)
- [Configuração do aplicativo](https://v2.tauri.app/reference/config/)
- [Diálogos nativos](https://v2.tauri.app/plugin/dialog/)
