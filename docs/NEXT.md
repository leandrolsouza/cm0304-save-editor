# Continuidade do desenvolvimento

Objetivo ativo: desenvolver os marcos do ROADMAP até a v1.0, sem reduzir o escopo aprovado.

## Checkpoint atual

- v0.2 e fluxo funcional da v0.3 implementados e validados. Evidências em `reports/v0.2-v0.3-validation.md`.
- Núcleo e CLI Rust: versão 0.2.0. Aplicativo: 0.3.0-alpha.1 (prévia de desenvolvimento, não distribuição final).
- Executável: `target/debug/cm0304-desktop.exe`.
- O aplicativo está aberto com o save inicial e duas alterações pendentes já salvas em `tauri_v03.cm4`.
- O CM0304 está aberto com `tauri_v03.cm4`, no perfil de Clodoaldo, mostrando passe 20 e instinto 20.
- Nenhuma das validações deste checkpoint avançou a data ou regravou o save no jogo.
- Antes de recompilar o executável desktop no Windows, fechar a instância pela interface para liberar o arquivo. As alterações pendentes são somente as geradas pelo próprio teste.

## Próxima implementação — v0.4

Preservar o checkpoint antes de ampliar o formato de Edit (atualmente valor i8) para CA/PA, dados pessoais e reputação. A alteração deve comportar campos de 1, 2 e 4 bytes, efeitos indiretos da CA e troca do grupo de goleiro; não tratar esses campos como atributos brutos independentes.

Referência adicional já baixada e extraída, fora do Git:

`game/research/reference/pregame/CM4PregameEditor/Database/CM4DataTypes.pas`

É o código-fonte público CM4 Pregame 0.10 do SourceForge. Usar como referência de layout, validar nas amostras e no jogo; não assumir equivalência irrestrita entre banco pré-jogo e save.

No início dos dados específicos do jogador, o leitor atual confirma CA em +6, PA em +8 e índice de playingData em +10. O loader da referência adicional detalha a região antes ignorada:

| Offset relativo ao início dos dados do jogador | Campo / largura |
|---:|---|
| +0, +2, +4 | reputações / int16 cada |
| +18 | valor estimado / int32 |
| +22 | moral / byte |
| +23 | fator de escolha de nação / byte |
| +24 | fitness / int16 |
| +26 | jadedness / int16 |
| +28 | condition / int16 |
| +30 | número de clubes que o observam / byte |
| +31 | máscara de jogos da seleção / int16 |
| +33 | último jogo da seleção / int16 |
| +35 | internacionalizações ao ingressar / byte |
| +36 | declarado para seleção / byte |
| +37 | peso / int16 |
| +39 | altura / int16 |
| +41 | ability offset / byte |
| +42 | valor de venda / int32 |

Esses campos novos ainda não foram adicionados ao modelo nem validados visualmente. Antes de expor altura/peso, conferir valores conhecidos na tela de informações. Condição/moral pertencem à etapa v0.5.

O modelo atual mantém `Person` com referências de nomes/nacionalidade e data de nascimento; guardar os offsets antes de ler os campos comuns. `personData` contém oito bytes de personalidade, mas ainda precisa de offsets no modelo para escrita. Posições e pés já existem nos 65 bytes de playingData.

A paridade atual exige igualdade de todos os campos do modelo antigo. Ao adicionar campos intencionais, manter essa comparação para o subconjunto histórico e testar os campos novos separadamente; não remover o verificador nem fazer a referência aceitar resultados incorretos.

## Verificações já executadas

- `cargo test --offline`: seis testes de integração Rust aprovados, mais suites vazias dos bins/lib.
- `cargo clippy --offline -p cm0304-core -p cm0304-cli --all-targets -- -D warnings`: aprovado.
- `cargo clippy --offline -p cm0304-desktop -- -D warnings`: aprovado.
- `node --test`: 12 testes JavaScript aprovados.
- `node tools/verify-rust-parity.js`: igualdade integral em duas amostras.
- Interface nativa: abrir, pesquisar, perfil, editar dois campos, revisar, desfazer/refazer e salvar cópia.
- Jogo: carregamento de `rust_v02.cm4` e `tauri_v03.cm4`; passe/instinto 20 de Clodoaldo conferidos em ambas.

Ainda faltam os marcos v0.4 a v1.0 e suas validações. A meta não está concluída.
