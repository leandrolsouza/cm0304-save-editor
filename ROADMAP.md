# Roadmap até a v1.0

Atualizado em 18/11/2026. Stack definida: **Rust + Tauri**. Este documento planeja a evolução; os recursos previstos ainda não estão implementados, salvo os indicados no estado atual.

## Objetivo

Entregar um editor desktop local para Windows, em português, para saves do **CM0304 4.1.5**. A v1.0 permitirá editar jogadores, comissão técnica, contratos e dados de clubes, trabalhar em lote e salvar cópias carregáveis pelo jogo. O usuário final não precisará de terminal, Node.js ou Rust instalado.

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

- [x] **v0.2:** núcleo Rust com leitura equivalente, transações de múltiplos campos/jogadores e gravação exclusiva validada. Cópia carregada no jogo.
- [x] **v0.3:** aplicativo Tauri com abertura, busca paginada, perfil, revisão, desfazer/refazer e salvar cópia. Fluxo completo executado na interface e cópia carregada no jogo.
- [x] **v0.4:** editor ampliado de jogadores, políticas de CA, abas, idade e busca sem acentos. Fluxo Tauri, posições/pés, goleiro e jogador/staff conferidos; cópias avançaram sete dias, foram regravadas e recarregadas no jogo.
- [ ] **v0.5:** estado e contratos existentes em prévia; remoção de lesões e suspensões integrada às transações. Transação mista Tauri de lesão/moral conferida no jogo; primeiros casos de cura e suspensão europeia passaram sete dias, regravação e recarga. Fluxo Tauri de suspensões testado até gravação/releitura, incluindo desfazer/refazer. Faltam conferir essa cópia no jogo, outras variantes e a matriz de contratos/valores.
- [ ] **v0.6–v1.0:** ainda não concluídas.

Evidências: `reports/v0.2-v0.3-validation.md` e `reports/v0.4-validation.md`. As prévias são marcos de desenvolvimento, não uma declaração de estabilidade da v1.0.

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

Os primeiros atalhos de lote serão **recuperar elenco**, **ajustar moral**, **aplicar piso/teto de atributos**, **aumentar/reduzir atributos selecionados** e **copiar um grupo de atributos**. Cada atalho usa as mesmas regras da edição individual; nenhum altera CA ou vínculos silenciosamente.

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

**v0.7 — Ferramentas para editar e analisar em escala**

- Filtrar por clube, nacionalidade, idade, posição, pé, CA/PA, atributo e situação contratual; combinar filtros e ordenar resultados.
- Selecionar jogadores individualmente, todo o elenco ou todos os resultados filtrados, com contagem explícita.
- Aplicar valor fixo, incremento/redução e limites mínimo/máximo em campos compatíveis.
- Criar e salvar presets locais, por exemplo: recuperar elenco, ajustar profissionalismo ou melhorar fundamentos de uma posição.
- Copiar atributos selecionados de um jogador para outro, preservando identidade e vínculos; validar conversões e limites de cada destino.
- Comparar jogadores lado a lado e comparar a mesma entidade entre dois saves por UID.
- Exportar resultados e diferenças para CSV/JSON, salvar filtros e manter histórico local das alterações realizadas.
- Desfazer/refazer transações inteiras e remover uma alteração pendente antes de salvar.
- Mostrar um painel do elenco com distribuição de idade/posição, médias de atributos, folha salarial e vencimentos próximos; permitir abrir os jogadores a partir desses resumos.
- Oferecer presets iniciais editáveis e permitir duplicar, renomear, excluir e exportar/importar presets locais, com versão e validação dos campos.

**v0.8 — Produto utilizável no Windows**

- Gerar instalador e documentar dependências de execução do Tauri/WebView2.
- Implementar recentes, restauração de filtros/preferências, atalhos e estados de carregamento/erro claros.
- Permitir cancelar buscas ou análises demoradas e confirmar descarte de alterações pendentes.
- Medir abertura, busca, consumo de memória e gravação nas amostras; registrar limites e corrigir travamentos.
- Testar instalação e fluxo completo em máquina sem Rust/Node.js e ampliar a matriz de saves e usuários.
- Guardar relatórios de gravação consultáveis no aplicativo e permitir localizar a cópia gerada e sua origem.

### Sequência de entrega e demonstrações

| Marco | Demonstração necessária na interface |
|---|---|
| **0.4 — Jogador** | Abrir Clodoaldo, editar vários grupos, revisar conversões e salvar uma cópia. |
| **0.5 — Carreira** | Alterar estado e contrato, remover uma lesão e uma suspensão suportadas e conferir os efeitos após evolução no jogo. |
| **0.6 — Clube** | Abrir um clube, navegar entre elenco e comissão, editar staff e dados financeiros/infraestrutura validados. |
| **0.7 — Lote** | Filtrar um elenco, aplicar um preset, revisar exceções, desfazer/refazer e comparar o resultado com a origem. |
| **0.8 — Beta** | Instalar em Windows sem ferramentas de desenvolvimento e completar os fluxos anteriores em outra carreira. |

A prioridade é concluir as validações pendentes da v0.5 e seguir para staff e clubes. Novas funcionalidades deste planejamento não antecipam a conclusão de nenhum marco.

**v0.9 — Estabilização**

- Congelar funcionalidades, corrigir regressões e concluir a matriz de operações e variantes suportadas.
- Finalizar ajuda em português, exemplos, limitações, notas de versão e instruções de recuperação usando a cópia original.
- Revisar mensagens, navegação por teclado, legibilidade, licenças e conteúdo do pacote.

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

- Transferências, novos empréstimos, rescisões e criação de contratos: exigem sincronizar elenco, propriedade, inscrições e referências.
- Criar/remover jogadores, renomear entidades e alterar estruturas de comprimento variável.
- Adicionar/remover nacionalidades e alterar elegibilidade além do campo principal validado.
- Importação CSV para escrita e automações com regras personalizadas.
- Competições, calendários, regras, resultados e ligas.
- Outros patches, outras edições, edição em memória, macOS/Linux e nuvem.

## Critérios de lançamento

- [ ] Aplicativo Rust + Tauri sem terminal ou toolchain de desenvolvimento para o usuário final.
- [ ] Áreas da v1.0 implementadas e validadas, com lista de campos e variantes suportadas.
- [ ] Nenhuma dependência do leitor/escritor JavaScript em produção.
- [ ] Original preservado e gravação transacional em cópia, sem apresentar arquivos parciais como válidos.
- [ ] Nenhum ajuste implícito de CA, conversão aproximada silenciosa ou lote parcialmente aplicado sem informação.
- [ ] Matriz de carregamento, avanço, eventos, regravação e novo carregamento aprovada.
- [ ] Nenhum defeito conhecido de corrupção, edição da entidade errada ou valor incorreto.
- [ ] Busca, filtros e edição responsivos, com progresso nas operações longas.
- [ ] Beta testado por alguém além do desenvolvedor e em outras carreiras.
- [ ] Documentação em português, incluindo cópias, limitações e recálculos do jogo.
- [ ] Licenças e atribuições revisadas; pacote sem jogo, saves ou banco do CM0304.
