# Investigação de game_db.dat

Versão interna: 147. Jogadores com dados completos: 22062 → 22062.
Pessoas/entidades: 32788 → 81528. Inclui registros virtuais, aposentados, árbitros e comissão técnica.

Entre os jogadores com dados completos: 8589 mudaram nos campos comparados; 13473 permaneceram iguais; 0 entraram; 0 saíram.
Campos comparados: CA, PA, clube, 65 bytes de atributos e oito atributos de personalidade. Igualdade aqui não cobre contratos, lesões ou todos os dados da pessoa.

Os CSVs exportam atributos convertidos pelo modelo CM Scout 3.10 e valores brutos com sufixo `_raw`. Posições ausentes preservam zero. A conferência visual está em attribute-validation.md.

## Exemplos extraídos do save inicial

| UID | Nome | Clube | CA | PA | Aceleração (raw) | Velocidade (raw) |
|---:|---|---|---:|---:|---:|---:|
| 8076 | Zinedine Zidane | Real Madrid C.F. | 191 | 194 | 16 | 14 |
| 9888 | Thierry Henry | Arsenal | 181 | 190 | 20 | 20 |
| 303210 | Leandro | Fortaleza EC | 113 | 138 | 14 | 15 |
| 311054 | Erandir | Fortaleza EC | 128 | 155 | 15 | 13 |
| 315958 | Coelho | Fortaleza EC | 83 | 126 | 16 | 16 |
| 315960 | Jessuí | Fortaleza EC | 83 | 173 | 14 | 13 |
| 317129 | Gedeon | Fortaleza EC | 104 | 134 | 11 | 5 |
| 317729 | Richarlyson | Fortaleza EC | 105 | 152 | 14 | 14 |
| 318768 | Alyson | Fortaleza EC | 122 | 151 | 14 | 15 |
| 2001642 | Clodoaldo | Fortaleza EC | 142 | 163 | 14 | 15 |
| 3301235 | Macedo | Fortaleza EC | 123 | 153 | 15 | 14 |

## Tabelas

| Tabela | Contagem inicial | Contagem final | Offset inicial no bloco |
|---|---:|---:|---:|
| awards | 791 | 791 | 37 |
| cities | 11351 | 11351 | 35636 |
| clubs | 3554 | 3554 | 421574 |
| competitions | 844 | 844 | 1160125 |
| continents | 6 | 6 | 1196421 |
| injuries | 83 | 83 | 1196527 |
| media | 69 | 69 | 1205429 |
| languages | 72 | 72 | 1211926 |
| nations | 230 | 230 | 1215020 |
| firstNames | 59136 | 59136 | 1314547 |
| surnames | 137487 | 137487 | 3444949 |
| commonNames | 19078 | 19078 | 8413000 |
| regions | 14 | 14 | 9192808 |
| sponsors | 1 | 1 | 9192938 |
| stadiums | 11123 | 11123 | 9193014 |
| stadiumChanges | 55 | 55 | 9726922 |
| stages | 163 | 163 | 9729016 |
| teams | 4714 | 4714 | 9745964 |
| weather | 70 | 70 | 10247732 |
| playerTypes | 136 | 136 | 10256570 |
| people | 32788 | 81528 | 10257934 |
| personData | 28023 | 28023 | 17370295 |
| playingData | 22099 | 22099 | 17846690 |

A leitura termina após playingData. Restam 520030 e 521686 bytes, respectivamente, ainda não interpretados.

Referência de estrutura: [código-fonte público do CM Scout 3.10, Michael Nygreen](https://sourceforge.net/projects/cmeditors/files/CM%20Scout/CM%20Scout%203.10/). Identidades e atributos ainda precisam de conferência visual no jogo antes de implementar escrita.
