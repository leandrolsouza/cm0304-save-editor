# Development

The desktop application targets Windows x64 and uses Rust, Tauri 2 and TypeScript. The JavaScript tools serve as a reference for diagnostics and parity checks.

## Set up and build

Install Node.js 22+, Rust, the Windows C++ build tools and the Microsoft Edge WebView2 Runtime. Run the following from the repository root.

```powershell
npm ci
cargo fetch --locked --target x86_64-pc-windows-msvc
npm run build:desktop
./target/debug/cm0304-desktop.exe
```

The desktop build uses Cargo offline. The installed application needs WebView2 and runs without the development toolchain.

| Path | Purpose |
| --- | --- |
| `crates/cm0304-core` | Save reading, conversion, search and edit transactions |
| `crates/cm0304-cli` | Inspection, exports, plans and research commands |
| `apps/desktop/src-tauri` | Tauri integration, IPC and the editor session |
| `apps/desktop/ui` | TypeScript, HTML and CSS for the desktop interface |
| `src`, `tests`, `tools` | JavaScript reference implementation and development checks |

## Test

| Area | Command |
| --- | --- |
| JavaScript reference | `npm test` |
| Rust workspace, including Tauri | `cargo test --offline --locked --workspace` |
| Rust formatting | `cargo fmt --all -- --check` |
| Rust lint | `cargo clippy --offline --locked --workspace --all-targets -- -D warnings` |
| Interface build | `npm run build:ui` |
| Interface tests with simulated IPC | `npm run test:ui` |
| Rust and JavaScript parity | `npm run verify:parity` |

Use Microsoft Edge for the UI tests, or set `$env:CM0304_UI_BROWSER = 'chrome'` to select Chrome. The regression suite checks both languages and window widths of 1280, 960 and 760 pixels. Native dialogs, actual save writes and behaviour in the game require separate verification.

The full suites and parity need local saves in `game/user data/games`. These samples are excluded from the repository. GitHub Actions runs the build, lint and sample-independent tests. Keep original saves intact and use a new destination for each write.

## Localisation

New installations default to English (UK). Preserve an explicit choice of Brazilian Portuguese. The current source strings are in Portuguese and the English translations live in `apps/desktop/ui/en.ts`. Pass user-facing strings through `t()`, supply the corresponding English entry and preserve identifiers, saved preference keys and names taken from careers. The HTML is localised at startup.

## CLI edit plans

Run the CLI from the repository root with a local save and a new destination. For example, `plan.json` can hold the following operation.

```json
[
  { "uid": 2001642, "attribute": "passing", "value": 20 }
]
```

```powershell
cargo run --offline -p cm0304-cli -- edit "source.cm4" "edited.cm4" "plan.json"
```

For structural operations such as removing recognised injury or suspension records, use `apply-plan` with a transaction object containing `edits`, `healInjuries` and `clearSuspensions`. Use `uid:<number>` for exact matches in the supported inspection commands. The editor validates the whole plan, protects existing destinations, rereads the copy and verifies the original before publishing the result. [Format and historical CLI examples (Portuguese)](DEVELOPMENT.pt-BR.md#planos-de-edição)

## Release on GitHub

1. Align the version in the Rust, npm and Tauri manifests. Run the full local suites and parity with your own saves.
2. Configure `TAURI_UPDATER_PUBLIC_KEY` as a GitHub Actions variable. Store `TAURI_SIGNING_PRIVATE_KEY` and `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` in Actions Secrets and keep a protected recovery copy.
3. Push the stable `v<version>` tag. The release workflow builds and signs the Windows x64 NSIS installer for the updater, packages the ZIP and SHA-256 checksum, and produces `latest.json`. A separate Windows runner installs, launches and uninstalls the application before the workflow creates a draft.
4. Review the draft and publish it as the latest stable release. Confirm the public ZIP checksum and updater manifest at `releases/latest/download/latest.json`. The updater signature is managed independently of Windows Authenticode.

Run `node tools/check-windows-updater.js` on Windows to verify a two-version update with WebView2, NSIS, a temporary signing key and a local copy of `save_inicial.cm4`. The test checks deferral while a save is open, automatic restart into the newer version, and preservation of the save and theme. The first public release passed this test with a 1.1.1 test build.

## Research and previous validations

The historical records document observed results and their limits. [Development history (Portuguese)](DEVELOPMENT.pt-BR.md) · [Save format (Portuguese)](../research/FORMAT.md) · [Validation reports (Portuguese)](../reports/)
