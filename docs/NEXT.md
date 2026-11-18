# Continuidade do desenvolvimento

Objetivo ativo: desenvolver os marcos do ROADMAP até a v1.0, sem reduzir o escopo aprovado.

## Estado atual — ler primeiro

v0.5 alpha em andamento. Implementado `container::replace_block` e `health::remove_injuries` no núcleo Rust, sem integração de cura na CLI/Tauri. Remoção em memória por UID, múltiplos jogadores, no-op para saudáveis, rejeição de seleção inválida e releitura com preservação de valores. Testes estruturais verificam todos os campos semânticos dos22.062 jogadores e status/suspensões dos não selecionados. Suite workspace completa21 testes e Clippy aprovados.

Prova nativa: `v05_heal_splice.cm4` carregou em26/7/2003, Hoekstra LesõesNada/Totalmenteapto/condição100%, sem editar condição diretamente. Avançou7dias, gravou `v05_heal_splice7d.cm4` em2/8/2003; CLI confirmou ausência da lesão. Recarga nativa concluída após reiniciar o jogo: perfilHoekstra segue LesõesNada/Totalmenteapto/condição100%. Jogo atualmente nessa cópia, janela675025550 (revalidar). Relatórios em `reports/v05-heal-splice*.json`. Próximo: integrar operações estruturais ao plano/revisão/desfazer/refazer/gravação exclusiva existentes e validar outras variantes/seleção; suspensões e ciclo de contratos CLI ainda pendentes. Não marcar v0.5 ou meta1.0 concluídas.

## Histórico dos checkpoints

- **v0.4 funcional concluída**, versão 0.4.0 em core/CLI/desktop/package. Matriz beta e estabilidade v1.0 não concluídas.
- Build desktop 0.4.0 aprovado. Executável: `target/debug/cm0304-desktop.exe`, atualmente fechado.
- Suite Rust com leitura nova: 14 testes aprovados. Clippy all-targets core/CLI/desktop aprovado. Paridade integral histórica em duas amostras aprovada após leitor de contratos.
- O teste de goleiro/jogador-staff foi reexecutado no fechamento 0.4.0 após acrescentar comparação explícita dos 15 bytes de staff: aprovado.
- Testes nativos adicionais: tauri_v04 carregou; Quim Jogo de mãos 20; Clodoaldo Guarda-Redes/Só Direito; Pär Zetterberg Velocidade 20 e aba staff acessível; busca molby encontrou Johnny Mølby.
- UI final com merge_plan: CA150 + finalização13 = dois alvos; restaurar CA142 removeu ambos, zero pendências e revisão desabilitada. Layout final examinado.
- Cópia v04_variants avançou sete dias, regravou em v04_variants7d e carregou novamente com data26/7/2003. Os13 alvos persistiram. Jogo está aberto com essa cópia, elencoBraga.
- Origem intacta. Evidências consolidadas em `reports/v0.4-validation.md`, `reports/v04-variants-write.json` e `reports/v04-variants-evolution.json`.

## Próxima etapa — v0.5

**Atualização 0.5.0-alpha.1:** o código agora contém edição de estado/contratos e abas correspondentes. O checkpoint 0.4 acima é histórico. Testes workspace (17) e teste adicional de spans (1) aprovados; Clippy all-targets aprovado. Ainda não declarar v0.5 concluída.

Validação nativa avançou: `tauri_v05.cm4` salva pelo desktop, oito campos de Quim confirmados pela CLI e cópia carregada no jogo. Avançou sete dias, gravou `tauri_v057d.cm4` e recarregou após reiniciar o jogo. Contrato, camisa99 e moral persistiram; condição/cansaço/valor estimado mudaram durante a evolução. Tela após recarga mostrou99% contra campo9800 no arquivo: investigar a apresentação/estado sem prometer igualdade exata. Ver `reports/v0.5-native-validation.md`. Jogo aberto no perfilInformação deQuim, data26/7/2003; janela4850406 (revalidar antes de usar). Próximo: validar a cópia CLI `v05_career.cm4` e seu evento de vencimento, depois operações de lesões/suspensões.

Corrigida no código a projeção de perfil para aplicar o plano completo antes de selecionar o jogador, preservando transações relacionadas como troca de camisas. Clippy workspace/all-targets aprovado após a correção; binário em execução ainda anterior a esse ajuste (fechar antes de recompilar).

Novo diagnóstico `health <save> [consulta]` conserva spans de lesões/suspensões/cartões/status. Todos os jogadores do save inicial têm status, portanto presença não significa lesão ativa. Implementação e evidências em `reports/v0.5-research.md`. Próximos passos: validar a cópia no jogo e mapear operações de recuperação/remoção, mantendo o restante do escopo aprovado.

Investigação de lesões avançou: catálogo83 tipos lido pelo Rust (`injury-types`), referências verificadas nas duas amostras; `health` exporta candidatos de interpretação. HoekstraUID356 tinha entorse tornozelo desde13/7/2003, confirmada na tela e no catálogo/registro. Probeisolado `v05_injury_probe.cm4` zerou palavras44/74: carregou, mas deixou cerca2semanas fora e estadoDevoltaaostreinos; não é cura completa. Regravado como `v05_injury_prober.cm4`, sem remover registro. Evidências `reports/v05-hoekstra-*.json` e `reports/v05-injury-probe.json`. Jogo atualmente nesse experimento, perfilHoekstra, data26/7/2003, janela4981478 (revalidar). Próximo: mapear retorno/recuperação natural, além da validação de contratos CLI ainda pendente. Suite18 testes inteira, Clippy e paridade aprovados após as mudanças do catálogo.

**Checkpoint mais recente:** experimento evoluiu até6/8/2003, gravado como `6a.cm4`, e Hoekstra segue com registro e previsão3semanas: zerar palavras não cura. Outros124 jogadores tiveram remoção natural de seus registros (JSONcomparativo salvo). Adicionados offsets dos contadores/flag de saúde; testes `career` passaram em ambas as amostras. Preparada cópia `v05_heal_splice.cm4`, baseada no save26/7 original do experimento, removendo38bytes do registroHoekstra, ajustando contador e tamanho game_db, preservando status e hashes de outros blocos. Rust relê corretamente. Próximo passo: carregar essa cópia no jogo e conferir lesão/estado físico, depois avanço/regravação/recarga. Ainda não exposta cura estrutural no produto. Jogo atual em6a/perfilHoekstra,6/8/2003, janela4981478; revalidar. Lista10suspensões em `reports/v05-ban-candidates.json`, sem operação implementada. Detalhes em `reports/v0.5-research.md`.

Leitura de diagnóstico de estado e contratos adicionada na base 0.4. Ver `reports/v0.5-research.md` e teste `crates/cm0304-core/tests/career.rs`.

- `PlayingInfo.state`: estimatedValue, saleValue, morale, fitness, jadedness, condition.
- `Person.contracts`: tipo, offsets relativos ao banco, clube/seleção, índice, salário semanal, término/início, situação, flags de transferência e camisa.
- CLI `career <save> [consulta]` exporta diagnóstico com databaseBase. O modelo público Player histórico continua igual; a leitura nova não removeu nem afrouxou a paridade.
- Clodoaldo contrato completo Fortaleza: wage1490, início1/7/1999, término1/1/2005; Quim condition9351/camisa1; Liam Chilvers UID5101855 contrato completoArsenal e empréstimoNorthampton.

Próximo trabalho: regras e metadados de edição de estado/contratos existentes; unidades e datas especiais; contrato selecionado explicitamente; preservar relação clube proprietário/equipe atual/empréstimo. Expor abasEstado/Contrato, com prévia e validação. Mapear lesões/suspensões sem zerar contadores de arrays variáveis. Depois executar ciclo no jogo incluindo datas/eventos relevantes.

A moeda/arredondamento salarial ainda requer validação específica: o campo é salário semanal interno; a tela mostra £6.000/mês para o valor1490. Não prometer equivalência exata de interface antes de conferir a regra.

## Cuidados de continuidade

- Não sobrescrever saves. Arquivos existentes de teste permanecem em `game/user data/games`.
- Antes de recompilar o desktop no Windows, fechar a instância pela UI para liberar o executável.
- CM0304 não trata Ctrl+A/Ctrl+V como controles Windows convencionais nos campos personalizados: eles inseriram letras. Para nomes curtos, usar teclas individuais e BackSpace, com uma ação e screenshot por chamada conforme skill.
- Ainda faltam v0.5–v1.0, nacionalidades adicionais em consulta, matriz de quatro saves de duas carreiras, outra pessoa e máquina limpa. Meta ativa.
