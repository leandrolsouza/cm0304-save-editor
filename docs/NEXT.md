# Continuidade do desenvolvimento

Objetivo ativo: desenvolver os marcos do ROADMAP até a v1.0, sem reduzir o escopo aprovado.

## Estado

- v0.2 e v0.3 validadas: `reports/v0.2-v0.3-validation.md`.
- v0.4 em implementação. Evidências e pendências: `reports/v0.4-validation.md`.
- Código novo ainda não commitado: modelo `Field`, transações em `extended.rs`, testes `extended.rs`, interface ampliada e backend Tauri.
- Núcleo/CLI ainda versão 0.2.0, aplicativo ainda 0.3.0-alpha.1; não anunciar v0.4 concluída.
- Build desktop atualizado e aprovado; executável fechado no momento deste checkpoint.
- Jogo aberto com `rust_v04.cm4` no perfil/informações de Clodoaldo. Copia carregou, país Portugal, 23 anos e passe 20 conferidos. Não houve avanço nem gravação no jogo.
- Hash original intacto; hashes da cópia e plano em `reports/v0.4-validation.md`.

## Próximos passos

Concluir pendências v0.4 do relatório antes de marcar o marco. Não confundir testes de releitura com conferência dos efeitos no jogo. Priorizar fluxo completo da interface com CA/posição e campos novos; conferir os efeitos indiretos projetados e alvos explícitos iguais ao valor original após mudança de CA.

A altura é armazenada em polegadas inteiras; modelo público usa centímetros arredondados, com inversa exata. Peso é kg; zero é não especificado. Referência: `game/research/reference/pregame/CM4PregameEditor/framePeopleUnit.dfm`.

Os testes Rust passaram (6 históricos + 3 novos; os 3 novos reexecutados após correção de altura). Clippy de todos os alvos core/CLI/desktop e build desktop passaram. Paridade deve ser reexecutada após finalização.

Ainda faltam v0.4 a v1.0, quatro saves de duas carreiras, testes com outra pessoa e máquina limpa. A meta permanece ativa.
