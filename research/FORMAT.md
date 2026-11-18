# Investigação do banco interno — CM0304 4.1.5

## Referência

A pesquisa começou pela inspeção binária das duas amostras. Para interpretar os campos, foi consultado o código-fonte publicado por Michael Nygreen para o [CM Scout 3.10](https://sourceforge.net/projects/cmeditors/files/CM%20Scout/CM%20Scout%203.10/), arquivo `CMScout-v3.10-source.zip`, disponibilizado em 24/12/2003. SHA-256 do ZIP consultado: `6bfd7e7c481381cfc36daa29d7815f697358c990e00d6ca07d5c6f0d0efe8b8f`.

As unidades `CM4SaveGameDatabase.pas`, `CM4SaveGameData.pas` e `CM4SupportClasses.pas` descrevem a ordem das tabelas, variantes de registros, campos e representação relativa dos atributos. O leitor JavaScript foi implementado neste projeto com base nesse layout. O material de referência baixado fica em `game/research/reference`, ignorado pelo Git. Não é executado nem necessário para usar o leitor.

## Estrutura validada nas amostras

O cabeçalho de `game_db.dat` ocupa 11 bytes: assinatura de nove bytes e versão uint16 little-endian. A versão das amostras é 147. A leitura das tabelas começa no offset 37 do conteúdo do bloco. Os offsets apresentados no relatório estrutural são relativos ao bloco; os offsets de jogadores retornados pelo CLI são absolutos no arquivo `.cm4`.

As tabelas têm contagem int32 e registros precedidos por um byte de tipo. Strings têm comprimento int32, caracteres UTF-16LE e terminador de dois bytes quando não vazias. A identidade `(id, uid)` fica no final de muitos registros, incluindo os nomes. Confundir essa identidade com um cabeçalho desloca a associação entre nomes e pessoas.

Os jogadores completos são registros de pessoa dos tipos 2 e 3, ligados por índices às tabelas de nomes, equipes, personalidade e dados de jogo. Tipo 3 também inclui dados de comissão técnica. Tipo 7 representa registros virtuais, sem o conjunto completo de atributos; seu crescimento não indica novos jogadores completos.

## Campos identificados

No início dos dados específicos do jogador, após o byte de tipo da pessoa, há três reputações uint16. CA e PA são int16 nos offsets relativos +6 e +8; o índice de `playingData` é int32 em +10. Outros campos e estruturas variáveis seguem antes dos dados comuns da pessoa.

Cada registro de `playingData` tem 75 bytes: tipo (1), atributos (65 bytes com sinal), byte ainda não interpretado (1) e identidade (8). A ordem dos campos está em `PLAYING_FIELDS`, no leitor. O offset exportado `attributes` aponta para o primeiro atributo, após o byte de tipo.

Posições, pés, atributos físicos e vários mentais aparecem diretamente nos bytes. Parte dos técnicos e atributos de goleiro usa representação relativa à habilidade atual; valores negativos são esperados. As fórmulas HighConvert/LowConvert da referência estão implementadas em `src/attribute-conversion.js`, usando CA, arredondamento por truncamento após soma de 0,5 e limites 1–20. Goleiro com valor >= 15 troca os grupos técnicos principal e secundário. Campos diretos preservam seus valores, inclusive zero. O CLI exporta tanto a conversão quanto os bytes originais.

`personData` armazena oito atributos de personalidade: adaptabilidade, ambição, lealdade, pressão, profissionalismo, espírito esportivo, temperamento e controvérsia.

## Evidência e limites

O leitor percorre as tabelas de forma sequencial, inclusive contratos, ofertas, lesões e variantes necessárias para manter os limites dos registros. Resolve todas as referências usadas nos 22.062 jogadores completos de cada amostra. Os testes verificam identidades conhecidas, offsets dos atributos, continuidade das tabelas e preservação dos buffers de entrada.

O save inicial contém 32.788 entidades em `people`; o final contém 81.528. O aumento de 48.740 corresponde a registros virtuais do tipo 7, que passam de 1.280 para 50.020. O número de jogadores completos permanece igual.

A leitura para após `playingData`: restam 520.030 bytes no primeiro arquivo e 521.686 no segundo, incluindo dados ainda não interpretados. Não há suporte prometido a outras versões, compressão ou outros conjuntos de variantes. A conferência visual de 61 campos de Wender e Quim no Braga coincidiu com a conversão; no Fortaleza, os perfis parcialmente ocultos de Clodoaldo e Erandir apresentam divergências, e Richarlyson está totalmente oculto. O conhecimento limitado do treinador é uma explicação provável, ainda não isolada experimentalmente. Consulte `reports/attribute-validation.md`. Não há escrita nem teste de carregamento de um save modificado.

Os nomes dos clubes e nações são os strings presentes no save, sem sobrepor traduções do banco de idiomas. Datas são exportadas como dia ordinal e ano brutos. Não é necessário usar `people_db.dat` do banco pré-jogo para obter os jogadores dos saves.

Uma escrita de tamanho constante foi testada em cópia: Clodoaldo, `flair` 18 → 20, um byte alterado. A cópia carregou no CM0304 4.1.5 e o perfil exibiu Instinto 20. Detalhes em `reports/save-edit-validation.md`.

O escritor também suporta os atributos relativos. A inversa enumera os bytes de -128 a 127, usa a CA e o grupo atual do jogador e seleciona a codificação exata mais próxima do byte original. Solicitações não representáveis são recusadas sem alterar a CA. O conjunto de valores alcançáveis depende da CA; não se presume que todo alvo de 1–20 seja sempre possível. O passe de Clodoaldo com CA 142 é convertido de byte -17 (13) para byte 26 (20), preservando os demais bytes.
