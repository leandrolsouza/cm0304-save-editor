# Continuidade do desenvolvimento

Objetivo ativo: desenvolver os marcos do ROADMAP até a v1.0, sem reduzir o escopo aprovado.

## Estado

- v0.2 e v0.3 validadas; v0.4 em validação, sem marcar concluída.
- Núcleo/CLI/desktop/package versão **0.4.0-alpha.1**. Build final deste checkpoint deve ser conferido antes de encerrar.
- Checkpoint anterior: `fdc751a`; novas alterações desta rodada ainda pendentes de commit.
- `reports/v0.4-validation.md` registra implementação, testes, UI e evolução. `reports/v04-evolution.json` registra os 12 alvos preservados e 20 recálculos após sete dias.
- Editor foi fechado para compilar a versão final com `merge_plan`; na execução anterior o fluxo completo salvou `tauri_v04.cm4` com quatro bytes alterados. Depois a política foi trocada para preserveValues e conferida no perfil, sem segunda gravação.
- Jogo aberto com `rust_v047d.cm4` na data 26/7/2003, tela do elenco Braga, após encerramento e recarga. Essa cópia vem da CLI, não da Tauri.
- Origem intacta. A cópia CLI reteve os 12 alvos de Clodoaldo após evolução/regravação/recarga no jogo.

## Próximos passos

- Conferir posições/pés e variantes goleiro/jogador-staff no jogo; concluir as pendências do relatório v0.4 antes de marcar o marco.
- Buscar nomes sem acentos pela interface, além do teste unitário de normalização.
- Executar fluxo da UI após `merge_plan`, especialmente remover a mudança de CA e limpar alvos relativos sem efeito; teste Rust já cobre isso.
- Carregar `tauri_v04.cm4` no jogo; não confundir com a cópia CLI já carregada.
- Validar visualmente a distribuição final após mudanças de layout; UI anterior com abas foi examinada.
- Depois, avançar para v0.5: estado, contratos, lesões/suspensões e valores, conforme ROADMAP.

## Notas de formato

Altura: polegadas inteiras no arquivo, centímetros arredondados no modelo público, com inversa exata. Peso: kg; zero não especificado. Referência local pregame `framePeopleUnit.dfm`. Nascimento ordinal base zero; 1980/dia59 exibiu 29/2/1980 no jogo.

Para v0.5, `read_person` ainda descarta contratos. `CM4SaveGameData.pas:1967` lê o segmento comum após os dados do tipo: byte extra (exceto trial), tipo de entidade 0/1, índice i32, salário semanal i32, data de término, situação/transferstatus, felicidade8, data inicial, felicidade/percepção/clubchoice/camisa/opções/unknown2. Capturar offsets dos contratos existentes e distinguir clube/seleção e empréstimos antes de editar. O pregame contém os nomes de campos, mas seu suporte de tipos não substitui o loader do save.

Ainda faltam v0.4 a v1.0, quatro saves de duas carreiras, teste com outra pessoa e máquina limpa. Meta ativa.

Build final 0.4.0-alpha.1 aprovado. `cargo test --offline`: 13 testes aprovados; clippy all-targets core/CLI/desktop aprovado. Executável atualizado está fechado e disponível em `target/debug/cm0304-desktop.exe`. A execução nativa documentada precede apenas a correção de fusão de planos e os ajustes finais de rótulo/validação de campo vazio; repetir os casos indicados acima na próxima rodada.
