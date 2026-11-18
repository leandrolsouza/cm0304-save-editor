# Teste de edição e carregamento — Clodoaldo

- Jogo: CM0304 4.1.5.
- Origem: `game/user data/games/save_inicial.cm4`.
- Cópia: `game/user data/games/save_teste_clodoaldo.cm4`.
- Jogador: Clodoaldo, Fortaleza EC, UID 2001642.
- Campo: `flair`, exibido como **Instinto** na tradução instalada.
- Alteração: **18 → 20**, no offset absoluto **20622052**.
- Tamanho dos dois saves: **35054444 bytes**.
- Diferença binária: **exatamente um byte**.

## Verificação no jogo

A cópia foi selecionada pelo nome no menu Jogar jogo → Retomar Jogo. O carregamento terminou normalmente e abriu o plantel do Braga em 19/07/2003. Pela pesquisa rápida, foi localizado Clodoaldo no Fortaleza. O perfil abriu e exibiu **Instinto 20**.

Não foi avançada a data nem regravada a partida no jogo. O teste confirma carregamento e leitura do perfil, sem avaliar uma temporada inteira ou comportamento após evolução dos atributos.

## Integridade

SHA-256 do original, confirmado também após o teste:
`1e2280f838e8516d4da69c3b15faf01df9ec7cd085a03f56a58e0f516894d3b4`

SHA-256 da cópia:
`aa66d7d8921a46ffd22f5fb6e037c9978e115a7bd091bd7891a219a484e403f7`

O comando cria exclusivamente arquivos novos e recusa sobrescrita. O arquivo `.cm4.edit.json` ao lado da cópia registra os detalhes da alteração. Os 10 testes passaram, incluindo a comparação de todos os bytes da edição e a preservação do buffer original.
