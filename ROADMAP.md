# Roadmap do CM0304 Save Editor

Atualizado em 22/11/2026. A **v1.0.0** foi empacotada para Windows x64 e aceita com as validações locais do CM0304 4.1.5. A proposta para o próximo ciclo está em [Depois da v1.0](#depois-da-v10). As demais seções registram o escopo e as evidências da v1.0.

## Objetivo

Centralizar **consulta da carreira e edição de saves do CM0304 4.1.5** em um aplicativo local para Windows, em português. A consulta deve permitir pesquisar, comparar e acompanhar jogadores, comissão técnica e clubes no mesmo save. A edição usa essas mesmas fichas e seleções, com revisão e gravação em cópia validada. O aplicativo funciona sem terminal, Node.js ou Rust instalado.

## Escopo de produto para a v1.0

A stack aprovada é **Rust + Tauri + TypeScript**. O produto deve permitir administrar um elenco, sua comissão e o clube, além de editar atributos. O resumo abaixo orienta a entrega; as seções de progresso registram separadamente o que já foi demonstrado.

| Prioridade | O que o usuário poderá fazer | Marco |
|---|---|---|
| Essencial | Editar atributos técnicos, mentais, físicos e de goleiro, CA/PA, posições, pés, personalidade, reputação e dados pessoais mapeados. | 0.4 |
| Essencial | Ajustar condição, ritmo, cansaço e moral, recuperar lesões e suspensões suportadas e administrar vínculos existentes por clube, com salário, vencimento, camisa, situação no elenco, valor e preço pedido. | 0.5 |
| Essencial | Editar atributos profissionais e reputação da comissão técnica, além de saldo, orçamentos, reputação e instalações do clube nos formatos validados. Consultar folha salarial e composição do elenco. | 0.6 |
| Essencial | Selecionar jogadores por clube e filtros, ajustar ou copiar atributos em lote, aplicar presets, revisar alterações de jogadores, staff e clubes juntas, desfazer e refazer alterações pendentes e exportar comparações. | 0.7 |
| Essencial | Retomar planos locais, instalar no Windows e usar todos os fluxos sem terminal. | 0.8 |
| Lançamento | Concluir testes de regressão, documentação, matriz de saves e distribuição versionada. | 0.9–1.0 |

### Próximas ampliações de edição

| Área | Operação proposta | Critério para liberar no Tauri |
|---|---|---|
| Transferência definitiva | Mover um jogador sem empréstimo ativo entre dois clubes, sem taxa, com prévia da mudança de elenco, do contrato e da folha dos dois clubes. | Confirmar elenco de origem e destino, elegibilidade em partida, persistência após regravação e consistência das referências de inscrição. O protótipo da v0.8 já fornece casos de partida para a validação. |
| Contratos | Editar bônus por presença, gol e assistência e o valor da cláusula de rescisão nas variantes mapeadas. Os quatro campos já foram conferidos na ficha após avanço e recarga. | Isolar o efeito de cada pagamento e testar uma proposta que acione a cláusula no jogo. |
| Gestão de elenco | Aplicar em lote situação no elenco e recuperação de lesões e suspensões suportadas, com seleção individual e prévia por vínculo ou ocorrência. | Remover um jogador da prévia, desfazer o lote inteiro e conferir no jogo os jogadores editados e excluídos. |
| Comissão técnica | Aplicar um preset de atributos profissionais a membros selecionados por função, com prévia de efeitos de CA e proteção de registros de dupla função. | Conferir a aplicação em dois membros, preservar os atributos de jogador de um membro com dupla função e reabrir a cópia no jogo. |
| Situação de mercado | Alterar disponibilidade para transferência ou empréstimo, quando os indicadores e registros relacionados estiverem mapeados. Mostrar o estado atual e o proposto na revisão. | Confirmar os estados na ficha e na lista do clube, preservar propostas existentes e repetir a conferência após avanço e recarga. |
| Vínculos e inscrições | Editar números de camisa por clube e inscrição quando houver registros distintos. Sinalizar conflitos de numeração na prévia. | Trocar números entre dois jogadores, confirmar a escalação no jogo e conferir a persistência após regravação. |
| Infraestrutura | Editar capacidade de estádio nas estruturas identificadas e indicar quando a alteração afeta mais de um clube que usa o mesmo estádio. | Conferir capacidade e ocupação em partida, verificar clubes que compartilham o estádio e recarregar a cópia. |

Essas operações são candidatas à inclusão antes da v0.9. A decisão de lançamento de cada operação depende da validação acima. Transferências com taxa, criação de contratos e empréstimos, rescisões e movimentação de staff exigem investigação própria e ficam no backlog pós-v1.0.

Para tornar a gestão do clube útil, o painel deve reunir **saldo, orçamento de transferências, orçamento salarial, folha atual, instalações de treino, formação de jovens e reputação**, distinguindo dados editáveis de resumos calculados. Campos financeiros e de infraestrutura só recebem edição após identificação e conferência no jogo; uma pendência nessa área mantém o marco aberto.

As ferramentas de lote devem permitir recuperar todo o elenco, alterar vencimentos de contratos existentes, ajustar salários, aplicar incrementos com limites e copiar atributos selecionados. Cada operação apresenta os destinos e a prévia individual, admite exclusões e pode ser desfeita inteira.

O aceite da v1.0 exige completar pela interface os fluxos de jogador, carreira, comissão, clube e lote. A demonstração de lançamento deve incluir recuperar vários jogadores, ajustar salário e vencimento por vínculo, editar um membro da comissão e o orçamento de um clube, revisar as mudanças juntas e confirmar a cópia no jogo.

A suíte Playwright com IPC simulado confirmou que a revisão mantém juntos salário e vencimento do empréstimo selecionado, atributo profissional da comissão e orçamento de transferências do clube. Os 29 cenários de interface passaram. O ensaio com Tauri nativo e IPC real executou esse fluxo no Braga. A recuperação em lote levou condição e ritmo de Quim e Fredrik a 100% e moral a 18. Dois lotes editaram o contrato de Quim para £1.870 por semana e vencimento em 30/06/2005. A mesma revisão mostrou Adaptabilidade 20 para Rui Águas e os orçamentos de transferências de £900.000 e £700.000. A interface salvou `0aceitetauri.cm4` (SHA-256 `420cbb41c5cf558a50901b54ce9f64d218af1a25a75469c870b982d46c6950ea`), e a releitura Rust confirmou os valores e o hash da origem. O CM0304 4.1.5 carregou a cópia em 19/07/2003 às 09:00. O perfil de Quim mostrou condição 100%, moral Muito Bom, remuneração arredondada de £7.500 por mês e vencimento em 30/06/2005. O jogo avançou até 20/07/2003 às 14:00, gravou `t.cm4` (SHA-256 `526d6c1d936b389c9bc9c77007cc9e8a869a238a4a55df666dbf8d8b9f9e765a`) e abriu a cópia após reiniciar. O perfil de Quim voltou a mostrar condição 100%, moral Muito Bom e o mesmo salário mensal e vencimento. O leitor Rust confirmou condição e ritmo em 100% e moral 18 para Quim e Fredrik, £1.870 semanais e vencimento em 30/06/2005 para Quim, Adaptabilidade 20 para Rui e ambos os orçamentos. A origem e a variante do Tauri conservaram seus hashes.

Uma variante conjunta gerada pela CLI Rust a partir do save inicial aplicou recuperação a Quim e Peter Hoekstra, removeu a lesão de Hoekstra, ajustou o empréstimo de Liam Chilvers para £571/semana até 29/02/2004, editou Adaptabilidade de Rui Águas para 20 e definiu os orçamentos de transferência do Braga em £900.000 e £700.000. A variante `0aceite.cm4` (SHA-256 `88af411b8f4921beefe28d1c5432ea04adc0606523fe2744e7190501ca3b61eb`) carregou no CM0304 4.1.5 em 19/07/2003 às 09:00. O perfil de Quim mostrou condição 100% e moral Muito Bom. Após o avanço, o jogo gravou `a.cm4` (SHA-256 `7c2d7630addf0d665d60e26d06a5969136431eaa4bd950b1c2fa16f275e5d441`) e retomou a carreira após reiniciar, em 20/07/2003 às 14:00. A releitura confirmou condição e ritmo em 100% nos dois jogadores, ausência da lesão de Hoekstra, contrato do Arsenal preservado e os alvos de empréstimo, comissão e orçamento. A moral de Hoekstra passou de 18 para 17 durante a simulação. O SHA-256 do save original permaneceu igual.

## Arquitetura

- **Rust:** leitura binária, modelos, índices de busca, conversão de atributos, regras de edição, transações, validação e gravação.
- **Tauri:** aplicativo desktop, seleção de arquivos, integração com Windows e distribuição.
- **Frontend TypeScript:** telas, formulários, filtros e revisão; biblioteca visual a definir na etapa da interface.
- O frontend chama operações do núcleo Rust; não calcula offsets nem grava bytes.
- O núcleo mantém o save e os índices em memória; a interface recebe páginas e perfis, sem transferir o banco inteiro a cada pesquisa.
- Operações demoradas fora do fluxo da interface, com progresso e cancelamento quando aplicável.
- Funcionamento local, sem backend remoto ou upload de saves.
- JavaScript atual como referência de migração e comparação. Nenhuma dependência do leitor/escritor JavaScript no aplicativo final.

## Estado atual — v0.1.0

Prova de conceito em JavaScript, validada em duas amostras:

- [x] Leitura do contêiner e comparação de blocos.
- [x] Leitura de 23 tabelas e identificação de 22.062 jogadores completos por amostra.
- [x] Busca por nome, clube e UID; exportação CSV/JSON.
- [x] Consulta de CA/PA, posições, atributos e personalidade.
- [x] Conversão e inversa dos atributos relativos.
- [x] Edição de um atributo por operação em nova cópia.
- [x] Cópias carregadas no jogo, com Instinto 20 e Passe 20 de Clodoaldo confirmados.
- [x] 12 testes automatizados aprovados.

Essa é a base histórica em JavaScript. A migração e a primeira interface já foram implementadas nos marcos abaixo. A parte final do banco permanece parcialmente desconhecida.

## Progresso de implementação

A prévia `0.5.0-alpha.3` inclui o histórico de variantes por origem e a gravação na pasta do save. O pacote NSIS x64 passou instalação, abertura dos dois saves de amostra e desinstalação no Windows de desenvolvimento. [Validação do instalador](docs/DEVELOPMENT.md#instalador-windows-para-testes-da-v08).

Um build Tauri de desenvolvimento gravou os dois lotes em `lotesmix.cm4` pelo fluxo de variantes. O CM0304 4.1.5 carregou a cópia em 19/07/2003 às 9h. Hoekstra exibiu condição 100%, Lesões Nada e físico Totalmente apto. Gyan exibiu Suspensões Nada. Em 26/07 às 10h, o jogo gravou `lotesmix7d.cm4`. O CLI confirmou zero lesões para Hoekstra, zero suspensões e um registro de cartões para Gyan, e situação Indispensável para Leandro no empréstimo ao Fortaleza. O contrato do Cruzeiro conservou a situação original. SHA-256 da regravação: `BEA731E70501BD762C59F0B30E9CDA4802ADE1D7B22F65F1D4C44E8A4D5CF885`. A origem e a cópia editada conservaram seus hashes.

A alpha.2 instalada pelo NSIS passou um ensaio de IPC real no WebView2 com `save_inicial.cm4`. A prévia combinou a recuperação física com a remoção da lesão de Peter Hoekstra e da suspensão de Baffour Gyan. Um segundo lote mudou a situação de Leandro no empréstimo ao Fortaleza de Importante para Indispensável, preservando a situação no contrato do Cruzeiro. Ao solicitar outra prévia, o editor recusou a inclusão pela prévia anterior. A revisão exibiu os dois lotes, e dois comandos de desfazer retiraram um de cada vez. O ensaio foi até a revisão e a instalação de teste foi removida.

Após reiniciar o CM0304, **Retomar Último Jogo** identificou `lotesmix7d` e carregou o plantel do Braga em 26/07/2003 às 10h. Na aba Informação de Leandro, o jogo exibiu **Este jogador é indispensável ao clube** nos detalhes do empréstimo ao Fortaleza e **Este jogador não é necessário no clube** nos detalhes do contrato profissional com o Cruzeiro. A tela confirmou também os vencimentos de 31/12/2003 no Fortaleza e 01/01/2005 no Cruzeiro, em acordo com a releitura Rust.

A prévia `0.5.0-alpha.2` reúne os lotes de jogadores, contratos e comissão, os presets, a comparação ampliada, o cancelamento de abertura e gravação e as melhorias de desempenho. O instalador NSIS x64 passou instalação, inicialização e desinstalação no Windows de desenvolvimento. Os critérios restantes dos marcos continuam em aberto. [Empacotamento e hash](docs/DEVELOPMENT.md#instalador-windows-para-testes-da-v08)

Na v0.8, o histórico de cópias filtra pela origem usando o SHA-256 do save ativo. O Playwright com IPC simulado confirmou que uma cópia de outra origem fica oculta. No Tauri nativo, **Salvar variante na pasta do save** gerou duas cópias, com Passe 18 e Passe 20 para Clodoaldo. O filtro mostrou as duas juntas, a releitura Rust confirmou os alvos e os hashes das cópias coincidiram com os gerados pelo CLI. O hash da origem foi preservado. A gravação pelo diálogo do Windows é um fluxo separado. Em um build posterior em release, o histórico abriu a variante com Passe 20 para Clodoaldo e, pelo botão Abrir origem, voltou ao save inicial com Passe 13.

Na v0.8, a gravação de uma cópia ganhou indicador de atividade e cancelamento cooperativo. A escrita verifica o cancelamento entre blocos e antes de publicar o arquivo. No WebView2 com IPC real, cancelar a gravação de uma variante manteve o hash de origem, a revisão e o conteúdo do diretório de teste. Repetir a gravação com o mesmo nome produziu uma cópia que o Tauri reabriu com Passe 20 para Clodoaldo. O ensaio usou a gravação na pasta do save. O diálogo do Windows fica fora dessa verificação.

Na v0.8, a abertura de save ganhou indicação de progresso e cancelamento cooperativo durante a leitura e o processamento do banco. O Rust descarta a leitura parcial, e a sessão ativa só é substituída após a conclusão. A suíte Playwright com IPC simulado confirmou que cancelar a abertura preserva a carreira ativa. No WebView2 com IPC real, a abertura de `save_modificado.cm4` foi acionada pelo fluxo de recentes e cancelada pelo botão da interface. O editor manteve `save_inicial.cm4` e o Passe de Clodoaldo de 13 → 20 na revisão. A validação do diálogo de arquivos do Windows fica separada desse ensaio.

O Tauri nativo, testado por meio do WebView2 com IPC real, abriu `save_inicial.cm4`, selecionou Pär Zetterberg e Jacky Munaron no Anderlecht e apresentou os alvos de 15 → 20 e 16 → 20 na prévia e na revisão. Desfazer retirou as duas alterações em uma operação. O fluxo usou um perfil isolado do WebView2. Na alpha.11, o Tauri gravou `0stafftauri.cm4` com os mesmos dois alvos. O SHA-256 `4D3422C2E5901D2F5FF839DA34B6654B1C3B96BAEFDA67358086009EAF2B0A1E` coincide com a variante da CLI carregada e avançada no CM0304. A releitura do perfil de jogador de Pär foi idêntica à da origem. O CM0304 4.1.5 abriu diretamente `0stafftauri.cm4` em 19/07/2003 às 09:00. A ficha de Jacky Munaron e a aba Perfil Staff de Pär Zetterberg exibiram Avaliação Capacidade Jogador 20. A aba de jogador de Pär também abriu.

Em uma cópia do save inicial, o CLI aplicou o mesmo alvo de avaliação da capacidade do jogador a Pär Zetterberg e Jacky Munaron, ambos do Anderlecht. A releitura confirmou 15 → 20 e 16 → 20, respectivamente. Os perfis de jogador de Zetterberg antes e depois foram idênticos na saída do CLI, e o hash do save inicial foi preservado. O CM0304 4.1.5 carregou essa cópia com o nome `0stafflote.cm4` em 19/07/2003 e exibiu Avaliação Capacidade Jogador 20 na ficha de Munaron e na aba Perfil Staff de Zetterberg. SHA-256 da cópia: `4D3422C2E5901D2F5FF839DA34B6654B1C3B96BAEFDA67358086009EAF2B0A1E`.

Na v0.7, **Comparar comissão técnica → Editar comissão em lote** permite selecionar membros por função, definir vários atributos profissionais e conferir a prévia por pessoa antes de adicionar uma operação à revisão. O núcleo valida o plano completo com as regras de CA em vigor. A suíte Playwright com IPC simulado confirmou a seleção, a prévia e a inclusão. Presets locais guardam os valores profissionais sem fixar os membros do lote. A suíte também confirmou a gravação e o carregamento de um preset. A cópia de Zetterberg e Munaron foi conferida no CM0304. O jogo avançou até 26/07/2003 às 0h e gravou `0stafflote26jul.cm4`. O CLI confirmou avaliação 20 para os dois, com Pär ainda identificado como jogador e staff. SHA-256 da regravação: `D76CF80656BC52E6D1184A36E533D6A36C16F14196E8583905A326BB3CADE10C`.

Após reiniciar o CM0304, **Retomar Último Jogo** identificou `0stafflote26jul` e carregou o plantel do Braga em 26/07/2003 às 0h.

O painel de contratos da v0.7 permite ajustar a situação no elenco em lote por jogador e vínculo, com prévia do valor anterior e do novo por contrato. O teste Rust confirmou a gravação no empréstimo selecionado e a preservação do vínculo proprietário. A suíte Playwright com IPC simulado confirmou a seleção, a prévia e a inclusão na revisão. O ensaio com Leandro gravou a situação Indispensável no empréstimo ao Fortaleza e preservou a situação no Cruzeiro. O jogo avançou, regravou e mostrou as duas situações após reiniciar. A exclusão de Clodoaldo na prévia foi verificada no Tauri com `000loteexclusao.cm4`, que passou por avanço, regravação e recarga no CM0304. Leandro ficou indispensável no Fortaleza, com o vínculo do Cruzeiro preservado, e Clodoaldo manteve a situação original.

A regressão da prévia `0.5.0-alpha.1` no Windows passou com 62 testes Rust em release, 12 testes JavaScript e a compilação TypeScript da interface. O Rust cobriu cancelamento de comparação, identificação de save inválido, alterações concorrentes na origem e no destino e a restauração de plano pelo hash do save.

- [x] **v0.2:** núcleo Rust com leitura equivalente, transações de múltiplos campos/jogadores e gravação exclusiva validada. Cópia carregada no jogo.
- [x] **v0.3:** aplicativo Tauri com abertura, busca paginada, perfil, revisão, desfazer/refazer e salvar cópia. Fluxo completo executado na interface e cópia carregada no jogo.
- [x] **v0.4:** editor ampliado de jogadores, políticas de CA, abas, idade e busca sem acentos. Fluxo Tauri, posições/pés, goleiro e jogador/staff conferidos; cópias avançaram sete dias, foram regravadas e recarregadas no jogo.
- [x] **v0.5 — Carreira, estado e contratos:** demonstração funcional concluída para os registros suportados. O Tauri gravou edições de estado, contratos e preço pedido, além de remoções de lesão e suspensão. O CM0304 confirmou avanço, regravação e recarga, incluindo vencimento, retorno de empréstimo e nova suspensão de um jogador tratado. O contrato ativo com entidade seleção e outros estados da estimativa de retorno continuam em investigação na matriz de estabilização. [Matriz da v0.5](reports/v0.5-matrix.md)
- [x] **v0.6 — Staff e gestão de clubes:** demonstração funcional concluída no Tauri e no CM0304 4.1.5 para os registros suportados. [Evidências do marco](#evidências-da-v06)
- [x] **v0.7 — Ferramentas para editar e analisar em escala:** demonstração funcional concluída no Tauri e no CM0304 4.1.5. [Evidências do marco](#evidências-da-v07)
- [x] **v0.8 — Produto utilizável no Windows:** instalação isolada e desempenho do lote verificados na versão distribuída. Validação local em duas carreiras, conforme o critério de aceite aprovado.
- [x] **v0.9 — Estabilização:** regressões, documentação e validações locais aprovadas.
- [x] **v1.0 — Distribuição para Windows x64:** pacote oficial gerado e auditado localmente.

A revisão ativa da v0.7 agrupa as entidades pelo UID do clube registrado no save. A suíte com IPC controlado confirmou que o jogador aparece dentro do clube vinculado. Relatórios arquivados continuam disponíveis por entidade. No Tauri nativo, a revisão do lote de Passe mostrou Clodoaldo sob Fortaleza e Quim sob Braga. Após retirar Quim pela revisão, a cópia foi relida com Passe 20 para Clodoaldo e Passe 4 para Quim. O hash da origem permaneceu igual.

Para testar os dois alvos juntos, a CLI Rust gravou Passe 20 para Quim (UID 1855) e Clodoaldo (UID 2001642), alterando dois bytes no total. O CM0304 4.1.5 carregou uma cópia idêntica da variante como `0p07.cm4` em 19/07/2003 às 09:00 e gravou `p.cm4` em 20/07/2003 às 14:00, após o evento de numeração do Braga. Após reiniciar, o jogo recarregou `p.cm4` na data avançada. A releitura Rust confirmou Passe 20 para os dois UIDs na regravação, SHA-256 `29451531f9edee4493db86c6f8a1a61063b0febbd2e618d11a1a636bda761842`. O perfil nativo de goleiro de Quim exibe os atributos da posição. O Passe dos dois foi conferido pelo leitor. [Relatório da gravação do lote](reports/v07-lote-passe-write.json)

A recuperação em lote da v0.7 combina condição e ritmo a 100%, teto de cansaço em 0 e piso de moral em 18. A prévia de múltiplos ajustes mostra o campo de cada linha. A validação pelo CLI na carreira B confirmou a redução do cansaço de Henriksen de 21 para 0 e o aumento da moral de 14 para 18. Para Jermouni, foram preservados o cansaço -28 e a moral 20. A origem conservou seu hash e a compilação da interface passou. O ensaio nativo e a conferência da cópia no CM0304 continuam pendentes.

A revisão da v0.7 lista as operações da sessão por área e permite retirar uma operação anterior. O Rust reaplica os alvos explícitos posteriores, valida o plano resultante e registra a retirada como uma ação reversível. O teste de regressão confirmou que alvos posteriores, inclusive para um campo sobrescrito, são preservados. O histórico de staff reaplica CA, PA e reputação separadamente e conserva a política ao reaplicar uma nova CA. O histórico também conserva separadamente os três valores de reputação e os dois de orçamento de transferências, usando os valores do save como referência. Um teste confirmou a retirada dos componentes anteriores e a preservação dos posteriores. No Tauri nativo, retirar a operação de vencimento de Leandro preservou o ajuste posterior nas instalações de treino do Fortaleza. Desfazer restaurou o vencimento. Refazer e gravar produziram uma cópia que o leitor Rust confirmou com treino 12 e o vencimento original de 31/12/2003.

A regressão em release passou com 44 testes Rust do workspace e um novo teste de lote contratual executado em separado. Esse teste grava salário e vencimento no empréstimo de Leandro, preserva o contrato do Cruzeiro e verifica as diferenças no comparador de elencos. Também passaram os 12 testes JavaScript e a [paridade dos dois saves](reports/rust-parity.json), com 22.062 jogadores por amostra e todos os campos históricos e de contêiner comparados.

Na v0.6, a ficha do clube compara membros da comissão por 11 atributos profissionais do save e filtra a lista pela função. A suíte com IPC controlado confirmou a ordem por atributo, o filtro de treinador adjunto e a abertura da ficha selecionada. O beta.13 confirmou essas ações no Tauri nativo com Rui Águas e José Manuel no Braga. [Evidência](docs/DEVELOPMENT.md#evidências-locais-do-beta)

Na central de contratos da v0.5, o clube pode alternar entre próximos 90 dias, vencidos, mensais e todos os vínculos mapeados, filtrar por tipo de vínculo e situação no elenco e ordenar por salário semanal. A situação e o salário refletem os valores propostos na revisão. O teste Rust confirmou as fronteiras de vencimento e o marcador mensal. A suíte com IPC controlado confirmou ordenação salarial, descoberta de contrato vencido, seleção por situação e navegação até o empréstimo selecionado. A conferência nativa da central ampliada segue pendente.

Na v0.8, o Tauri nativo guardou um plano de vencimento com o SHA-256 da origem. Após reiniciar, recusou o JSON no save inicial e manteve a revisão vazia. Na cópia salarial usada como origem, o mesmo JSON restaurou a mudança de 31/12/2003 para 29/02/2004 e gerou uma segunda cópia com SHA-256 idêntico ao da primeira, mantendo a origem intacta.

No Tauri nativo em Windows, um lote salarial do Fortaleza aplicou +20% com teto semanal de £1.600. A prévia e a revisão mostraram Clodoaldo de £1.490 para £1.600 e o empréstimo de Leandro de £273 para £328. O CLI releu os dois valores da cópia, o hash da origem permaneceu igual e Ctrl+Z/Ctrl+Y retiraram e restauraram o lote inteiro na revisão. O CM0304 carregou a cópia em 19/07/2003 e exibiu £6.500/mês para Clodoaldo e £1.300/mês para o empréstimo de Leandro. O contrato proprietário de Leandro no Cruzeiro ficou em £273/semana no save e £1.100/mês na tela. Após sete dias, o jogo gravou `v07salarios7d.cm4` em 26/07/2003. A releitura Rust confirmou £1.600, £328 e £273 por semana, respectivamente. Após reiniciar e retomar o arquivo regravado, o CM0304 exibiu novamente os três valores mensais. SHA-256 da regravação: `112D5D1A2939C188A0689C2D41673DAA8D92450F6058B4AC79FCB09C63AA4D40`.

O núcleo Rust, o CLI e a interface da v0.7 aceitam vários ajustes numéricos de jogador no mesmo lote. A cópia de teste foi relida com Passe 20, Técnica 18 e Moral 20 para Clodoaldo e Quim. O CLI recusou um campo repetido, e o hash da origem permaneceu igual. A suíte com IPC controlado confirmou o envio de dois ajustes e a invalidação da prévia ao remover um deles. Grupos locais guardam os UIDs selecionados manualmente ou a partir de uma busca filtrada e selecionam os membros disponíveis na lista atual. A suíte também confirmou a restauração de um grupo de 50 jogadores com a exclusão preservada. O beta.13 gravou e releu um lote nativo com Passe, Técnica e Moral para dois jogadores do Braga. O CM0304 confirmou o ciclo de sete dias com regravação e recarga. Passe e Técnica persistiram e a moral foi recalculada de 18 para 19. [Evidência](docs/DEVELOPMENT.md#evidências-locais-do-beta)

O CLI e a tela de clubes da v0.7 também projetam vencimentos em lote por UID e índice do vínculo. Na cópia da amostra inicial, os contratos escolhidos de Clodoaldo e do empréstimo de Liam foram relidos com vencimento em 29/02/2004. O teste recusou um empréstimo posterior ao término do contrato proprietário. A suíte com IPC controlado confirmou as datas na prévia e a inclusão do lote. No Tauri nativo, a prévia e a revisão mostraram o empréstimo de Leandro ao Fortaleza de 31/12/2003 para 29/02/2004. O CLI releu a nova data em `v07vencimento.cm4`, com salário semanal de £328, contrato do Cruzeiro preservado e hash da origem intacto. O CM0304 mostrou 29/02/2004 no empréstimo e 01/01/2005 no Cruzeiro. A partida avançou sete dias até 26/07/2003, foi regravada em `v07venc7d.cm4` e carregou após reiniciar o jogo. A interface e o leitor Rust confirmaram as duas datas e os salários da cópia regravada. SHA-256: `1045AB5A2F258F12A12158F85A371C1C0CA9C99E9122ADF1F3EAF6641A22F401`.

O CLI e a tela de clubes da v0.7 projetam salários semanais em lote por UID e índice do vínculo, com ajuste percentual e teto opcional. A tela separa os totais registrados do clube e dos empréstimos e permite incluir os alvos na revisão. Na amostra inicial, +20% com teto de £600 levou os vínculos de Liam Chilvers a £600 no Arsenal e £571 no Northampton. A cópia foi relida pelo Rust com os dois alvos, e o hash da origem permaneceu igual. O cenário com IPC controlado percorreu a prévia e a inclusão pela interface. O ciclo nativo de salários e vencimentos está documentado acima para o Fortaleza. A variante de Liam também passou pelo ciclo de abertura, avanço, regravação e recarga descrito nas evidências da v0.7.

O núcleo Rust, o CLI e a área de clubes do Tauri comparam elencos entre saves por UID do clube e do jogador. Nos saves inicial e de um mês, o Fortaleza teve cinco jogadores com diferenças e quatro sem alterações nos campos comparados. O Braga teve quatro saídas e uma alteração de contrato. Na cópia do teste de Clodoaldo, o relatório mostrou Passe de 13 para 20. A suíte com IPC controlado confirmou entradas, saídas, diferenças e exportação JSON. No Tauri nativo em Windows, a seleção dos dois saves reais e a exportação do Fortaleza pelos diálogos de arquivo geraram o mesmo relatório do CLI, acrescido dos nomes dos saves. O JSON nativo teve SHA-256 `E59867513BA090FCC0BDCC34366ED2D46E3636EEC65668914B0497A19DF98F7D`.

O perfil do clube da v0.7 inclui faixas etárias com acesso aos jogadores, distribuição por posição e médias de Passe, Finalização, Desarme, Velocidade e Reflexos. Os dados vêm do save aberto. A suíte com IPC controlado confirmou as contagens, a média formatada e a navegação do resumo ao jogador.

A comparação de jogadores da v0.7 exporta JSON versionado com os dois perfis, valores numéricos e diferenças, contratos e, quando aplicável, nome e data de cada save. A suíte com IPC controlado confirmou datas de origem no cabeçalho, salários e diferenças no JSON. A comparação de elencos inteiros também está implementada, com exportação JSON e CSV.

Na v0.8, o fechamento da janela principal do Tauri pede confirmação quando há um plano ativo. No Windows, Alt+F4 abriu o aviso nativo. Voltar manteve os alvos do lote e Fechar editor encerrou a janela no segundo pedido.

Na v0.8, ao abrir um save, o editor recupera a última busca concluída, incluindo texto, filtros, ordem e modo de lista de observação. A suíte com IPC controlado confirmou a restauração de texto e ordenação após recarregar a interface.

No Tauri nativo em release, a busca exata por UID retornou apenas Quim para ` UID: 1855 `, mostrou zero resultados para um UID ausente e exibiu erro para `uid:abc`. A busca por nome encontrou Clodoaldo na mesma sessão após o erro.

Na v0.8, a busca paginada de jogadores roda fora da thread da interface, com indicação de carregamento e cancelamento cooperativo durante o filtro no Rust. A suíte com IPC controlado confirmou que uma resposta tardia é descartada e que a próxima busca volta a mostrar resultados. A comparação individual e a de elencos compartilham o cancelamento cooperativo da leitura e do processamento. No WebView2 nativo em build de depuração, foi acionado o cancelamento da comparação de Clodoaldo entre os dois saves fornecidos enquanto o seletor do Windows estava aberto. Após confirmar a seleção, a operação retornou como cancelada. A repetição na mesma sessão abriu o resultado. O ensaio nativo confirma cancelamento antes da leitura, e a suíte com IPC simulado verifica descarte de uma resposta tardia.

A interface da v0.8 expõe atalhos para abertura, revisão, desfazer/refazer e salvar cópia na revisão. A suíte de interface com IPC controlado percorreu a revisão, o histórico e a gravação usando o teclado. Os testes nativos do beta.16 confirmaram os atalhos de revisão, histórico e gravação.

Na v0.8, a revisão guarda localmente o resumo das últimas 30 cópias verificadas, com caminhos, data e hashes da origem e da cópia. Novas gravações também arquivam o relatório detalhado em IndexedDB para consulta no histórico. Após reiniciar o Tauri no Windows, o botão de saves recentes reabriu a origem e o histórico preservou os dois salários do lote do Fortaleza, com valores anteriores e finais. O botão Abrir cópia carregou o arquivo registrado como save ativo.

A revisão da v0.7 exibe painéis recolhíveis por entidade, distinguindo jogador, comissão e clube. A suíte de interface com IPC controlado confirmou três alterações em um único painel do clube. No beta.10, a revisão também agrupou por operação. O WebView2 confirmou remoção de uma operação anterior, preservação da posterior, desfazer/refazer e gravação apenas do campo mantido. [Evidência nativa](docs/DEVELOPMENT.md#evidências-locais-do-beta)

Os presets da v0.7 agora podem ser renomeados e duplicados pelo formulário de lote, com rejeição de nomes já usados. Um perfil sem biblioteca salva recebe três presets iniciais editáveis para passe, reflexos e condição. A suíte de interface com IPC controlado confirmou as configurações no JSON exportado após a duplicação e a renomeação.

Na v0.7, a busca de jogadores permite mostrar ou ocultar Clube, Idade, CA, PA e UID. A preferência é local e foi verificada após recarregar a interface em navegador com IPC controlado.

A [referência de desempenho da v0.8](reports/v0.8-performance.md) registra cinco execuções por operação e save no CLI Rust em release. A gravação passou de 992,0 para 596,9 MiB de pico observado no save maior, com cópias idênticas por SHA-256. As medições de sessão e interface Tauri seguem pendentes.

Na v0.8, o comando `npm run build:installer` gerou o primeiro instalador NSIS x64 local com a versão de prévia `0.5.0-alpha.1`. O build terminou com um aviso sobre os metadados usados pelo atualizador do Tauri. No Windows de desenvolvimento, a instalação em pasta de teste registrou o produto no perfil atual, abriu a janela do editor e removeu o executável pelo desinstalador. A instalação em Windows limpo é opcional. Veja [as instruções de empacotamento](docs/DEVELOPMENT.md#instalador-windows-para-testes-da-v08).

Na v0.7, a primeira prévia de lote já permite selecionar jogadores de um elenco, aplicar valor fixo, pontos, percentual, piso ou teto e revisar os limites por jogador. O lote validado entra no histórico como uma operação. O formulário explica o arredondamento para a unidade mais próxima, com empates afastados de zero e resolução de 0,01 ponto percentual para condição e ritmo. Na prévia Rust do save inicial, +5% elevou a moral de 10 para 11 para Quim e Clodoaldo, enquanto +100% no Passe levou Quim de 4 para 8 e marcou Clodoaldo como limitado a 20. O frontend passou 22 verificações com IPC controlado. Seleção por filtros, presets, comparações e validação nativa permanecem em aberto.

Evidências: `reports/v0.2-v0.3-validation.md` e `reports/v0.4-validation.md`. As prévias são marcos de desenvolvimento, não uma declaração de estabilidade da v1.0.

Na v0.6, CA/PA e reputação de staff têm escrita experimental no plano CLI e formulário integrado ao Tauri, com políticas de CA e efeitos derivados registrados. Onze verificações de frontend com IPC controlado passaram. A cópia corrigida de Rui e Pär passou avanço de oito dias, regravação e recarga; o beta.10 também confirmou a revisão e gravação do formulário nativo para Rui Águas. Detalhes em `reports/v0.6-staff-header.md`.

A conferência no jogo validou os efeitos previstos de Rui no carregamento e revelou uma correção necessária para jogador/staff: os atributos profissionais usam CA de jogador. Leitura, inversa e projeção foram corrigidas; a nova cópia passou o ciclo no jogo, e falta ampliar a matriz. Evidências em `reports/v0.6-dual-staff-correction.md`.

A edição experimental de oito campos de personalidade está integrada ao núcleo e ao formulário de staff, com proteção a registros compartilhados e conflitos entre papéis. Dois testes de integração e 12 verificações de frontend foram aprovados. O Tauri nativo gravou a variante, e o CM0304 4.1.5 carregou, avançou, regravou e recarregou a carreira. O jogo voltou a mostrar Adaptabilidade 20 para Rui Águas e a releitura confirmou Adaptabilidade e Profissionalismo 20 para Rui e Pär Zetterberg. [Relatório de validação](reports/v0.6-staff-personality.md) Gestão de clubes permanece no escopo da v0.6.

A consulta de clubes por UID, elenco e staff/direção foi integrada ao núcleo, CLI e frontend Tauri. Dois testes de clubes e 16 verificações de frontend passaram. Falta conferir a navegação nativa e implementar/validar finanças e infraestrutura. Evidências em `reports/v0.6-clubs-navigation.md`.

A ficha financeira da v0.6 mostra as despesas registradas de remunerações e prêmios para o mês atual, o mês anterior e a temporada. O núcleo fornece os valores para registros financeiros completos. No Tauri nativo, a ficha do Fortaleza em `pgtoeditfim.cm4` exibiu as mesmas seis cifras do CM0304, incluindo £103.486 em prêmios no mês anterior. A folha estimada a partir dos contratos permanece disponível na mesma ficha.

O diagnóstico Rust `club-data` lê os registros financeiros e os campos brutos de infraestrutura nas duas amostras. Cada amostra contém 3.415 registros financeiros básicos, 138 completos e um sem saldo. O campo identificado como saldo do Braga passa de 525.326 no save inicial para 1.464.831 no save de um mês. Na cópia de 27/07, o jogo confirmou o saldo de £685.427 do registro completo. O orçamento de transferências da temporada e o restante foram identificados e editados separadamente. O CM0304 exibiu os alvos de £900.000 e £700.000. As demais variantes financeiras seguem em investigação. Veja [a conferência no jogo](reports/v0.6-club-finance-native.md).

Na v0.5, os casos de suspensão de 19 e 58 bytes passaram avanço/regravação/recarga. Contratos de Clodoaldo e empréstimo de Liam passaram o mesmo ciclo: o vencimento de Clodoaldo foi convertido pelo jogo em contrato mensal; salários e vínculos de Liam persistiram. Detalhes e pendências em `reports/v0.5-suspensions.md` e `reports/v0.5-contract-variants.md`. A matriz de estabilização mantém as variantes em investigação.

Na v0.6, a edição experimental de balanço de clubes com registro financeiro completo está integrada à revisão, ao histórico e à gravação em cópia no Tauri. O frontend passou 21 verificações com IPC controlado. A ficha do clube mostra a folha contratual e a estimativa após contribuições de empréstimo, conferidas para o Braga em duas datas. A lista de vencimentos de 90 dias reflete o plano de edição e abre o contrato correspondente. O jogo carregou o balanço editado de £2.500.000 e regravou £2.811.025 após oito dias, em acordo com a evolução do lucro. Treino e formação foram editados em conjunto no Tauri nativo, com cópia relida e dois bytes alterados. O CM0304 carregou a cópia de infraestrutura do Braga, exibiu condições excelentes com academia e preservou treino 20 e formação 19 após sete dias e nova gravação.

O orçamento salarial e a folha registrada foram identificados nos registros financeiros completos. No Tauri nativo, o orçamento do Fortaleza foi alterado de £30.912 para £80.000 por semana, revisado, salvo e conferido ao reabrir a cópia pelo histórico. O CM0304 exibiu o teto alterado e preservou o valor semanal após avançar 12 dias e gravar novamente. Veja [a conferência de finanças](reports/v0.6-club-finance-native.md#orçamento-salarial-e-folha-registrada).

A ficha do clube também mostra a folha mensal projetada de jogadores quando alterações salariais no plano afetam o total. A projeção considera tanto o salário do contrato quanto a contribuição de empréstimo, limitada ao salário devido pelo clube proprietário. A folha gravada continua disponível para comparação. O teste Rust cobre a contribuição com alterações nos dois vínculos. No Tauri nativo, o aumento de £215 por semana no contrato de Clodoaldo projetou uma folha de £62.376 por mês. A cópia reaberta mostrou esse total nos contratos e £61.516 no campo registrado. No CM0304, a primeira consulta às finanças confirmou a projeção sem avançar o calendário. Em 24/06, a regravação manteve o salário editado e refletiu a evolução da folha durante a simulação.

A investigação da lotação do estádio identificou o vínculo da equipe principal do Braga com o Estádio 1º de Maio. O CM0304 confirmou a edição separada da capacidade total e dos lugares sentados em uma cópia de laboratório. O leitor Rust e a ficha do Tauri mostram os dois valores e sinalizam referências compartilhadas pelas equipes principais. A edição de lotação entrou na revisão do Tauri para estádios vinculados a uma única equipe principal de clube. Estádios compartilhados ficam disponíveis para consulta. O Rust gerou uma cópia idêntica à validada no CM0304. O formulário no Tauri nativo produziu uma cópia do Braga com o mesmo SHA-256 do save testado no CM0304. Os cinco testes Rust de clubes passaram. Veja [a conferência no jogo](reports/v0.6-club-finance-native.md#lotação-do-estádio).

## Funcionalidades da v1.0

### Escopo mínimo do editor

A v1.0 deve atender aos fluxos abaixo pela interface. Esta lista é compromisso de produto; não indica que os recursos já estejam prontos.

- **Montar um jogador:** editar atributos visíveis e ocultos, CA/PA, posições e pés numa única revisão; copiar apenas grupos escolhidos de outro jogador e criar um preset reutilizável.
- **Preparar um elenco:** selecionar titulares, reservas ou todo o clube; recuperar condição, reduzir cansaço, ajustar moral e remover lesões/suspensões suportadas, com prévia individual dos efeitos.
- **Gerenciar contratos:** localizar vencimentos próximos, editar os contratos existentes e conferir salário semanal, total salarial do clube, término, camisa e situação no elenco. Identificar contratos de empréstimo separadamente.
- **Melhorar a comissão:** encontrar treinadores, preparadores, olheiros e fisioterapeutas; editar os atributos específicos disponíveis, capacidade, reputação e personalidade; preservar dupla função jogador/staff.
- **Gerenciar o clube:** consultar elenco e comissão, ajustar saldo, orçamentos e infraestrutura mapeados, mostrando a unidade e o significado de cada campo.
- **Encontrar e comparar:** combinar filtros, ordenar resultados e comparar dois jogadores ou a evolução de uma entidade entre saves, inclusive atributos ocultos e contratos suportados.
- **Controlar as mudanças:** revisar valores anteriores e finais por entidade, retirar alterações do plano, desfazer/refazer operações inteiras e salvar cópia com relatório local.

### Fluxos adicionais para a v1.0

| Marco | Funcionalidade | Aceite |
|---|---|---|
| **0.5** | Diagnóstico contratual por clube com vencidos, próximos de vencer e empréstimos a terminar. Filtrar por prazo e ordenar por impacto na folha. | Encontrar os contratos dos próximos 90 dias e simular a folha após editar salários e vencimentos de vínculos existentes. |
| **0.6** | Painel da comissão por função e fundamentos de trabalho, com comparação entre os membros do mesmo clube. | Localizar quem tem o maior atributo de olheiro ou treino no clube, abrir a ficha e revisar a edição no contexto da comissão. |
| **0.7** | Grupos de elenco salvos localmente, com seleção manual e filtros por posição, idade ou situação contratual. Aplicar múltiplos ajustes em um único lote. | Selecionar uma lista de jovens, ajustar dois atributos e moral, excluir um jogador na prévia e desfazer o lote inteiro. |
| **0.7** | Simulação de ajustes salariais em lote, com total atual, projetado e diferença para o clube, além de impacto por vínculo. | Aplicar um percentual com teto, revisar os empréstimos separadamente e conferir o total ao reabrir a cópia. |
| **0.7** | Comparação de elencos entre dois saves da mesma carreira por UID, com entradas, saídas e diferenças de atributos, salários e vencimentos. | Comparar o save inicial e o de um mês e exportar as diferenças do clube selecionado. |
| **0.8** | Salvar versões de teste A e B a partir da mesma origem, com nome, data e resumo do plano no histórico local. | Gerar as duas cópias, localizar cada uma pelo histórico e confirmar que o hash da origem permaneceu igual. |

Os primeiros atalhos de lote serão **recuperar elenco**, **ajustar moral**, **aplicar piso/teto de atributos**, **aumentar/reduzir atributos selecionados** e **copiar um grupo de atributos**. Cada atalho usa as mesmas regras da edição individual; nenhum altera CA ou vínculos silenciosamente.

### Ampliação do escopo de edição

| Marco | Funcionalidade acrescentada | Demonstração |
|---|---|---|
| **0.5** | Editar valor de mercado e preço pedido separadamente, com remoção do preço pedido quando o registro representar essa opção. Investigar edição de valores de bônus por jogo e por gol e de cláusulas de rescisão já presentes no contrato. | Ajustar o preço pedido de um jogador e o salário de um empréstimo, conferir os dois clubes envolvidos e passar pela data relevante. Para bônus e cláusulas, confirmar também o efeito quando acionados. |
| **0.6** | Editar saldo, orçamento de transferências, orçamento salarial, reputação, treino e formação conforme a estrutura do clube. Investigar a edição de capacidade de estádio em registros identificados, incluindo a distinção entre capacidade total e lugares sentados. | Preparar uma revisão com alterações do clube e da comissão, projetar a folha e carregar a cópia. Para estádio, conferir o valor e a consistência após regravar no jogo. |
| **0.7** | Lote de recuperação que combine condição, cansaço e moral, com exclusões por jogador. Comparar folha atual e projetada, detalhar a contribuição de cada empréstimo e exportar a revisão com UIDs e valores anteriores e novos. | Preparar uma cópia de recuperação do Fortaleza com mudanças de jogadores, comissão e clube. Retirar uma alteração específica, desfazer/refazer e conferir a revisão e os totais na cópia. |
| **0.8** | Prototipar uma transferência definitiva entre clubes em cópia de laboratório. Mapear propriedade, elenco, contrato, inscrições, status de empréstimo e efeitos financeiros. | Transferir um jogador sem empréstimo em duas carreiras, conferir os dois clubes e avançar até uma partida elegível. Registrar o resultado e a decisão de escopo antes da v0.9. |

Bônus, cláusulas, estádio e transferência são investigações com critérios de aprovação. Os três primeiros podem entrar na v1.0 após validação dos registros e efeitos no jogo. A transferência só entra na v1.0 mediante revisão explícita do escopo após o protótipo. As demais operações da tabela compõem a entrega planejada.

O alvo da v1.0 inclui os recursos abaixo, além da edição campo a campo. As caixas representam entregas pendentes; identificar um campo ou implementar sua leitura não conclui a edição.

| Versão | Funcionalidades adicionais | Demonstração de aceite |
|---|---|---|
| **0.5** | Central de contratos com filtros de vencimento, vínculo e situação; resumo de salário semanal/mensal; ações de condição, descanso e moral. | Localizar vencimentos, editar contratos existentes e conferir os efeitos após a data relevante no jogo. |
| **0.6** | Perfil completo de staff; navegação clube → elenco/comissão; painel financeiro e de infraestrutura. | Editar um treinador, um olheiro e um fisioterapeuta; validar também uma dupla função e os campos de clube. |
| **0.7** | Editor de lotes com valores absolutos, incrementos e percentuais nos campos numéricos compatíveis; arredondamento explícito, limites e prévia por jogador. | Aplicar um aumento ao elenco, mostrar quem atingiu o limite e desfazer a operação inteira. |
| **0.7** | Presets por posição e por grupo de atributos; copiar somente os campos escolhidos; editor local de presets com importação/exportação. | Copiar fundamentos de um jogador sem copiar identidade ou contrato e conferir cada destino. |
| **0.7** | Listas de observação locais; colunas configuráveis; filtros salvos; comparação lado a lado e entre saves. | Salvar uma lista, reabrir o aplicativo e comparar os mesmos UIDs entre duas datas da carreira. |
| **0.7** | Revisão agrupada por clube, entidade e operação; remover uma entidade ou operação do plano; relatório de antes/depois exportável. | Excluir um jogador de um lote antes de gravar e confirmar que ele permaneceu intacto. |
| **0.8** | Sessão de trabalho local: guardar o plano pendente e retomá-lo com verificação do hash da origem. | Retomar uma edição após fechar o aplicativo; recusar reaplicação automática se o save de origem mudou. |

O editor permite criar e excluir listas de observação por nome e alternar a lista ativa. A seleção e os jogadores são locais. A lista anterior é recuperada em “Observação”. A suíte Playwright validou a migração, a independência entre listas, a restauração da lista ativa e a comparação do mesmo UID entre saves. Os 33 cenários passaram.

A suíte Playwright com IPC simulado confirmou que a seleção de colunas e a lista de observação sobrevivem à recarga da interface. Depois de reabrir o save, o filtro de observação manteve o UID 600, a comparação com outro save consultou esse UID e a remoção retirou o jogador dos resultados filtrados. Os 30 cenários passaram.

A retomada do plano valida o hash da origem e recusa a importação se o arquivo tiver mudado no disco desde a abertura. O teste Rust confirmou os dois estados em arquivos reais. No Playwright com IPC simulado, a recusa preservou a revisão pendente e o mesmo plano foi restaurado após liberar a origem de teste.

Listas, presets e planos de trabalho são arquivos locais do editor. A edição percentual não se aplica a identificadores, datas, posições ou categorias contratuais; cada campo informa suas operações permitidas. Nenhuma dessas funções autoriza alterar estruturas ainda não mapeadas.

### Direção do produto

A v1.0 será um editor de carreira com navegação entre jogador, contrato, comissão e clube. A edição de atributos é apenas a primeira área. O escopo inclui operações individuais e em lote, ferramentas de comparação e revisão das alterações antes de gravar uma cópia.

Prioridade de implementação: completar jogadores e contratos, acrescentar staff e clubes e então integrar as operações em lote. A interface desktop será desenvolvida em Tauri, com toda leitura, conversão, validação e escrita no núcleo Rust.

| Área | Escopo planejado |
|---|---|
| **Jogadores** | Atributos técnicos, mentais, físicos, de goleiro e ocultos; edição de CA/PA, posições, lados, pés, personalidade e reputação. |
| **Dados pessoais** | Data de nascimento, nacionalidade principal, altura e peso. Consulta de identidade, UID e nacionalidades adicionais. |
| **Estado** | Condição física, cansaço e moral; recuperar lesionados e remover suspensões por operações específicas, após mapear todas as referências necessárias. |
| **Contratos** | Editar salário, término, camisa e situação no elenco em contratos existentes. Distinguir clube detentor, equipe atual e empréstimo. |
| **Valores** | Editar campos financeiros do jogador com efeitos confirmados, distinguindo valor exibido e valor de venda. |
| **Comissão técnica** | Busca e edição de atributos profissionais, CA/PA, reputação e personalidade. Suportar a distinção entre staff e jogador/staff. |
| **Clubes** | Busca, navegação por elenco e staff; edição de saldo, orçamentos existentes e níveis de treino/formação identificados no save. |
| **Edição em lote** | Seleção por elenco ou filtro; definir, aumentar/reduzir ou limitar atributos; presets e prévia antes de aplicar. |
| **Análise** | Comparar jogadores lado a lado; filtros de posição, idade, CA/PA e atributos; comparar saves e exportar listas. |
| **Fluxo** | Múltiplas alterações, desfazer/refazer antes de salvar, revisão por entidade, cópia validada e histórico local de relatórios. |

### Operações que devem estar disponíveis na interface

| Tela | Ações previstas até a v1.0 |
|---|---|
| Perfil do jogador | Editar vários atributos juntos; alterar CA/PA, posições e pés; ajustar dados pessoais, reputação e personalidade; revisar efeitos indiretos. |
| Estado e contrato | Recuperar condição, descansar, ajustar moral, tratar lesão/suspensão validada, alterar salário, vencimento, camisa, situação no elenco e valores financeiros. |
| Elenco do clube | Selecionar parte ou todo o elenco, recuperar jogadores, aplicar ajustes de atributos e localizar contratos próximos do fim. |
| Comissão técnica | Editar atributos de trabalho e capacidade, reputação e personalidade; acessar o perfil de jogador quando houver dupla função. |
| Gestão do clube | Ajustar saldo, orçamentos e infraestrutura confirmados; consultar elenco, comissão e compromissos salariais. |
| Ferramentas | Copiar conjuntos de atributos, aplicar presets, comparar jogadores e saves, exportar resultados e desfazer uma operação inteira. |

Cada operação deve informar os alvos e os valores finais antes da aplicação. Recursos dependentes de engenharia reversa continuam como pendências do marco correspondente até a validação; sua ausência impede declarar aquele marco concluído.

São metas de implementação, não recursos já demonstrados. Cada domínio depende de identificar os campos, validar referências e conferir os efeitos no jogo. Se uma meta não puder ser validada, o roadmap deverá ser revisto explicitamente antes da v1.0.

### Regras entre campos

- **CA/PA:** distinguir PA fixa e valores especiais, se presentes. Ao alterar CA, mostrar o efeito nos técnicos e oferecer a escolha entre preservar valores convertidos, quando representáveis, ou manter bytes relativos e aceitar os valores recalculados. Sem política silenciosa.
- **Posições:** mudar a classificação de goleiro pode trocar o grupo de conversão; conferir todos os atributos afetados.
- **Lesões e suspensões:** distinguir estado ativo, histórico e referências. Não implementar recuperação apenas zerando um byte sem validar a estrutura.
- **Contratos e finanças:** mostrar moeda, unidade salarial, datas e entidade contratante. Distinguir saldo, orçamento de transferências e salarial.
- **Lotes:** apresentar afetados, conflitos e alvos inválidos; nenhuma aplicação parcial silenciosa.

## Marcos

### Escopo detalhado por versão

**v0.4 — Editor completo de jogadores**

- Editar técnicos, mentais, físicos, atributos de goleiro e ocultos individualmente ou em conjunto.
- Editar CA, PA, reputação nacional/atual/mundial e personalidade: profissionalismo, ambição, lealdade e demais campos identificados.
- Editar familiaridade por posição, lado de atuação e habilidade com cada pé.
- Editar nascimento, nacionalidade principal, altura e peso; calcular idade na data do save.
- Mostrar valor original, valor proposto e alterações indiretas causadas por CA ou posição.
- Organizar o perfil em abas e sinalizar campos ainda sem validação; procurar jogadores com e sem acentos.

**v0.5 — Carreira, estado e contratos**

- Recuperar condição física, reduzir cansaço e ajustar moral de um jogador ou seleção.
- Consultar lesão, previsão de retorno e suspensão; operações para recuperar/remover apenas quando as referências estiverem validadas.
- Editar salário e vencimento de contratos existentes, camisa e situação no elenco.
- Consultar clube proprietário, equipe atual e vínculo de empréstimo, evitando confundir essas entidades.
- Editar valor de venda e outros valores financeiros identificados, explicando o que o jogo recalcula.
- Exibir resumo contratual e avisos de datas inconsistentes antes de aplicar.

**v0.6 — Staff e gestão de clubes**

- Buscar membros da comissão por nome, clube e função; navegar entre clube, elenco e comissão.
- Editar atributos de treino, observação e gestão, além de CA/PA, reputação e personalidade suportadas.
- Tratar jogadores que também integram a comissão sem perder nenhum dos vínculos.
- Editar saldo, orçamento de transferências e orçamento salarial nos campos existentes e confirmados.
- Editar níveis de instalações de treino e formação identificados; consultar reputação do clube e permitir edição após validação.
- Mostrar quantidade de jogadores, distribuição por posição, salários e contratos próximos do vencimento.

### Evidências da v0.6

| Parte da demonstração | Resultado verificado |
|---|---|
| Navegar entre clube, elenco e comissão | O Tauri nativo abriu a comissão do Braga, comparou os atributos, filtrou pela função e abriu a ficha selecionada. A suíte também cobre navegação entre perfis de dupla função. [Progresso](#progresso-da-implementação) |
| Editar comissão e preservar dupla função | Rui e Pär passaram por avanço, regravação e recarga na variante corrigida de CA e atributos. O formulário nativo conferiu CA, PA e reputação de Rui com política explícita. A edição de personalidade também persistiu nos registros de staff e jogador de Pär em `b.cm4`. [CA e reputação](reports/v0.6-staff-header.md) · [Personalidade](reports/v0.6-staff-personality.md) |
| Editar finanças do clube | O Tauri gravou saldo e orçamento salarial do Fortaleza em variantes verificadas no jogo. Os orçamentos de transferências do Braga permaneceram em £900.000 e £700.000 em `t.cm4`. [Finanças](reports/v0.6-club-finance-native.md) |
| Conferir infraestrutura e reputação | `bragainfra7d.cm4` preservou treino 20 e formação 19. O CM0304 exibiu a reputação Mundial do Braga após reabrir `26.cm4`. O ensaio de estádio exclusivo também conferiu lotação e público. [Infraestrutura](reports/v0.6-club-finance-native.md) |

Saldo em registro básico e lotação de estádio compartilhado seguem em investigação. A aprovação integral da matriz da v1.0 continua aberta.

**v0.7 — Ferramentas para editar e analisar em escala**

- Filtrar por clube, nacionalidade, idade, posição, pé, CA/PA, atributo e situação contratual; combinar filtros e ordenar resultados.
- Selecionar jogadores individualmente, todo o elenco ou todos os resultados filtrados, com contagem explícita.
- Aplicar valor fixo, incremento/redução e limites mínimo/máximo em campos compatíveis.
- Ajustar salários e vencimentos de contratos existentes em lote, com seleção explícita, unidade salarial, datas e validação de cada vínculo; separar empréstimos dos contratos do clube proprietário.
- Criar e salvar presets locais, por exemplo: recuperar elenco, ajustar profissionalismo ou melhorar fundamentos de uma posição.
- Copiar atributos selecionados de um jogador para outro, preservando identidade e vínculos; validar conversões e limites de cada destino.
- Comparar jogadores lado a lado e comparar a mesma entidade entre dois saves por UID.
- Exportar resultados e diferenças para CSV/JSON, salvar filtros e manter histórico local das alterações realizadas.
- Desfazer/refazer transações inteiras e remover uma alteração pendente antes de salvar.
- Mostrar um painel do elenco com distribuição de idade/posição, médias de atributos, folha salarial e vencimentos próximos; permitir abrir os jogadores a partir desses resumos.
- Oferecer presets iniciais editáveis e permitir duplicar, renomear, excluir e exportar/importar presets locais, com versão e validação dos campos.

Os presets locais guardam vários ajustes, incluindo condição, ritmo, cansaço, moral e personalidade. A exportação usa JSON v2 e a importação aceita v1. Na prévia de lote, campos indisponíveis aparecem por jogador e bloqueiam a revisão até que a seleção seja corrigida. Um teste de interface confirmou a exclusão de um jogador na segunda página, mantendo os 50 compatíveis e exigindo uma nova prévia. A conferência desse cenário usou respostas de teste para a comunicação com o Rust.

No beta.10, a cópia seletiva no Tauri usou Fredrik como origem e aplicou apenas Passe a Artur Jorge e Pedro Costa. A releitura confirmou Passe 14 em ambos e preservou CA, Técnica, identidade, vínculos e estado. Só mudaram dois bytes, um por destino. [Evidência nativa](docs/DEVELOPMENT.md#evidências-locais-do-beta)

A comparação de elencos exporta JSON e CSV. O CSV inclui origem e destino, entradas, saídas, diferenças por atributo e contratos de cada save quando o conjunto de vínculos muda. A geração foi conferida com os dois saves de amostra para o Braga, com 40 jogadores com diferenças e quatro saídas. O Playwright confirmou que os dois botões enviam ao Tauri os dados no formato esperado. No Tauri para Windows, a exportação nativa gravou 170 linhas de dados, com 163 diferenças de atributos, quatro saídas, dois registros do contrato de Quim e um total de 32 jogadores sem diferenças. Veja [o CSV do Braga](reports/v07-braga-comparacao-native.csv).

No Tauri nativo, o preset Recuperação foi aplicado a Artur Jorge e Quim no save inicial do Braga. A revisão apresentou seis campos e dez bytes alterados. A releitura de `bragarecuperacaopreset.cm4` confirmou condição e ritmo a 100%, cansaço zero e moral 18 nos dois jogadores. O teste também confirmou Ctrl+Shift+S na revisão.

O CM0304 4.1.5 carregou a cópia de recuperação em 19/07/2003. Os perfis de Artur Jorge e Quim mostraram condição física de 100% e moral “Muito Bom”. Ritmo e cansaço foram conferidos na releitura dos campos do save. Após avançar até 27/07, o jogo regravou `bragarecuperacaor.cm4` com moral 18 nos dois perfis. Artur Jorge ficou com condição 100%, ritmo 95,20% e cansaço -84. Quim ficou com condição 98%, ritmo 100% e cansaço -29. A cópia de recuperação manteve o SHA-256 original. A regravação abriu após reiniciar o CM0304. Ambos os perfis exibiram moral “Muito Bom” e condição 100% na recarga, enquanto o campo de condição gravado de Quim era 98%.

### Evidências da v0.7

| Parte da demonstração | Resultado verificado |
|---|---|
| Filtrar elencos, aplicar recuperação e tratar exceções | A suíte da interface cobriu filtros, grupos e exclusão na prévia. No Tauri, o preset Recuperação foi aplicado no Braga e a exclusão de um vínculo no lote de situação do Fortaleza foi preservada após regravação e recarga no jogo. |
| Combinar jogador, comissão e clube | A revisão nativa do Braga reuniu recuperação, salário, vencimento, Adaptabilidade e orçamentos em `0aceitetauri.cm4`. O jogo regravou e recarregou a variante como `t.cm4`. |
| Conferir folha, salários e vencimentos | O Tauri projetou e gravou salários e vencimentos no Fortaleza. O jogo preservou os alvos e o vínculo do Cruzeiro nos arquivos `v07salarios7d.cm4` e `v07venc7d.cm4`. |
| Revisar operações e comparar saves | O Tauri confirmou retirada, desfazer e refazer, além da exportação da comparação por UID. O CSV do Braga registra diferenças de atributos, saídas e vínculos. [Exportação](reports/v07-braga-comparacao-native.csv) |

A variante de dois salários de Liam Chilvers também foi validada no CM0304. A cópia `v07liamsalarios.cm4` já está na pasta de saves do jogo. A releitura confirmou Arsenal de £523 para £600 por semana e Northampton Town de £476 para £571 por semana, com três bytes alterados e sem mudanças nos demais campos do jogador. O SHA-256 da origem permaneceu igual. SHA-256 da cópia `669B2B0A23D1850423A1AD0D10798FC7C1A1F47A539934A2707795B960012FAC`.

O jogo abriu uma cópia idêntica em 19/07/2003 às 09h e mostrou £2.400/mês no Arsenal, dos quais £120/mês durante o empréstimo, e £2.300/mês no Northampton. Em 20/07 às 14h, gravou o avanço em `30.cm4` e retomou o arquivo após reiniciar. O leitor Rust confirmou £600 e £571 por semana e os vencimentos de 01/06/2005 e 31/10/2003. SHA-256 da regravação `3C1FFB6D3BEA2E720A2C2A5469E588BD30D01943FC2F52807FBBFB7EA2665B29`.

**v0.8 — Produto utilizável no Windows**

- Gerar instalador e documentar dependências de execução do Tauri/WebView2.
- Implementar recentes, restauração de filtros/preferências, atalhos e estados de carregamento/erro claros.
- Permitir cancelar buscas ou análises demoradas e confirmar descarte de alterações pendentes.
- Medir abertura, busca, consumo de memória e gravação nas amostras; registrar limites e corrigir travamentos.
- Validar instalação e desinstalação em uma pasta isolada no Windows e ampliar a matriz de saves e usuários.
- Guardar relatórios de gravação consultáveis no aplicativo e permitir localizar a cópia gerada e sua origem.

### Sequência de entrega e demonstrações

| Marco | Demonstração necessária na interface |
|---|---|
| **0.4 — Jogador** | Abrir Clodoaldo, editar vários grupos, revisar conversões e salvar uma cópia. |
| **0.5 — Carreira** | Alterar estado e contrato, remover uma lesão e uma suspensão suportadas e conferir os efeitos após evolução no jogo. |
| **0.6 — Clube** | Abrir um clube, navegar entre elenco e comissão, editar staff e dados financeiros/infraestrutura validados. |
| **0.7 — Lote** | Filtrar e recuperar um elenco, combinar edições de jogador, comissão e clube, revisar exceções, conferir a folha projetada e comparar a cópia com a origem. |
| **0.8 — Beta** | Usar o instalador do beta e completar os fluxos anteriores em outra carreira. |

Os marcos funcionais da v0.5 à v0.7 foram demonstrados localmente. O aceite usa a cobertura local das operações suportadas. As variantes em pesquisa estão registradas na matriz.

No Tauri para Windows, o cancelamento da comparação do Braga foi acionado depois de selecionar o segundo save. A sessão ficou disponível sem abrir um relatório parcial. A repetição na mesma janela concluiu a comparação e exportou o CSV com o UID 1488 nos dois registros do contrato de Quim.

O instalador atualizado foi testado no Windows de desenvolvimento em uma pasta de teste. A versão instalada abriu o save inicial, alterou o drible de Clodoaldo de 16 para 17 e reabriu a cópia pelo histórico. A comparação binária identificou um byte alterado. A desinstalação removeu o executável e preservou a cópia fora da pasta de instalação. O teste em Windows limpo é opcional.

O Tauri gravou uma variante com Passe 20 para Clodoaldo. Após recarregar a interface, o histórico foi aberto sem um save ativo e o relatório arquivado foi exportado pelo diálogo Salvar Como. O JSON registrou Passe 13 → 20 e os dois SHA-256 coincidiram com os arquivos de origem e cópia no disco.

O protótipo `transfer-lab` validou equipe, contrato e lista de plantel em duas carreiras. No Braga, Rui Borges apareceu no plantel do Pampilhosa após a atualização da lista. Na carreira brasileira, Ronaldo Angelim foi titular pelo Ceará contra o Remo e manteve vínculo e salário na regravação. A realocação altera a distribuição dos salários entre os clubes. Taxa de transferência, inscrições em outras competições e reconciliação do histórico ainda exigem evidência. Para a v1.0, a transferência fica na CLI de laboratório. A promoção ao Tauri depende da prova de saída com a lista de inscrição fechada. [Evidências do protótipo](research/FORMAT.md#transferências-observadas-em-duas-datas)

**v0.9 — Estabilização**

A prévia `0.9.0-beta.1` inicia a série de estabilização com as funcionalidades já disponíveis na alpha.29, a retenção dos filtros de vencimento por clube e a mensagem de incompatibilidade da versão interna do banco. A aprovação da matriz final e os ensaios externos continuam sendo critérios de conclusão do marco.

- Congelar funcionalidades, corrigir regressões e concluir a matriz de operações e variantes suportadas.
- Finalizar ajuda em português, exemplos, limitações, notas de versão e instruções de recuperação usando a cópia original.
- Revisar mensagens, navegação por teclado, legibilidade, licenças e conteúdo do pacote.

Na base da alpha.18, passaram 69 testes Rust em modo release, 12 testes de paridade e diagnóstico em JavaScript e 34 cenários Playwright. A regressão da interface cobre cancelar e confirmar a exclusão de uma lista de observação, preservar a lista padrão e navegar pelas abas do jogador com End e seta para a direita. O rótulo Lista ativa agora está associado ao seletor e foi conferido pelo nome acessível.

**v1.0 — Lançamento**

- Publicar pacote versionado para Windows, hashes de verificação e documentação.
- Publicar uma lista precisa dos campos editáveis, formatos aceitos e limitações conhecidas.
- Entregar todas as funções acima que compõem o escopo aprovado; qualquer retirada exige revisão explícita deste documento.

As funções de edição dependem de evidência do formato e do efeito no jogo. Encontrar um campo no leitor não basta para declarar sua escrita suportada. Implementações em andamento só mudam o estado de um marco após cumprir seu critério de conclusão.

| Versão | Entrega | Critério de conclusão |
|---|---|---|
| **0.2 — Núcleo Rust** | Migrar leitor, conversões, busca e escritor; plano de múltiplas alterações e CLI Rust de diagnóstico. | Paridade com a prova de conceito e edição de vários campos/jogadores, com diferenças binárias limitadas ao plano. |
| **0.3 — Aplicativo Tauri** | Abrir save, buscar jogadores, perfil, edição já suportada, revisão e salvar cópia. | Fluxo completo sem terminal, usando o núcleo Rust. |
| **0.4 — Jogadores** | CA/PA, posições, pés, personalidade, reputação e dados pessoais. | Regras entre CA, posição e atributos verificadas; novos campos conferidos no jogo. |
| **0.5 — Estado e contratos** | Condição, cansaço, moral, lesões, suspensões, salário, término, camisa, situação no elenco e valores. | Cada operação passa pelo ciclo de carregamento, conferência, avanço, regravação e novo carregamento. |
| **0.6 — Staff e clubes** | Editor de staff, elenco, saldo, orçamentos e infraestrutura mapeada. | Entidades e efeitos confirmados, incluindo jogadores/staff e estruturas financeiras distintas. |
| **0.7 — Produtividade** | Lotes, presets, comparação, filtros avançados, exportação e desfazer/refazer. | Prévia e resultado coincidem, com conflitos e exceções explícitos. |
| **0.8 — Beta Windows** | Pacote distribuível, desempenho, matriz ampliada e testes com usuários. | Funcionar fora do ambiente de desenvolvimento e em outras carreiras. |
| **0.9 — Candidata final** | Escopo fechado, regressões, documentação e pacote final. | Nenhum bloqueador aberto e matriz completa aprovada. |
| **1.0 — Estável** | Publicação do editor, documentação e lista de suporte. | Todos os critérios de lançamento atendidos. |

Marcos sem datas prometidas. A validação começa na v0.2 e acompanha todas as entregas. A migração e a primeira interface precedem a expansão dos domínios.

## v0.2 — Concluída

- [x] Inicializar controle de versão; jogo, saves e referências externas fora da distribuição.
- [x] Criar crate do núcleo Rust, separando contêiner, banco, modelos, conversão, edição e persistência.
- [x] Migrar leitura com verificação de limites e erros contextualizados.
- [x] Comparar contagens, identidades, valores, offsets e hashes com JavaScript nas duas amostras.
- [x] Migrar conversão/inversa, testando os dois modos para CA de 0 a 200 e alvos impossíveis.
- [x] Criar plano por UID/campo, com conflitos, valores anteriores/novos e detecção de dados compartilhados entre entidades.
- [x] Editar vários campos e jogadores em transação; nesta etapa CA/PA continuam inalteradas.
- [x] Gravar em temporário, reler, conferir diferenças planejadas e finalizar sem sobrescrever arquivos existentes.
- [x] Testar formato incompatível, truncamento, referências inválidas, falha de escrita e cancelamento antes da finalização.
- [x] Relatório com hashes, valores convertidos/brutos e offsets de diagnóstico.

**Conclusão:** Rust reproduz a leitura atual e gera uma cópia com vários campos de dois jogadores editados, validada pelo leitor e pelo jogo, com original intacto.

## Interface

- **Início:** abrir save, recentes, compatibilidade e progresso.
- **Jogadores:** tabela virtualizada, busca, filtros, seleção múltipla e comparação.
- **Perfil:** abas de atributos, posições, dados pessoais, estado e contrato.
- **Staff:** função, atributos e vínculo.
- **Clubes:** informações, elenco, staff, finanças e infraestrutura.
- **Lotes:** seleção, operação, prévia e validação.
- **Alterações:** diferenças por entidade, desfazer/refazer, conflitos e salvar cópia.
- **Diagnóstico:** versão, contagens, erros e relatório técnico.

Campos sem suporte não recebem controles de edição funcionais apenas em aparência. Bytes e offsets ficam nos detalhes de diagnóstico.

## Validação

Usar pelo menos quatro saves de duas carreiras distintas, com início e data posterior de cada carreira. Ampliar as amostras para cobrir ligas, tamanhos de banco, goleiros, jogador/staff, empréstimos, lesionados, suspensos e estruturas financeiras diferentes.

| Carreira | Save local | Data | Blocos | Passe de Clodoaldo ao gravar e reler pelo CLI |
|---|---|---|---:|---:|
| A · amostras originais | `save_inicial.cm4` | 19/07/2003 | 87 | 13 → 20 |
| A · amostras originais | `save_modificado.cm4` | 16/08/2003 | 267 | 13 → 20 |
| B · jogo novo com duas divisões do Brasil, treinador no Fortaleza | `matrizbinicial.cm4` | 29/12/2003 | 87 | 12 → 20 |
| B · mesma carreira após evolução no jogo | `matrizbavancado.cm4` | 15/01/2004 | 123 | 12 → 20 |

Na regressão do núcleo Rust da alpha.4, uma única transação por checkpoint editou Passe e salário semanal de Clodoaldo, treino de jogadores de linha de um membro da comissão e saldo de um clube com registro financeiro completo. As quatro cópias foram relidas em consultas separadas pelo CLI.

| Checkpoint | Salário semanal (£) | Treino de linha | Balanço após edição (£) | Bytes alterados |
|---|---:|---|---:|---:|
| A inicial | 1.490 → 1.590 | Rui Águas, 12 → 13 | Braga, 625.326 | 7 |
| A após um mês | 1.490 → 1.590 | Rui Águas, 12 → 13 | Braga, 1.564.831 | 7 |
| B inicial | 1.651 → 1.751 | Marcelo Pantera, 11 → 12 | Fortaleza, 2.969.489 | 6 |
| B após evolução | 1.651 → 1.751 | Marcelo Pantera, 11 → 12 | Fortaleza, 2.816.969 | 6 |

O Passe foi 20 nas quatro cópias. Uma comparação binária independente confirmou a contagem de bytes alterados de cada relatório, sem mudança de tamanho. Os hashes dos originais permaneceram iguais e os hashes das cópias coincidiram com os relatórios.

O `0.9.0-beta.4` repetiu a transação de quatro campos em todos os checkpoints. A releitura confirmou os mesmos valores e as contagens de 7, 7, 6 e 6 bytes alterados. Apenas `game_db.dat` mudou nas quatro cópias, e os hashes das origens permaneceram iguais. Na carreira B após evolução, o SHA-256 da cópia foi `fc84040974e05b55a64664e33fbbfb9b106448d039c090a6d913402487c37738`, idêntico ao ensaio nativo abaixo. A regressão automatizada reproduz os quatro SHA-256 e compara o conteúdo das origens. [Teste de transações](crates/cm0304-core/tests/transactions.rs)

No checkpoint B após evolução, o Tauri nativo preparou os quatro campos nos formulários de jogador, staff e clube. A revisão reuniu os quatro campos e gravou `mistotauri.cm4`. A cópia teve SHA-256 `FC84040974E05B55A64664E33FBBFB9B106448D039C090A6D913402487C37738`, igual ao da cópia do CLI. O hash da origem permaneceu igual.

O CM0304 4.1.5 abriu uma cópia idêntica, `0mistotauri.cm4`, em 15/01/2004 às 14h. O jogo mostrou Passe 20 e salário de £7.000 por mês na ficha de Clodoaldo, treino de jogadores de campo 12 na ficha de Marcelo Pantera e balanço de £2.816.969 nas finanças do Fortaleza. Em 10/02/2004 às 21h15, o jogo gravou `x.cm4`. A releitura Rust confirmou Passe 20 e treino de linha 12, além do saldo atualizado de £2.426.988. O vínculo de Clodoaldo mudou durante a evolução. O novo contrato registrou £1.651 por semana, início em 02/01/2004 e vencimento em 31/01/2007. SHA-256 da regravação `FAD6FA1B1053E1310F6971019184DEF232FC4958D0BC79BD39CBBBB1901C79DE`. Os hashes da origem e da variante carregada permaneceram iguais. Após reiniciar o CM0304, Retomar Último Jogo carregou x.cm4 na mesma data e hora. A tela confirmou Passe 20, treino de linha 12, salário do novo contrato de £6.500 por mês e balanço de £2.426.988.

Cada teste de Passe mudou um byte e preservou o SHA-256 da origem. Nas duas amostras da carreira B, uma transação mista de Clodoaldo também confirmou a gravação e releitura de Passe 20, Moral 18, £1.800 por semana e vencimento em 30/06/2006. Cada cópia mudou seis bytes e preservou o hash da origem. O CM0304 carregou `matrizbavmisto.cm4` em 15/01/2004 e exibiu Passe 20, moral Muito Bom, £7.250 por mês e vencimento em 30/06/2006.

Após avançar de 15/01 a 22/01/2004, o CM0304 gravou `matrizbmistor.cm4` e recarregou a cópia depois de reiniciar. Passe permaneceu em 20. A moral passou de 18 para 17 e continuou como Muito Bom na tela. Antes do avanço, o perfil indicava uma proposta do Fortaleza em avaliação. Depois, o contrato exibido tinha nova data de início (02/01/2004), vencimento em 31/01/2007 e salário de £6.500/mês (£1.651/semana na releitura Rust). Isso demonstra que o vínculo mudou durante a simulação. SHA-256 da regravação: `57EA47D52E9439594E501B44BDE0BBB57E36BC777B1BE334D15640161AC9CE8D`. Os hashes da origem e da cópia editada permaneceram iguais.

Em 22/01/2004, o perfil de Clodoaldo já exibia “Propostas: Nada”. A partir de `matrizbmistor.cm4`, o Rust alterou três bytes em `matrizbcontrato.cm4` para ajustar o salário a £2.000/semana e o vencimento a 30/06/2007. O CM0304 mostrou £8.000/mês e 30/06/2007 ao carregar a cópia. Após sete dias, o jogo gravou `matrizbcontrato7d.cm4` em 29/01/2004. O Rust releu os dois alvos e a data de início original de 02/01/2004. Após reiniciar, o jogo recarregou a regravação e exibiu os mesmos valores. Os hashes da origem e da cópia editada permaneceram iguais. SHA-256 da regravação: `779386466BD5796DCC2D68FEC361887F9E5B0F5CF2034EA5841B433CF916FD8C`.

Para cada operação e variante suportada:

1. Registrar hash, versão e entidades da origem.
2. Conferir a prévia e aplicar alterações conhecidas em cópia.
3. Reler e conferir campos, referências e todas as diferenças binárias previstas.
4. Carregar no jogo e conferir em telas adequadas, sem tratar valores ocultos/estimados como igualdade exata.
5. Avançar pelo menos sete dias; testar também o evento/data relevante para contratos, lesões e suspensões.
6. Registrar recálculos, salvar em outro arquivo e carregar novamente.
7. Confirmar integridade da origem e registrar resultados na matriz.

Evolução normal não é automaticamente falha. Distinguir recálculos de treino, CA e passagem do tempo de gravações incorretas. Detectar e recusar variantes e compressão não suportadas.

## Depois da v1.0

O próximo ciclo melhora os dois usos do aplicativo. A pessoa pode abrir um save só para pesquisar e comparar ou partir dos mesmos resultados para preparar uma edição. Busca, seleção e navegação entre fichas atendem aos dois percursos.

### v1.1 — Melhorias no fluxo de edição

| Prioridade | Melhoria | Como verificar |
|---|---|---|
| 1 | **Continuar de onde parou.** Depois de adicionar uma edição à revisão, manter a aba e a posição na ficha. Ao voltar à lista, retomar no item selecionado com a mesma busca, filtros e página. | Editar o salário de um vínculo, continuar na aba Contratos e voltar à mesma lista para editar outro jogador. Repetir o percurso em Staff e Clubes, inclusive em janela compacta e pelo teclado. |
| 2 | **Enxergar a edição na própria ficha.** Destacar os campos alterados e indicar quando o valor está no rascunho ou já entrou na revisão. Deixar o valor do save acessível junto ao campo e manter o acesso a **Ir ao campo** quando houver erro. | Alterar Passe e salário em abas diferentes, adicionar ambos à revisão e reabrir a ficha. Identificar os dois campos e conferir seus valores de origem e propostos. |
| 3 | **Corrigir sem refazer o caminho.** Na revisão, localizar uma alteração por nome ou campo, conferir o vínculo e voltar direto ao campo que precisa de ajuste. Mostrar quantas alterações o filtro está exibindo. | Num plano misto, achar um salário, corrigir esse valor e voltar à revisão no mesmo filtro. Conferir os efeitos derivados e desfazer/refazer sem perder as demais alterações. |
| 4 | **Ajustar a seleção do lote na prévia.** Facilitar a correção de uma linha ou a exclusão de um jogador sem reconstruir a seleção nem reaplicar o preset. Manter visíveis as linhas que exigem atenção. | Aplicar o preset **Elenco · Recuperação**, retirar um jogador e gerar outra prévia com os mesmos ajustes. Confirmar na revisão quem entrou no lote e desfazer o lote inteiro. |
| 5 | **Encontrar a cópia gravada.** No resultado da gravação, mostrar o nome e o caminho da cópia e oferecer acesso à pasta de destino e ao histórico da sessão. | Gravar duas variantes com nomes diferentes e localizar a última cópia para abrir no CM0304. Conferir que a origem conserva o SHA-256 e que o destino existente continua protegido. |

**Presets de lote.** A prévia está acessível pela ficha do jogador, pelos resultados filtrados e pelas ferramentas do elenco. Inclui os presets **Elenco · Recuperação**, **Moral**, **Consistência**, **Força e velocidade**, **Propensão a lesões** e **Dois pés**, além dos presets salvos na instalação. A ficha individual pré-seleciona o jogador aberto. Os lotes de busca e de clube usam a seleção explícita e entram na revisão como uma operação.

#### Ampliações em avaliação

- **Na ficha do clube.** Oferecer **Abrir elenco em Jogadores** e **Abrir comissão em Staff**, aproveitando a composição do elenco e as telas de busca existentes. Conferir os alvos pelo vínculo, inclusive quando o jogador estiver emprestado. Na volta, manter a ficha do clube aberta no mesmo ponto.
- **Na ficha do jogador e no elenco.** Oferecer **Elevar moral**, **Descansar**, **Definir condição** e **Remover lesões** onde a operação já é suportada. Manter condição, ritmo e cansaço explícitos na prévia. Para testar cenários, permitir baixar a moral ou aumentar o cansaço pelos mesmos controles. Conferir que o lote respeita o recorte escolhido e que um jogador retirado fica de fora.
- **Contratos e habilidade.** O lote atual usa data fixa. Uma ação **Estender vencimento** exige calcular a partir da data de cada vínculo, tratar contratos mensais e validar datas como 29 de fevereiro. Mudanças em CA exigem política explícita e alvos representáveis. Alterações de PA em lote devem respeitar os valores aceitos pelo núcleo.
- **Pesquisa de novas operações.** Remover uma queixa específica do jogador, encerrar um contrato, criar jogadores e alterar preferências ou inscrições exigem validação própria. Para cenários financeiros, usar ajustes explícitos de saldo e orçamentos já suportados, com os valores na revisão.

Usar três tarefas para medir quantas ações a v1.0 exige e quantas vezes é preciso voltar à mesma aba. Editar o mesmo grupo de campos em dois jogadores, corrigir um contrato a partir da revisão e ajustar uma seleção de lote. Repetir as tarefas após as mudanças, com teclado e janela compacta. Usar a regressão com IPC simulado para a navegação e o Tauri nativo para conferir a gravação e a localização da cópia.

### Consulta e observação

- **Mapa do elenco.** A aba **Análise** abre com a tática e os titulares gravados para a equipe principal do técnico humano, quando reconhecidos. A simulação de ausência ou fim de empréstimo abre a vaga afetada e mantém os outros titulares nas suas posições. As formações manuais oferecem uma distribuição sugerida. [Guia de uso](README.md#usar-o-aplicativo-no-windows) · [Leitura da tática](research/FORMAT.md#tática-ativa-do-técnico-humano)

A v1.0 já oferece busca com filtros, buscas salvas, listas de observação, comparação e exportação CSV/JSON. O próximo passo é ligar esses recursos em um percurso de consulta que funcione de ponta a ponta e compartilhe as fichas do editor.

| Prioridade | Melhoria | Como verificar |
|---|---|---|
| 1 | **Pesquisa por perfil.** Combinar vários critérios de atributo na mesma busca e abrir a comparação ou adicionar jogadores à observação a partir dos resultados. Preservar a busca, os filtros e a seleção ao voltar. | Buscar laterais canhotos com mínimos de Aceleração e Cruzamentos, comparar dois resultados e guardar os escolhidos na lista de observação. Ao voltar, retomar a mesma busca com a revisão vazia. |
| 2 | **Scout Rating por posição.** Exibir as sete notas na ficha. Após validar a fórmula e a ordenação, disponibilizar uma coluna e ordenar pela nota da posição escolhida. Na edição, mostrar o percentual da origem e o percentual com as mudanças da revisão. | Conferir as notas de goleiros e jogadores de linha com o CM Scout 3.10 no mesmo save, à precisão exibida de duas casas. Testar a ordem dos resultados e verificar que desfazer uma edição restaura a nota anterior. [Fórmulas e referência](research/FORMAT.md#referência) |
| 3 | **Preferências e rivalidades para consulta.** Mostrar nas fichas de Jogadores, Staff e Clubes as relações já reconhecidas, com acesso à ficha de cada alvo. | Percorrer relações a partir de Totti e do Manchester United e conferir as listas com as amostras lidas no diagnóstico. [Preferências de pessoas](research/FORMAT.md#preferências-de-pessoas-e-clubes) · [Preferências e rivalidades do clube](research/FORMAT.md#preferências-e-rivalidades-na-ficha-do-clube) |
| 4 | **Listas de observação no jogo.** Investigar a importação e a exportação no formato de shortlist do CM0304, a partir das listas já existentes no editor. | Confirmar no CM0304 4.1.5 que o jogo carrega uma lista exportada e que o editor importa uma lista criada pelo jogo com os mesmos jogadores. |

### v1.0.x — Estabilidade em outras carreiras

- Executar o roteiro da v1.0 em uma carreira independente, com ligas ou fase da temporada diferentes das amostras locais. Se houver outro Windows x64 disponível, aproveitar para testar a instalação limpa. [Roteiro de teste](docs/DEVELOPMENT.md#beta-em-outra-carreira) · [Instalação em Windows limpo](docs/DEVELOPMENT.md#verificação-opcional-em-windows-limpo)
- Registrar versão do editor, etapa, resultado esperado, erro e hashes relevantes em relatos que possam ser compartilhados sem anexar a carreira. Priorizar falhas que afetem a integridade da origem, o vínculo editado ou o carregamento da cópia no jogo.
- Publicar correções de problemas reproduzíveis em versões 1.0.x. Para correções na escrita, reproduzir com uma regressão, conferir a origem por SHA-256, validar a cópia no editor e completar o ciclo afetado no CM0304.

### Pesquisa posterior — Transferência definitiva sem taxa

1. **Primeiro ensaio.** Em uma carreira com inscrições fechadas, usar uma transferência concluída pelo próprio CM0304 como controle para entender a saída de um jogador já inscrito. Comparar com uma variante de laboratório e conferir os dois elencos, o contrato, as inscrições dos demais jogadores e a elegibilidade na partida relevante. Avançar, regravar e recarregar no jogo. A chegada de Nem ao Porto após o prazo já passou por esse ciclo. [Evidências de inscrição](research/FORMAT.md#bloco-squad_mandat)
2. **Decisão de escopo.** Registrar quais referências o jogo atualiza, quais variantes podem ser reconhecidas com segurança e quais exigem recusa. Enquanto o tratamento das referências de saída estiver em aberto, a transferência segue na CLI de laboratório.
3. **Integração após a prova.** Para jogadores sem empréstimo ativo e nos vínculos validados, oferecer no Tauri uma prévia de origem, destino, contrato, inscrição e impacto projetado na folha dos dois clubes. Integrar à revisão, a desfazer/refazer e à gravação em cópia. Distinguir a folha registrada no save da projeção dos contratos. Confirmar o fluxo nativo em duas carreiras, incluindo uma saída com a lista fechada e uma recusa que preserve a origem e a revisão.

### Outras ideias para depois

- **Editar preferências e rivalidades.** Para liberar a escrita, validar em cópia de laboratório a troca de um alvo existente em cada tipo de ficha, a preservação dos demais registros e o ciclo de abertura, avanço, regravação e recarga no CM0304. [Consulta de preferências e rivalidades](#consulta-e-observação)
- Reavaliar planejamento e evolução do elenco, notas nas listas de observação, conferência de hashes no histórico e diagnóstico para suporte a partir das dificuldades observadas no uso da v1.1.
- Priorizar, conforme os casos encontrados, contratos ativos com seleções, estados adicionais de recuperação de lesões, saldo financeiro básico e edição de estádios compartilhados. A edição de cada variante depende de evidência no jogo. [Matriz de carreira](reports/v0.5-matrix.md) · [Decisões de escopo](#decisões-de-escopo-das-ampliações-para-a-v10)
- Investigar transferências com taxa, novos empréstimos, rescisões e criação de contratos após a prova da transferência sem taxa. Essas operações exigem sincronizar elenco, propriedade, inscrições e referências.
- Criar/remover jogadores, renomear entidades e alterar estruturas de comprimento variável.
- Adicionar/remover nacionalidades e alterar elegibilidade além do campo principal validado.
- Importação CSV para escrita e automações com regras personalizadas.
- Competições, calendários, regras, resultados e ligas.
- Outros patches, outras edições, edição em memória, macOS/Linux e nuvem.

## Decisões de escopo das ampliações para a v1.0

| Ampliação | Evidência atual | Decisão para a v1.0 |
|---|---|---|
| Transferência definitiva | Protótipo com elenco, vínculo e partida em duas carreiras. O histórico sem taxa de Angelim persistiu e a transferência nativa para o Santos confirmou que a folha registrada pode ficar defasada. No Porto, o jogo preservou os 25 registros da Liga dos Campeões e recusou um jogador fora da lista na escalação contra o Celta. O protótipo recusa realocar um jogador com referência ao clube atual. [Formato](research/FORMAT.md#bloco-squad_mandat) | Manter a transferência na CLI de laboratório, conforme o escopo base. A chegada de Nem ao Porto após o prazo passou pelo ciclo completo e o jogo o manteve fora dos inscritos contra o Celta. Para promover a operação ao Tauri, validar o tratamento dos registros na saída de um jogador já inscrito. |
| Bônus e cláusula | Os ensaios de presença, gol e assistência observaram o evento e o aumento correspondente na despesa agregada. A cláusula existente foi validada com a oferta de £100.000 do Santos por Angelim, aceita na variante de £10.000 e recusada no controle. A edição persistiu após gravação pelo Tauri e pelo CM0304. [Matriz](reports/v0.5-matrix.md) | Incluir na v1.0 a edição dos quatro campos em vínculos que já os contêm, nas variantes cobertas pela matriz. |
| Gestão de elenco | Recuperação e situação no elenco passaram pela gravação nativa, avanço e recarga. A exclusão individual de um vínculo na prévia foi confirmada após recarga. [Matriz](reports/v0.5-matrix.md) | Incluída no escopo da v1.0 para as variantes de vínculo, lesão e suspensão cobertas pela matriz. |
| Lote de comissão | Dois alvos conferidos no jogo em cópia da CLI, com preservação da dupla função. O Tauri demonstrou prévia, desfazer e gravação. O jogo abriu a variante do Tauri e mostrou os dois alvos. A variante idêntica da CLI passou por avanço e recarga. [Validação](#progresso-de-implementação) | Incluído na v1.0 para os atributos profissionais aceitos na prévia, com seleção por membro, presets e revisão transacional. |
| Situação de mercado | A alpha.14 permite revisar e editar transições mapeadas nos vínculos elegíveis. Quim e Clodoaldo foram usados para conferir os estados 4, 6, 7 e 68 ao longo da simulação. Quínio (Braga B) passou pelo ciclo Tauri → jogo → avanço → regravação → recarga. As quatro propostas existentes continuaram visíveis na abertura, e a transferência para o Loures foi concluída em 29/07. Em uma carreira brasileira, a transição 68 → 69 de Michel (Fortaleza) também passou por carregamento, avanço, regravação e recarga. [Evidências e hashes](research/FORMAT.md#situação-de-transferência-e-empréstimo) | Manter as seis transições mapeadas no escopo da v1.0. Todas passaram pelo ciclo local no CM0304. O ciclo local no CM0304 cobre o aceite da v1.0. [Matriz por transição](research/FORMAT.md#cobertura-do-sintetizador-atual) |
| Numeração por vínculo | O Tauri gravou a troca de Quim e Fredrik, confirmada em partida e na recarga. Na Liga dos Campeões, Vítor Baía atuou com a 30 e Mário Silva continuou inelegível com a 99. Os números persistiram após regravação e recarga. [Matriz](reports/v0.5-matrix.md) | Incluir a troca de camisas por vínculo nas variantes validadas, com detecção de duplicidade no clube e preservação dos registros de inscrição. |
| Lotação do estádio | O jogo confirmou capacidade total e lugares sentados em estádio de uso exclusivo. O Tauri produziu uma variante idêntica. Três saves reais confirmaram a recusa de edição em estádio compartilhado e a interface o exibiu para consulta. [Validação](reports/v0.6-club-finance-native.md#lotação-do-estádio) | Aprovada para estádios de uso exclusivo. Braga–Marítimo teve 14.471 espectadores no controle de 28.000 lugares e 963 na variante de 1.000. A capacidade editada persistiu após regravação e recarga. Estádios compartilhados ficam disponíveis para consulta na v1.0. A edição compartilhada segue em investigação. |

## Critérios de lançamento

Os hashes locais de `0aceitetauri.cm4`, `t.cm4`, `0aceite.cm4` e `a.cm4` conferem com os ensaios de aceite. A cópia atual de `x.cm4` tem SHA-256 `94513ac9af44dbc1e017ecdfb447c20b881168472fcf76c28f983c4d892764a1`, diferente do registrado no ensaio misto. O leitor atual ainda confirma Passe 20 de Clodoaldo, treino de linha 12 de Marcelo Pantera e saldo de £2.426.988 do Fortaleza. O relato do ensaio preserva o hash que foi observado naquela execução.

O `0.9.0-beta.17` mostra no rodapé a versão retornada pelo Tauri. O executável extraído exibiu `0.9.0-beta.17` e repetiu o lote do Braga com a mesma cópia por SHA-256. O roteiro empacotado inclui retomada de plano e responsividade. O ZIP e o NSIS passaram no 7-Zip, e o executável extraído coincide com o build. SHA-256 do ZIP `8846dcdf2d5b5aba084ff8bbbad2bd9839cfc8778d4b64fa68c562174eee3d81`. O instalador está sem assinatura Authenticode.

A regressão do beta.17 aprovou 94 testes Rust em release, 12 testes JavaScript, 34 cenários de interface com IPC simulado, formatação e Clippy. A auditoria confirmou a licença MIT, os avisos gerados a partir do `Cargo.lock`, o checksum do ZIP e os sete arquivos do NSIS. SHA-256 do instalador `715d17336869b1691de3f2fc10b9f56a42ee396492be17a84bf6d16cb8c98e46`.

A instalação do beta.17 reutilizou a pasta isolada dos ensaios anteriores e registrou a versão correta. O executável instalado coincidiu com o extraído do NSIS e mostrou a versão no rodapé. A edição em lote do Braga gerou a cópia de referência. Após a desinstalação, o executável foi removido e os hashes da origem e da cópia permaneceram iguais.

O `0.9.0-beta.16` indexa jogadores e referências compartilhadas durante a validação de alterações. No CLI, o plano de 203.108 mudanças de personalidade caiu de 23,90 s para 1,82 s em uma execução por build, com cópia e relatório idênticos. No executável extraído do NSIS, o Tauri retomou as 203.108 alterações do plano exportado pelo beta.15 e preservou o hash da origem. Passaram 92 testes Rust em release. O 7-Zip conferiu o ZIP e o NSIS, e o executável extraído coincidiu com o build. SHA-256 do ZIP `28a95214b1a08ef273142540d1945150dff9034c26ce137b77bf0e007d4dee71`. O instalador está sem assinatura Authenticode. [Medições](reports/v0.8-performance.md)

A instalação silenciosa do beta.16 reutilizou a pasta isolada do teste anterior e registrou a nova versão. O binário instalado coincidiu com o extraído do NSIS. O editor instalado comparou a comissão do Braga e gravou o mesmo lote nativo de Passe, Técnica e Moral, com SHA-256 `41eed3d61c63361e58647d41134a38687ff1ac6b176d2fe686d964cdf99d34dc`. A desinstalação removeu o binário e preservou os hashes do save aberto, da cópia e do controle de personalidade.

No WebView2 do beta.16, Alt+R abriu a revisão, Ctrl+Z retirou a edição de Passe de Clodoaldo, Ctrl+Y restaurou o alvo 20 e Ctrl+Shift+S abriu o diálogo nativo para salvar a cópia. O arquivo `atalhos.cm4` mudou um byte, com SHA-256 `975b316edd6527e947115e8614ebc6ba9570869a90850c01f3109583a9e63c75`. O hash da origem permaneceu igual.

No diálogo nativo do beta.16, o editor recusou a retomada do plano compacto de 203.108 alterações da carreira brasileira com o save inicial do Braga aberto. A mensagem indicou que o plano pertencia a outro save, a revisão permaneceu vazia e o save aberto conservou o SHA-256 original.

No beta.16, o botão de cancelamento foi acionado via WebView2 enquanto o seletor nativo do segundo save estava aberto. Após selecionar o save avançado, o editor confirmou o cancelamento, manteve a revisão vazia e preservou o hash da origem. A segunda tentativa na mesma sessão abriu a comparação de Clodoaldo normalmente.

O beta.16 recusou pelo diálogo nativo uma cópia de laboratório com versão interna 148 de `game_db.dat`. A mensagem indicou a versão esperada 147. O save atual continuou aberto com Passe 20 na revisão e Passe 19 no formulário, e a origem conservou seu SHA-256.

A suíte Rust do beta.16 passou com 94 testes em release após a inclusão das regressões de cancelamento de comparação e de proteção de referências compartilhadas. A formatação e o Clippy passaram.

O `0.9.0-beta.15` usa JSON compacto quando a formatação do plano de sessão excede 16 MB. Os três testes Rust de planos passaram, inclusive a preservação de 200.000 alterações após decodificação e a recusa quando nem o JSON compacto cabe. O executável extraído repetiu o teste nativo de comparação da comissão e gravação em lote do Braga, com os mesmos hashes do beta.14. O 7-Zip conferiu o ZIP e o NSIS, e o executável extraído coincidiu com o build de release. SHA-256 do ZIP `9ac2ca46de0001005bee4beb1f3736d4356872663d2d1271de1417fca5077171`. O instalador está sem assinatura Authenticode.

No beta.15, passaram 92 testes Rust em release, incluindo a gravação de transações mistas nos quatro checkpoints de referência, 12 testes JavaScript e 34 cenários de interface com IPC simulado. Formatação e Clippy também passaram. A licença MIT coincidiu com a do repositório, os avisos de terceiros coincidiram com a geração a partir do `Cargo.lock` e o NSIS continha somente o executável e seis recursos de instalação.

No diálogo nativo do beta.15 extraído, o plano de 203.108 alterações da carreira brasileira foi exportado em 13.015.927 bytes. Com indentação de dois espaços, o mesmo conteúdo teria 21.952.772 bytes. Um novo processo do editor importou o JSON pelo diálogo nativo, restaurou as 203.108 alterações na revisão e preservou o SHA-256 do save de origem.

O `0.9.0-beta.14` aplica o limite de 16 MB tanto ao guardar quanto ao retomar um plano de sessão. A regressão Rust em release, os 12 testes JavaScript e os 34 cenários de interface com IPC simulado passaram. No WebView2 do executável extraído, a comparação da comissão do Braga e a gravação de Passe, Técnica e Moral em lote produziram a mesma cópia do beta.13, com a origem intacta. O 7-Zip confirmou a integridade do ZIP e do NSIS, e o executável extraído e a licença MIT coincidiram com os arquivos de referência. SHA-256 do ZIP `8baedc1fe4fecbf9e46e3f26161b7dfcb78611dccd5c2d5914a8a312aa317afc`. O instalador está sem assinatura Authenticode.

No executável do beta.14, uma cópia da carreira brasileira recebeu 12 ajustes em lote sobre 25.552 jogadores, com 304.826 alterações pendentes. Ao selecionar Guardar plano, o editor recusou a exportação acima de 16 MB, manteve a revisão e preservou o SHA-256 do save de origem.

A auditoria do beta.14 confirmou os avisos de terceiros contra a geração com o `Cargo.lock` e os textos suplementares. O NSIS contém o executável do editor e seis recursos de instalação. A paridade Rust com o leitor de referência confirmou todos os campos históricos comparados para 22.062 jogadores em cada save fornecido e todos os 87 e 267 blocos, respectivamente. [Paridade](reports/rust-parity.json)

O `0.9.0-beta.13` pagina a revisão e os relatórios arquivados em 50 entidades. No WebView2 do executável extraído, o lote de 22.062 jogadores abriu a revisão em 1.335 ms e confirmou a gravação em 1.382 ms. A releitura e a comparação binária confirmaram somente 22.044 bytes de moral alterados, com SHA-256 da cópia idêntico ao ensaio do beta.12. O ZIP tem SHA-256 `8ab7c3eb12508ad2415927858c45d85ff8820179fe9fcb7111d87638dd4e3c31`. O NSIS passou na conferência de integridade e está sem assinatura Authenticode. [Medições](reports/v0.8-performance.md)

No mesmo beta, a instalação isolada produziu um executável idêntico ao contido no NSIS. A desinstalação removeu o binário e preservou o save de teste criado pelo CM0304 após o lote de Passe, Técnica e Moral, SHA-256 `66daa25a11edcd16971fe4c670882dd545e3ab8d56bb489b11bf94e6ceac726e`.

O `0.9.0-beta.12` pagina a prévia de lotes em 50 linhas. O executável extraído do NSIS exibiu a prévia de 22.062 jogadores no WebView2 em 1.436 ms nessa execução, com os mesmos 22.044 jogadores com alterações e o SHA-256 do save preservado. Passaram 89 testes Rust em release, 12 testes JavaScript e 34 cenários de interface com IPC simulado. Instalação e desinstalação isoladas preservaram o save de teste. O ZIP (SHA-256 `3b372eb8ddf865a4efefb5ec476f9e554602c82f0799ff089919372eaebb2687`) passou pela conferência de integridade, executável, licença MIT e avisos de terceiros. O instalador está sem assinatura Authenticode. [Medições](reports/v0.8-performance.md)

O `0.9.0-beta.11` corrige o descarte de rascunhos ao abrir a revisão. No WebView2 do executável extraído do NSIS, o teste confirmou que uma abertura inválida preserva a sessão e que, depois de descartar explicitamente o rascunho, a cópia grava apenas os valores presentes na revisão. O ZIP tem SHA-256 `bfd6d5bf5d47ee71f1e3fc241b720ebb7aa80e0e5aa20f92bb78f2707bfadfcf`. A auditoria conferiu a integridade, a licença MIT e a correspondência do executável com o build de release. O instalador está sem assinatura Authenticode. [Evidência nativa](docs/DEVELOPMENT.md#evidências-locais-do-beta)

Passaram 89 testes Rust em release, 12 testes JavaScript e 34 cenários de interface com IPC simulado no Chrome. A instalação e a desinstalação isoladas do beta.11 preservaram o SHA-256 do save de teste. Os avisos do ZIP coincidiram com a geração a partir do `Cargo.lock` e dos textos suplementares.

O `0.9.0-beta.10` aceita duas casas decimais nos ajustes em pontos de condição e ritmo, inclusive em presets. A suíte de interface confirmou que o preset de condição a 70,01% produz o inteiro 7001 no IPC e que 70,011% é recusado. Passaram 34 cenários com IPC simulado no Chrome. A auditoria confirmou a integridade do ZIP e do NSIS, a licença MIT e a correspondência entre o executável extraído e o build de release. A instalação e a desinstalação isoladas no Windows de desenvolvimento preservaram o save de teste. O ZIP tem SHA-256 `ba5b413f3243c33176d96e5c983e4ad963cf6a9a52534c12a836f2733d1214be`. O instalador está sem assinatura Authenticode.

No beta.9, a instalação silenciosa isolada e a desinstalação terminaram com código zero no Windows de desenvolvimento. O executável instalado tinha versão `0.9.0-beta.9` e o mesmo SHA-256 do build de release. A desinstalação removeu o executável e preservou a cópia de teste `ensaio.cm4` (SHA-256 `9014002d73d5db1e3861e141f1746c6053f05ddd0b70dbc1ff1a013851e6f5a8`). A auditoria do ZIP também conferiu a licença MIT, os avisos gerados pelo `Cargo.lock` e o roteiro incluído. A paridade confirmou os campos históricos de 22.062 jogadores em cada amostra e os 87 e 267 blocos comparados. [Paridade](reports/rust-parity.json)

A instalação silenciosa do beta.8 em uma pasta isolada no Windows de desenvolvimento terminou com código zero. O executável instalado tinha versão `0.9.0-beta.8` e SHA-256 idêntico ao binário contido no NSIS. A desinstalação removeu o executável e preservou por SHA-256 a cópia de teste colocada na mesma pasta.

Na CLI de release do beta.8, uma operação desconhecida acrescentada ao plano de aceite fez a transação inteira ser recusada. O mesmo plano válido também foi recusado quando aplicado a uma cópia reduzida aos primeiros 8.192 bytes do save inicial. Os dois comandos retornaram erro, nenhum destino foi criado e o SHA-256 do save inicial permaneceu igual.

O executável extraído do NSIS do beta.8 tem versão de produto e de arquivo `0.9.0-beta.8` e SHA-256 `82f1a762cd12f928c70aefe1a9c74fc134038ce32eea445f3a8ad829a94039cc`, idêntico ao binário de release. Rodando o arquivo extraído no Windows de desenvolvimento, o Tauri abriu `save_inicial.cm4` com 22.062 jogadores, exibiu a ficha de Quínio e fechou normalmente. Esse ensaio executou o binário empacotado diretamente.

O `0.9.0-beta.8` integra a leitura estrutural dos registros individuais de inscrição e as regressões de inscrição e suspensão. A CLI de laboratório protege a transferência quando há referência de elenco ao clube atual. O pacote x64 inclui o roteiro para carreira independente. O 7-Zip confirmou a integridade do ZIP e do NSIS, que contém o executável do editor e seis recursos de instalação. O instalador extraído coincide por SHA-256 com o build, a licença MIT coincide com a do repositório e os avisos de terceiros coincidem com a geração a partir do `Cargo.lock` e dos textos suplementares. O instalador do beta está sem assinatura Authenticode e o roteiro informa esse estado. Passaram 88 testes Rust em release, 12 testes JavaScript e 34 cenários de interface com IPC simulado no Chrome.

A repetição de `npm run verify:parity` no beta.8 confirmou os campos históricos dos 22.062 jogadores de cada amostra e os metadados e hashes de todos os blocos comparados, 87 no save inicial e 267 no save de um mês. [Resultado da paridade](reports/rust-parity.json)

O `0.9.0-beta.7` passou 86 testes Rust em release, 12 testes JavaScript e 34 cenários de interface com IPC simulado. O 7-Zip validou o ZIP e o NSIS, e a instalação isolada no Windows de desenvolvimento produziu o executável esperado. No WebView2 real, o botão da barra de título foi bloqueado durante uma gravação ativa, que terminou com a cópia íntegra. A abertura dos dois saves fornecidos teve mediana próxima de 500 ms e a busca até a linha aparecer na tela, 200 ms. [Pacote e ensaios nativos](docs/DEVELOPMENT.md#procedência-e-licenças-do-pacote) · [Desempenho](reports/v0.8-performance.md#webview2-no-beta7)

O `0.9.0-beta.6` passou 85 testes Rust em release e 34 cenários de interface com IPC simulado. A substituição de blocos cobre remoções de saúde e edições de mercado. A comparação em uma amostra de laboratório de 236 MB confirmou resultados byte a byte idênticos ao build anterior. [Desempenho](reports/v0.8-performance.md#repetição-na-alpha29) · [Pacote](docs/DEVELOPMENT.md#procedência-e-licenças-do-pacote)

As CLIs beta.6, beta.7 e beta.8 reproduziram por SHA-256 a variante de aceite `0aceite.cm4`, antes carregada e regravada pelo CM0304 4.1.5. A transação alterou 22 bytes e removeu 38, com a origem intacta. A releitura do arquivo regravado e recarregado anteriormente no jogo, `a.cm4`, confirmou condição de Quim em 100%, ausência da lesão de Hoekstra, empréstimo de Liam Chilvers a £571 até 29/02/2004, Adaptabilidade 20 de Rui Águas e os dois orçamentos do Braga. [Ensaio de aceite](#escopo-de-produto-para-a-v10)

Na base do `0.9.0-beta.4`, passaram 84 testes Rust em release, incluindo a nova regressão com quatro checkpoints, e 34 cenários de interface com IPC simulado. A instalação isolada, o ensaio de rascunho e a rejeição de um save incompatível foram conferidos no Windows de desenvolvimento. Os avisos de terceiros do ZIP coincidiram com a geração pelo `Cargo.lock`, e a licença MIT com a do repositório. [Validação do beta.4](docs/DEVELOPMENT.md#procedência-e-licenças-do-pacote)

O `0.9.0-beta.1` passou 83 testes Rust, 12 testes JavaScript e 34 cenários de interface com IPC simulado. O instalador produziu o executável de release numa pasta isolada do Windows de desenvolvimento, e a desinstalação preservou o save de teste. A auditoria conferiu licença, avisos e conteúdo do pacote. [Auditoria do beta](docs/DEVELOPMENT.md#procedência-e-licenças-do-pacote)

A alpha.29 conferiu, com WebView2 e IPC nativos, a seleção de prazos de 30 e 60 dias, a abertura do vínculo exato e a projeção do subtotal e da folha do Braga antes de gravar. A cópia do diálogo nativo mudou um byte no salário do empréstimo de Fredrik e preservou o contrato dele com o Porto. [Ensaio do painel de contratos](docs/DEVELOPMENT.md#beta-em-outra-carreira)

A regressão local da alpha.29 aprovou 82 testes Rust em release no workspace, 12 testes JavaScript e 34 cenários de interface no Chrome com IPC simulado. A inspeção do ZIP e do NSIS conferiu os hashes, a licença MIT, os avisos gerados pelo `Cargo.lock` e o conteúdo distribuído. Naquele marco, a confirmação do bloqueio da barra de título e os dois testes externos ainda estavam pendentes. [Auditoria do pacote](docs/DEVELOPMENT.md#procedência-e-licenças-do-pacote)

Na alpha.27, `npm run verify:parity` confirmou a paridade dos campos históricos de 22.062 jogadores em cada uma das duas amostras e de todos os blocos comparados (87 no save inicial e 267 no save de um mês). A comparação binária do ensaio no WebView2 confirmou que Cruzamentos 11 → 20 de Clodoaldo alterou um único byte no save inicial, no offset 20622025. A origem permaneceu intacta. [Paridade das amostras](reports/rust-parity.json) · [Ensaio nativo](docs/DEVELOPMENT.md#instalador-windows-para-testes-da-v08)

O `0.9.0-beta.9` atualiza o roteiro de carreira independente com a conferência de suspensões após uma partida. A regressão local aprovou 89 testes Rust em release, 12 testes JavaScript e 34 cenários de interface com IPC simulado no Chrome. O 7-Zip validou a integridade do ZIP e do NSIS. O executável extraído do instalador coincide por SHA-256 com o build de release. O ZIP tem SHA-256 `b3eec74c2525bc29315bddfca894a41c09091cb730f83c2d586ecf03dfb41c18`. [Roteiro](docs/DEVELOPMENT.md#beta-em-outra-carreira) · [Ensaio no CM0304](reports/v0.5-suspensions.md)

- [x] Aplicativo Rust + Tauri sem terminal ou toolchain de desenvolvimento para o usuário final. O executável instalado do beta.7 abriu, gravou e recarregou uma cópia com o PATH limitado ao Windows. [Ensaio de instalação](docs/DEVELOPMENT.md#procedência-e-licenças-do-pacote)
- [x] Áreas suportadas na v1.0 implementadas e validadas, com lista de campos e variantes suportadas e decisão registrada para cada ampliação candidata.
- [x] Nenhuma dependência do leitor/escritor JavaScript em produção. A alpha.6 empacota apenas os três recursos do frontend compilado. A interface chama o IPC do Tauri e o núcleo Rust executa leitura e escrita. [Configuração do pacote](apps/desktop/src-tauri/tauri.conf.json)
- [x] Original preservado e gravação transacional em cópia, sem apresentar arquivos parciais como válidos. A regressão em release cobre destino exclusivo, alteração concorrente da origem, cancelamento e publicação de transações mistas. A matriz de quatro saves conferiu os hashes dos originais. Na regressão mista, a cópia preservou salário e data do vínculo de Liam Chilvers no Arsenal enquanto atualizou o salário e o vencimento do empréstimo ao Northampton junto com alterações de jogador, comissão, clube e remoções de saúde.
- [x] Nenhum ajuste implícito de CA, conversão aproximada silenciosa ou lote parcialmente aplicado sem informação. As políticas de CA são explícitas e os alvos inexatos são recusados. A prévia sinaliza limites e bloqueia lotes com campos indisponíveis. A regressão com Quim e Clodoaldo confirma que um alvo impossível impede a publicação do lote inteiro, mesmo quando o outro alvo pode ser gravado sem mudança de CA. [Teste de regressão](crates/cm0304-core/tests/batch_contracts.rs)
- [x] Matriz local de carregamento, avanço, eventos, regravação e novo carregamento aprovada para as operações suportadas. Contratos ativos de seleção e outros estados de recuperação de lesão continuam em pesquisa.
- [x] A bateria local das operações suportadas passou sem defeitos conhecidos de corrupção, edição da entidade errada ou valor incorreto.
- [x] Busca, filtros e edição responsivos, com progresso nas operações longas. O beta.7 mediu abertura, busca e ficha nas duas amostras de referência e confirmou gravação e cancelamento por IPC real. Na carreira brasileira local, o beta.13 processou um lote de 25.552 jogadores. A prévia abriu em 1,97 s, a revisão em 1,88 s e a gravação foi confirmada em 1,84 s. A releitura confirmou 25.483 bytes de moral alterados e o hash da origem preservado. O beta.16 retomou um plano de 203.108 alterações em 3,35 s e gravou a cópia em 1,89 s. [Medições](reports/v0.8-performance.md)
- [x] Aceite com validações locais no CM0304 4.1.5, conforme decisão do responsável pelo projeto. O roteiro de teste independente permanece disponível para validação adicional.
- [x] Documentação em português, incluindo cópias, limitações e recálculos do jogo. O guia também explica a consulta por formato de registro financeiro e o fluxo para recuperar o save de origem. [Guia de uso e compatibilidade](README.md#usar-o-aplicativo-no-windows)
- [x] Pacote 1.0.0 auditado. O ZIP e o NSIS passaram no teste de integridade, o binário extraído coincidiu com o build, a licença MIT e os avisos coincidiram com as referências, e o NSIS contém apenas o editor e seis recursos. SHA-256 do ZIP `b041f9226014674f1647db2ab04ed327503f49d0f2e1074838e1e65ecf6a28d7`. SHA-256 do NSIS sem assinatura Authenticode `ebaafada03516aa84d5b0e524e49b1a78a5667fc5875d25b587b28e169f539a0`. O editor instalado exibiu 1.0.0, gravou a cópia de referência e foi desinstalado com os saves preservados.

Na auditoria local da alpha.11, o instalador NSIS x64 continha apenas o executável do editor e seis componentes de instalação. O ZIP continha o instalador, a licença MIT, os avisos de terceiros e o roteiro de teste. Os hashes do instalador e da licença conferiram com os arquivos de referência. Os avisos empacotados coincidiram com a geração pelo Cargo.lock para Windows x64, que listou 227 dependências externas com licença declarada. A inspeção confirmou a ausência de saves, banco e executável do jogo. A licença MIT foi confirmada para o código do editor. O aceite da v1.0 usa os ensaios locais. [Pacote e validação](docs/DEVELOPMENT.md#instalador-windows-para-testes-da-v08)

A regressão local da alpha.6 aprovou 64 testes Rust com `cargo test --offline --locked --release --workspace` e 12 testes JavaScript com `npm test`. O conjunto Rust inclui leitura de saves reais, conversão, variantes de contrato, remoções de saúde e suspensão, edição de comissão, finanças, escrita transacional e testes de sessão do desktop.

O worktree também passou `cargo fmt --all -- --check` e `cargo clippy --offline --locked --workspace --all-targets -- -D warnings`. Após ajustar a leitura das rubricas financeiras para usar grupos fixos de quatro bytes, os sete testes de clubes passaram em release. A leitura pelo CLI confirmou os mesmos totais nos dois checkpoints do Fortaleza.

A suíte de interface passou 34 cenários no Chrome headless com IPC simulado e nenhum erro de execução do frontend. O comando `npm run test:ui` compila a interface, fornece os arquivos locais pelo Playwright e executa a regressão de troca de save, descarte, desfazer/refazer, lotes, comparação de contratos e exportações. A suíte também verificou que a interface permite sair quando o editor está ocioso e protege a saída durante a leitura de um save. [Suíte de regressão](tools/check-staff-ui.playwright.js)

A regressão local da alpha.7 passou com 65 testes Rust em release no workspace e 12 testes JavaScript. A suíte Rust inclui diagnóstico de mercado, comparação de elencos, edição em lote e persistência de sessão com verificação do hash da origem.

A regressão da alpha.13 passou com 68 testes Rust no workspace em release e 12 testes JavaScript. A transação mista cobre agora a entrada do diário de mercado junto com edições de jogadores, comissão, clube e remoções de saúde. O teste confere a gravação no deslocamento recalculado da cópia e o valor anterior no deslocamento da origem.
