# CM0304 Save Editor

Aplicativo local de consulta e edição de saves do **CM0304 4.1.5** para Windows x64, com núcleo **Rust** e interface **Tauri + TypeScript**. Na versão **1.1.0**, você pode pesquisar jogadores, staff e clubes, salvar buscas, manter listas de observação, comparar perfis e exportar resultados. Nas mesmas fichas, pode preparar edições, aplicar operações em lote, revisar as mudanças e gravar uma cópia validada da carreira. [Escopo da v1.1](ROADMAP.md#v11--melhorias-no-fluxo-de-edição) · [Roadmap de consulta](ROADMAP.md#consulta-e-observação)

## Download para Windows x64

Baixe o ZIP na [release mais recente](https://github.com/leandrolsouza/cm0304-save-editor/releases/latest), extraia o conteúdo e execute o instalador. O editor usa o Microsoft Edge WebView2 e abre saves `.cm4` sem compactação do **CM0304 4.1.5**. As edições são gravadas em uma nova cópia da carreira.

O aplicativo verifica atualizações ao abrir e instala versões assinadas quando não há uma carreira aberta nem uma operação em andamento. O instalador inicial pode exibir um aviso do SmartScreen por estar sem assinatura Authenticode. O SHA-256 do ZIP está nos arquivos da release.

Projeto independente, sob a [licença MIT](LICENSE). Para relatar um problema, abra uma [issue](https://github.com/leandrolsouza/cm0304-save-editor/issues) com a versão do editor, a versão do jogo e os passos para reproduzir.

## Notas de versão

- **1.1.0** preserva a aba e a rolagem ao adicionar à revisão, sinaliza os campos no rascunho e na revisão e permite buscar uma alteração e voltar ao controle correspondente. Na prévia do lote, você pode retirar jogadores individualmente. Após gravar, a confirmação mostra o nome, o caminho e os atalhos para a pasta de destino e o histórico.

- **1.0.0** reúne a edição validada de jogadores, contratos, comissão técnica e clubes, com lotes, revisão, planos de sessão e gravação em cópia para o CM0304 4.1.5.

- **Beta.17** mostra no rodapé a versão do executável e inclui o roteiro de teste com retomada de planos e medição dos tempos do editor.

- **Beta.16** reduz o tempo de validação de lotes grandes. No teste nativo, o editor retomou um plano de 203.108 alterações do beta.15 e gravou uma cópia que passou por avanço, regravação e recarga no CM0304. [Medições](reports/v0.8-performance.md)
- **Beta.15** exporta o plano de sessão em JSON compacto quando a indentação ultrapassaria 16 MB.
- **Beta.14** aplica o limite de 16 MB na exportação e na importação. Se o plano exceder o limite, a revisão continua disponível para gravar a cópia ou reduzir o conjunto de alterações.

## Usar o aplicativo no Windows

Arraste o título ou o espaço livre no cabeçalho para mover a janela. Clique duas vezes nessa área para maximizar ou restaurar. Os botões de minimizar, maximizar e fechar ficam no próprio cabeçalho e acompanham o tema selecionado.

Use **Idioma** no cabeçalho para alternar entre **Português** e **English**. A preferência fica salva neste computador. A troca retorna à tela inicial do save ativo e preserva a revisão e o histórico de desfazer/refazer. Adicione os rascunhos à revisão antes de trocar o idioma. Os diálogos do sistema seguem o idioma do Windows e os nomes da carreira são exibidos conforme registrados no save.

Use **Tema** no cabeçalho para alternar entre **Verde original**, **CM0304 retrô** e **Claro**. A mudança é imediata e a preferência fica salva neste computador. O tema retrô usa painéis azul-violeta e seleções amarelas inspirados no jogo.

A ficha de Staff separa atributos profissionais, CA/PA e reputação, personalidade e vínculos em abas. As edições dessas abas entram juntas na revisão. A ficha de Clubes separa finanças, infraestrutura, estádio e elenco, com acesso à revisão no rodapé. Em Elenco, alterne entre Jogadores e Staff, filtre por nome e use **Vencimentos** ou o menu **Ferramentas** para as operações do clube.

As notificações aparecem como toasts no canto superior direito, logo abaixo do cabeçalho, inclusive dentro dos diálogos. Informações e confirmações de sucesso somem após alguns segundos. Avisos e erros ficam visíveis até você dispensá-los ou até chegar outra notificação. Na ficha do jogador, use **Ir ao campo** para localizar a primeira edição inválida antes de enviar o rascunho à revisão.

Em janelas compactas, a lista dá lugar à ficha quando você seleciona um resultado. Use **Voltar à lista** para retomar a busca. Na edição em lote, a prévia indica quais jogadores têm campos indisponíveis.

1. Abra o editor, clique em **Abrir save** e selecione um arquivo `.cm4` do CM0304 4.1.5.
2. Se o save tiver um clube controlado por um técnico humano, o editor abre em **Meu clube**. Use os atalhos ou procure um perfil em **Jogadores**, **Staff** ou **Clubes**. Nas fichas, ajuste os campos disponíveis e use **Adicionar à revisão**.
3. Em **Alterações**, confira os valores originais, os novos e os efeitos derivados. Use **Buscar alterações** para localizar um nome, campo ou vínculo e **Ajustar** para voltar ao controle da ficha. A busca fica ativa quando você volta à revisão. **Desfazer** e **Refazer** indicam qual campo ou área será afetado. Se a prévia falhar, use **Validar novamente** para poder gravar a cópia.
4. Clique em **Salvar cópia validada** e escolha um nome novo. Use até 26 caracteres antes de `.cm4`, limite observado no jogo. Use **Abrir pasta de destino** na confirmação para localizar o arquivo e carregue a cópia no CM0304. Para voltar ao estado anterior, carregue o save de origem no jogo.

**Variantes A/B.** Na revisão, expanda **Salvar variante na pasta do save** e dê um nome a cada cenário. O plano permanece aberto para preparar o cenário seguinte. No histórico, marque **Apenas cópias do save ativo** para localizar as variantes que vieram da mesma origem.

**Meu clube.** Cada atalho prepara uma operação na revisão com um clique. Em **Finanças**, é possível somar £100 milhões ao saldo, elevar a verba restante para transferências a £200 milhões quando ela estiver abaixo desse valor e acrescentar £25 mil ao orçamento salarial semanal. Os acréscimos usam o valor atual da revisão, quando houver. O orçamento de transferências da temporada é preservado. Na **Equipe principal**, os atalhos removem lesões ou suspensões, definem a condição em 100% ou elevam a moral para Excelente. Cada operação pode ser desfeita de uma vez. A tela indica quando um campo está indisponível e mostra quantos jogadores podem receber cada ajuste. Se houver mais de um clube controlado por um técnico humano, escolha o clube na tela. Sem um clube identificado, o editor abre em **Jogadores**.

Em **Alterações**, abra o **Histórico de cópias gravadas** para ver relatórios de sessões anteriores. **Abrir cópia** e **Abrir origem** usam os caminhos registrados na gravação. Os hashes exibidos correspondem aos arquivos no momento em que a cópia foi criada. Na alpha.16, o histórico também abre antes de carregar um save e oferece **Exportar relatório JSON** para cada relatório arquivado.

**Busca por UID.** Para localizar um jogador pelo identificador exato, use `uid:` seguido do número, como `uid:1855` para Quim. A busca por nome ou clube continua disponível no mesmo campo.

**Filtros de jogadores.** Posição, nacionalidade e idade ficam no grupo **Perfil**. Expanda **Capacidade e atributos**, **Pé e vínculo** ou **Buscas salvas** conforme precisar. **Ordenar resultados** controla a ordem da lista.

**Comparação de jogadores.** A comparação abre em **Visão geral**, com abas para **Posições**, **Atributos** e **Contratos**. A diferença é o valor da direita menos o da esquerda. **Somente diferenças** simplifica a leitura na tela. Use **Exportar** para baixar a comparação completa em CSV ou JSON.

**Listas de observação.** Em **Jogadores**, crie listas por nome e use o filtro de observação para mostrar os jogadores da lista ativa. A lista padrão é **Observação**.

**Troca de camisas.** Na ficha do clube, abra **Trocar números de camisa**, selecione dois jogadores e adicione a troca à revisão. O editor identifica o vínculo de empréstimo na seleção e valida os dois números juntos.

**Retomar uma edição.** Em **Alterações**, use **Guardar plano** para exportar as mudanças pendentes em JSON de até 16 MB. Para continuar em outra sessão, abra o save de origem e use **Retomar plano**. O editor valida o SHA-256 registrado no plano e confere se o arquivo no disco ainda corresponde ao save aberto.

**Edição de jogadores em lote.** Use **Alterar seleção** para escolher jogadores ou recuperar um grupo salvo. Escolha entre **Ajustar atributos**, **Recuperação e ocorrências** e **Copiar atributos**. Em **Ajustar atributos**, carregue um preset com **Usar preset** ou expanda **Gerenciar presets** para criar, renomear, importar ou exportar.

**Exemplo com o Braga.** Em **Clubes**, abra o Braga e escolha **Editar jogadores em lote** no menu **Ferramentas**. Selecione os jogadores, carregue **Elenco · Recuperação** e use **Prévia dos ajustes da lista** no rodapé. Confira condição, ritmo, cansaço e moral antes de adicionar o lote à revisão. Se algum campo estiver indisponível, ajuste a seleção e gere outra prévia. Para estudar a evolução de um elenco, use **Comparar elenco com outro save** e exporte o resultado em CSV ou JSON.

**Mapa do elenco.** Em **Clubes → Análise**, o mapa abre com a tática e os titulares gravados da equipe principal do técnico humano quando o save contém um registro reconhecido. Vagas que ficaram em branco no jogo aparecem como **Sem titular**. Em **Simulação**, selecione a ausência ou o fim do empréstimo de um titular para abrir a vaga e ver **Quem pode entrar**. O mapa mantém os demais titulares em suas posições e agrupa separadamente os candidatos com lesão ou suspensão registrada. O seletor de formações manuais oferece uma distribuição sugerida para comparar. **Detalhes da posição** mostra a familiaridade na posição e no lado, quando aplicável. As simulações funcionam na tela e não entram na revisão.

**Prévia de presets.** Na ficha de um jogador, use **Presets de edição** para começar com o jogador aberto. Nos resultados da busca e nas ferramentas do elenco, selecione os jogadores que quer incluir. Escolha um preset e use **Prévia do preset**. Na prévia, use **Excluir** ao lado de um jogador para removê-lo da seleção e gere outra prévia com os mesmos ajustes.

**Valores no editor e no jogo.** Salários contratuais e orçamento salarial são editados em libras por semana. O CM0304 pode apresentar valores mensais e arredondá-los. Na ficha do clube, a folha registrada vem do save, enquanto a projeção aplica os ajustes salariais do plano aos contratos. Ao carregar ou avançar a carreira, o jogo pode recalcular a folha, condição, ritmo, moral e atributos. Na recuperação do Braga, o campo gravado de condição de Quim era 98% após oito dias e o perfil no jogo exibiu 100% na recarga. [Validação de finanças](reports/v0.6-club-finance-native.md) · [Validação de recuperação](ROADMAP.md)

Em **Clubes → Finanças**, a visão geral mostra saldo registrado, verba restante para transferências e orçamento salarial semanal, seguidos da movimentação por período. Expanda **Detalhes da folha** para consultar os totais mensais ou **Editar finanças** para preparar mudanças nos campos disponíveis. Recolher o formulário mantém o rascunho aberto na sessão. Os valores enviados à revisão aparecem junto aos registrados.

Os campos editáveis de Finanças, Infraestrutura e Estádio exibem o valor registrado abaixo do controle. **Adicionar à revisão** fica junto ao formulário e é habilitado quando há uma edição válida.

Os requisitos do instalador de prévia estão nas [instruções de empacotamento](docs/DEVELOPMENT.md#instalador-windows-para-testes-da-v08).

### Recuperar uma carreira

Se o CM0304 recusar a variante ou se o resultado da edição não for o esperado, volte à tela de carregamento do jogo e selecione o arquivo de origem. Em **Alterações → Histórico de cópias gravadas**, confira os caminhos de origem e destino. **Abrir origem** reabre o arquivo no editor para conferir os valores e preparar outro plano. Use um nome novo para cada tentativa.

Se o jogo já tiver regravado a variante, o histórico ainda mostra os hashes da gravação feita pelo editor. Para preservar o teste inicial e comparar a evolução, use **Salvar como** no CM0304 antes de avançar ou regravar a carreira.

### Compatibilidade da prévia

| Área | O que determina a disponibilidade |
|---|---|
| Arquivo `.cm4` | O leitor exige o contêiner sem compactação e a versão interna 147 de `game_db.dat`, observados no CM0304 4.1.5. A versão interna é um identificador do banco. A cobertura declarada é para o patch 4.1.5. |
| Contratos | É possível editar salário, vencimento, camisa e situação no elenco conforme o vínculo. Em contratos completos com campos mapeados, a prévia também edita bônus por presença, gol e assistência e o valor de uma cláusula de rescisão existente. Os quatro valores persistiram após recarga no jogo. Os ensaios dos bônus observaram o aumento correspondente na despesa agregada do clube. A oferta de £100.000 do Santos por Angelim acionou a cláusula editada de £10.000 e foi recusada no controle. |
| Contrato proprietário e empréstimo | Cada vínculo tem seu próprio cartão. O teste de Liam Chilvers conferiu salário e datas distintos no Arsenal e no Northampton após regravar e recarregar. O teste de Igor também conferiu o retorno ao clube proprietário no vencimento do empréstimo. [Variantes contratuais](reports/v0.5-contract-variants.md) |
| Diagnóstico de contratos | Na ficha do clube, filtre vencimentos por prazo, vínculo e situação no elenco. A lista mostra a quantidade de vínculos e a soma semanal dos salários exibidos, com os valores da revisão quando houver alterações pendentes. |
| Contrato mensal | O modo mensal usa um marcador no save e o jogo apresenta o vencimento como N/D. A conversão do contrato de Clodoaldo foi conferida em dois cenários, no vencimento durante a simulação e por edição explícita no Tauri. [Matriz de carreira](reports/v0.5-matrix.md) |
| Situação de mercado | A edição exige um único contrato completo com o clube do jogador, um treinador reconhecido na equipe principal desse clube e um diário de mercado compatível. Na aba **Contratos**, o seletor mostra as transições disponíveis a partir do estado atual. Quando a edição está indisponível, o estado aparece para consulta. [Validação no jogo](reports/v0.5-matrix.md) |
| Lesões | A remoção de registros de 38, 40, 42 e 44 bytes passou por avanço, regravação e recarga no jogo. As datas e estimativas exibidas dependem dos dados disponíveis em cada registro. [Matriz de carreira](reports/v0.5-matrix.md) |
| Suspensões | A remoção foi conferida para expulsão doméstica, europeia e internacional, além de acúmulo de amarelos europeu e internacional. A edição preserva os registros de cartões. O jogo voltou a aplicar suspensões a jogadores tratados e mostrou a nova punição de Delmer após regravação e recarga. [Relatório de suspensões](reports/v0.5-suspensions.md) |
| Finanças do clube | Saldo e orçamentos são editáveis no formato financeiro completo. No formato básico, o saldo bruto fica disponível para consulta. Alguns registros não têm saldo. |
| Prestações de transferências | Nas fichas do comprador e vendedor, consulte o jogador, a contraparte, a direção do pagamento, o valor bruto e o número de prestações restantes quando o save contém a tabela reconhecida. |
| Cláusulas de revenda | Na ficha do clube comprador, consulte o jogador, o percentual e o clube beneficiário registrados no save. |
| Estádio | Lotação total e lugares sentados podem ser editados quando o estádio tem uma única equipe principal de clube vinculada. Em estádios compartilhados, os valores ficam disponíveis para consulta. A edição de lotação foi conferida na partida Braga–Marítimo e persistiu após regravação e recarga. |
| Transferência definitiva | A transferência sem taxa está em validação na CLI de desenvolvimento. A interface permite editar a situação de mercado do vínculo existente nos casos descritos acima. |

Para confirmar se uma operação passou pelo ciclo de carregamento, avanço, regravação e recarga, consulte a [matriz da v0.5](reports/v0.5-matrix.md) e a [validação de clubes da v0.6](reports/v0.6-club-finance-native.md). O [formato do contêiner](research/FORMAT.md) descreve os limites conhecidos do leitor.

## Aplicativo desktop

O código do editor é distribuído sob a [licença MIT](LICENSE).

### Campos disponíveis para edição

| Área | Campos e operações disponíveis no aplicativo |
|---|---|
| Jogadores · técnicos | Cruzamentos, drible, finalização, cabeceio, chutes de longe, marcação, passe, pênaltis, desarme, primeiro toque, técnica, bolas paradas, escanteios e lançamentos longos. |
| Jogadores · mentais | Agressividade, antecipação, coragem, criatividade, decisões, determinação, influência, instinto, posicionamento, sem bola, trabalho em equipe e índice de trabalho. |
| Jogadores · físicos | Aceleração, agilidade, equilíbrio, força, impulsão, resistência, velocidade e aptidão natural. |
| Jogadores · goleiro | Jogo de mãos, jogo aéreo, comando de área, comunicação, pontapé, lançamento, reflexos, saídas, sair de soco, um contra um e excentricidade. |
| Jogadores · complementares | Jogo sujo, consistência, jogos importantes, propensão a lesões, versatilidade e laterais. |
| Jogadores · CA, posição e dados pessoais | Habilidade atual (CA), potencial (PA), reputações nacional, atual e mundial. Goleiro, líbero, defensor, volante, meio-campista, meia ofensivo, atacante, ala, função livre, lados direito, esquerdo e central, pé direito e esquerdo. Data de nascimento, nacionalidade principal, altura em cm e peso em kg. A altura precisa ter representação exata em polegadas inteiras no save. |
| Jogadores e comissão · personalidade | Adaptação, ambição, lealdade, pressão, profissionalismo, espírito esportivo, temperamento e controvérsia. Pessoas com dupla função compartilham esses campos. |
| Estado e valores | Condição e ritmo de jogo em porcentagem, cansaço, moral, valor estimado e preço pedido. Remoção de lesões e suspensões existentes nas variantes mapeadas. |
| Contratos existentes | Salário semanal, vencimento, camisa e situação no elenco por vínculo. Em vínculos compatíveis, conversão para contrato de mês a mês, situação de mercado, bônus por presença, gol e assistência e valor da cláusula de rescisão existente. A ficha do clube permite trocar as camisas de dois jogadores quando os vínculos são elegíveis. |
| Comissão técnica | Treino de jogadores de linha e goleiros, fisioterapia, conhecimento tático, trabalho com jovens, gestão de pessoas, avaliação de capacidade e potencial, motivação, disciplina e determinação. CA, PA e reputações nacional, atual e mundial quando disponíveis no perfil. |
| Clubes | Saldo, orçamento salarial, orçamento de transferências da temporada e orçamento de transferências restante nos registros financeiros completos. Reputação da equipe principal, níveis de treino e formação de jovens, lotação total e lugares sentados nas estruturas compatíveis. |
| Lotes | Atributos, recuperação e remoções por jogador. Salário, vencimento e situação no elenco por vínculo. Atributos profissionais por membro da comissão, com presets locais. |

Ao mudar a CA ou a classificação de goleiro, escolha no formulário como tratar os atributos relativos. A revisão mostra os efeitos derivados e rejeita alvos sem representação exata. O editor apresenta apenas os campos disponíveis no registro carregado. A confirmação em jogo de cada variante está na [matriz de carreira](reports/v0.5-matrix.md), na [validação de clubes](reports/v0.6-club-finance-native.md) e no [roadmap](ROADMAP.md).

A consulta de staff foi integrada ao Tauri, com busca, paginação, funções e navegação jogador/staff. O fluxo nativo com save real passou abertura, consulta, edição de treino de campo de Rui Águas, revisão, desfazer/refazer e gravação. A cópia carregou no CM0304 com o alvo 20. Veja [validação nativa de staff](reports/v0.6-staff-native.md).

O núcleo também oferece conversão de atributos profissionais e escrita experimental de 11 atributos de staff pelo plano CLI e pela interface, com revisão e desfazer/refazer. Os testes preservam os dados de jogador em pessoas com dupla função. Dez verificações de frontend passaram com IPC controlado. O caso de treino de campo de Rui passou avanço de sete dias, regravação e recarga, mantendo o alvo 20; outras edições ainda precisam desse ciclo. Detalhes em [validação nativa](reports/v0.6-staff-native.md).

CA, PA e reputação de staff podem ser editadas no Tauri com política explícita de CA e revisão dos efeitos derivados. O formulário nativo confirmou as alterações de Rui Águas, e a cópia corrigida de Rui e Pär passou por oito dias de evolução, regravação e recarga no jogo. [Cabeçalho de staff](reports/v0.6-staff-header.md)

A conferência nativa corrigiu a conversão de jogador/staff: seus atributos profissionais usam a CA da função de jogador. O erro da cópia anterior foi corrigido e a nova cópia preservou os alvos após evolução e recarga; detalhes em [correção da dupla função](reports/v0.6-dual-staff-correction.md).

Oito campos de personalidade também têm escrita experimental no núcleo e controles no formulário de staff. Em pessoas com dupla função, esses campos são compartilhados com o perfil de jogador. Adaptabilidade e Profissionalismo 20 para Rui Águas e Pär Zetterberg persistiram após o jogo avançar, regravar e recarregar uma cópia preparada no Tauri. [Validação de personalidade](reports/v0.6-staff-personality.md)

A área Clubes permite buscar por nome ou UID e navegar entre clube, elenco e comissão. Há edições validadas no jogo para saldo, orçamentos, infraestrutura e lotação de estádio. Veja a [navegação de clubes](reports/v0.6-clubs-navigation.md) e os [testes de finanças e infraestrutura](reports/v0.6-club-finance-native.md).

```powershell
npm install
npm run build:desktop
./target/debug/cm0304-desktop.exe
```

O executável de desenvolvimento funciona sem Node.js em tempo de execução. O teste independente com uma carreira própria segue pendente. [Pré-requisitos e roteiro de desenvolvimento](docs/DEVELOPMENT.md)

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
