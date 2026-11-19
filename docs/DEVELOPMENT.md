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

```powershell
cargo run --offline -p cm0304-cli -- staff "origem.cm4" "Braga"
cargo run --offline -p cm0304-cli -- staff "origem.cm4" "uid:4950"
```

Leitura separada do modelo de jogadores, com vínculos de clube/seleção e atributos brutos. Não oferece escrita nem conversão validada para 1–20. Evidências e pendências em `reports/v0.6-staff-research.md`.

O frontend de consulta também pode ser verificado em navegador isolado com IPC controlado; comandos e limites da prova em `reports/v0.6-staff-ui.md`. Isso não substitui a conferência no aplicativo Tauri com save real.

## Referências de Tauri

- [Comandos Rust](https://v2.tauri.app/develop/calling-rust/)
- [Configuração do aplicativo](https://v2.tauri.app/reference/config/)
- [Diálogos nativos](https://v2.tauri.app/plugin/dialog/)
