# Evidências e relatórios

- Preserve relatórios e artefatos históricos. Corrija conclusões incorretas com contexto suficiente para distinguir o ensaio antigo da correção.
- Registre cenário, origem e variante, UID/campo quando aplicável, valores anteriores/alvos/relidos, comandos ou procedimento, resultado e limitações.
- Para escrita binária, registre hashes e preservação da origem e dos blocos não envolvidos quando esses checks forem realizados. Não invente medições nem hashes.
- Diferencie: teste automatizado, UI com IPC simulado, Tauri nativo, carregamento no CM0304 e ciclo completo de avanço, regravação e recarga após reiniciar o jogo.
- Declare como pendente qualquer etapa não executada. Uma cópia relida pelo editor ou carregada no jogo não comprova persistência após evolução da carreira.
- Restrinja conclusões à versão do jogo, às carreiras e às variantes observadas. Novos formatos e offsets devem ser documentados também em `research/FORMAT.md` quando confirmados.
- Use nomes descritivos e relativos ao cenário. Evite incorporar caminhos pessoais ou arquivos do jogo aos artefatos versionados.
- Antes de regenerar JSON, CSV ou imagens existentes, confira se são evidência histórica ou saída descartável. Revise o diff e mantenha somente artefatos pertinentes à tarefa.
