# CM0304 Save Editor — MVP

Editor local para CM0304 4.1.5, com núcleo **Rust** e interface **Tauri + TypeScript**. Os marcos v0.2 e v0.3 já permitem abrir um save, buscar jogadores, preparar várias alterações, revisar, desfazer/refazer e salvar uma cópia validada. Os arquivos do jogo e os saves ficam em `game/`.

O [roadmap até a v1.0](ROADMAP.md) amplia o produto para jogadores, staff, contratos, clubes e edição em lote. O marco **0.4.0** acrescenta CA/PA, posições, personalidade, dados pessoais, abas, idade e busca sem acentos. O trabalho rumo à v1.0 continua ativo.

A [validação da v0.4](reports/v0.4-validation.md) registra o fluxo Tauri e uma cópia CLI que avançou sete dias, foi regravada e carregada novamente no jogo. Os 12 alvos editados persistiram; outros atributos sofreram recálculos durante a evolução.

A prévia **0.5.0-alpha.1** acrescenta estado, valores e contratos existentes. Lesões e suspensões participam do mesmo plano de revisão e desfazer/refazer dos campos. A transação mista de lesão/moral foi conferida no jogo; os primeiros casos de remoção de lesão e suspensão passaram avanço de sete dias, regravação e recarga. Consulte [transações da v0.5](reports/v0.5-transactions.md) e [suspensões](reports/v0.5-suspensions.md) para evidências e pendências. A matriz completa de variantes ainda está em desenvolvimento.

## Aplicativo desktop

A consulta de staff foi integrada ao código do Tauri, com busca, paginação, funções e navegação jogador/staff. O frontend passou testes com respostas controladas; execução nativa com save real e edição de staff continuam pendentes. Veja [validação da consulta de staff](reports/v0.6-staff-ui.md).

O núcleo também oferece conversão de atributos profissionais e escrita experimental de 11 atributos de staff pelo plano CLI. Os testes preservam os dados de jogador em pessoas com dupla função. A edição pela interface e o ciclo da cópia no jogo ainda estão pendentes; detalhes em [escrita de staff](reports/v0.6-staff-editing.md).

```powershell
npm install
npm run build:desktop
./target/debug/cm0304-desktop.exe
```

Consulte [desenvolvimento](docs/DEVELOPMENT.md) para pré-requisitos e comandos Rust. O executável de desenvolvimento não exige Node.js para executar. A distribuição em máquina limpa ainda será validada na v0.8.

A comparação integral entre Rust e JavaScript e os testes no jogo estão registrados em [validação v0.2/v0.3](reports/v0.2-v0.3-validation.md).

## Ferramentas JavaScript de referência

Os comandos abaixo preservam a prova de conceito para diagnóstico e comparação. Eles exigem Node.js 22+; não são usados pelo aplicativo para ler ou gravar saves.

```powershell
npm test
npm run compare
npm run analyze
node tools/game-db-cli.js players "game/user data/games/save_inicial.cm4" "Fortaleza"
node tools/validate-ui.js
```

`analyze` exporta os dois saves para `reports/players-inicial.csv` e `reports/players-modificado.csv`. Colunas sem sufixo são os atributos convertidos pelo modelo CM Scout 3.10; colunas `_raw` preservam os bytes com sinal. No JSON, esses grupos são `attributes` e `rawAttributes`. Zeros de posições ausentes e de campos diretos são preservados.

A conversão usa a CA e distingue goleiros de jogadores de linha. Não usa PA, valores observados de outros jogadores nem médias para substituir dados ausentes.

## Conferência no jogo

No save inicial, o treinador está no Braga. Foram conferidos visualmente Clodoaldo, Erandir e Richarlyson, do Fortaleza. Os perfis têm atributos ocultos; os valores visíveis incluem divergências e não permitem confirmar igualdade com todos os atributos reais. Como controle, 61 atributos visíveis de Wender e Quim, do próprio Braga, coincidiram com a conversão. Essas observações estão nos testes de regressão.

- `reports/attribute-validation.md`: comparação campo a campo, incluindo divergências.
- `reports/fortaleza-attributes.json`: nove jogadores do Fortaleza, com valores brutos e convertidos.
- `research/FORMAT.md`: estrutura identificada e referência consultada.

## Edição em cópia

```powershell
node tools/edit-save-cli.js "game/user data/games/save_inicial.cm4" "game/user data/games/minha_copia.cm4" 2001642 flair 20
```

A edição suporta os atributos diretos listados em `EDITABLE_ATTRIBUTES` e os relativos listados em `RELATIVE_ATTRIBUTES`, sem redimensionar registros. Recusa sobrescrever arquivos existentes, editar o original, valores inválidos e dados de atributos compartilhados entre jogadores. A cópia é relida e recebe um arquivo `.edit.json` com UID, campo, valores convertidos e brutos, offset e hashes.

Para editar um atributo técnico na escala 1–20:

```powershell
node tools/edit-save-cli.js "game/user data/games/save_inicial.cm4" "game/user data/games/minha_copia_passe.cm4" 2001642 passing 20
```

A conversão inversa procura, entre os 256 bytes com sinal, um valor que resulte exatamente no atributo solicitado usando a CA e o grupo de posição atuais. Entre codificações equivalentes, escolhe a mais próxima do byte original. Valores não representáveis são recusados; a CA não é ajustada automaticamente. O jogo pode recalcular atributos ao avançar a carreira; o teste de carregamento não verifica essa evolução.

A cópia `clodoaldo_passe20.cm4` carregou no CM0304 4.1.5 e exibiu Passe 20 no perfil de Clodoaldo. Foi alterado apenas um byte, preservando a CA em 142. Veja `reports/technical-edit-validation.md`. Use nomes de arquivo de até 26 caracteres antes de `.cm4`; o jogo truncou um nome maior e não conseguiu carregar aquela tentativa.

A cópia `save_teste_clodoaldo.cm4` foi carregada no CM0304 4.1.5 e o perfil do jogador exibiu Instinto 20, após editar `flair` de 18 para 20. Veja `reports/save-edit-validation.md`. O teste não avançou a data nem regravou a partida no jogo.

O leitor percorre 23 tabelas do banco interno de versão 147 e resolve 22.062 jogadores completos por amostra. Parte final de `game_db.dat` permanece sem interpretação. A escrita preserva todos os bytes exceto o atributo selecionado.
