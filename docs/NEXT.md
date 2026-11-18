# Continuidade do desenvolvimento

Objetivo ativo: desenvolver os marcos do ROADMAP até a v1.0, sem reduzir o escopo aprovado.

## Checkpoint

- **v0.4 funcional concluída**, versão 0.4.0 em core/CLI/desktop/package. Matriz beta e estabilidade v1.0 não concluídas.
- Build desktop 0.4.0 aprovado. Executável: `target/debug/cm0304-desktop.exe`, atualmente fechado.
- Suite Rust com leitura nova: 14 testes aprovados. Clippy all-targets core/CLI/desktop aprovado. Paridade integral histórica em duas amostras aprovada após leitor de contratos.
- O teste de goleiro/jogador-staff foi reexecutado no fechamento 0.4.0 após acrescentar comparação explícita dos 15 bytes de staff: aprovado.
- Testes nativos adicionais: tauri_v04 carregou; Quim Jogo de mãos 20; Clodoaldo Guarda-Redes/Só Direito; Pär Zetterberg Velocidade 20 e aba staff acessível; busca molby encontrou Johnny Mølby.
- UI final com merge_plan: CA150 + finalização13 = dois alvos; restaurar CA142 removeu ambos, zero pendências e revisão desabilitada. Layout final examinado.
- Cópia v04_variants avançou sete dias, regravou em v04_variants7d e carregou novamente com data26/7/2003. Os13 alvos persistiram. Jogo está aberto com essa cópia, elencoBraga.
- Origem intacta. Evidências consolidadas em `reports/v0.4-validation.md`, `reports/v04-variants-write.json` e `reports/v04-variants-evolution.json`.

## Próxima etapa — v0.5

Leitura de diagnóstico de estado e contratos já adicionada, sem expor edição destes campos. Ver `reports/v0.5-research.md` e teste `crates/cm0304-core/tests/career.rs`.

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
