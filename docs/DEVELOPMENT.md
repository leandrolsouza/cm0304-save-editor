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

## Referências de Tauri

- [Comandos Rust](https://v2.tauri.app/develop/calling-rust/)
- [Configuração do aplicativo](https://v2.tauri.app/reference/config/)
- [Diálogos nativos](https://v2.tauri.app/plugin/dialog/)
