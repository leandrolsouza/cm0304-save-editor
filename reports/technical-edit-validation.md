# Edição técnica: passe de Clodoaldo

## Resultado no jogo

A cópia `game/user data/games/clodoaldo_passe20.cm4` carregou normalmente no CM0304 4.1.5. Após pesquisar Clodoaldo e abrir o perfil do Fortaleza em 19/07/2003, o campo **Passe exibiu 20**. Instinto continuou em 18, pois esta cópia foi criada a partir do save inicial, independentemente do teste anterior.

Não foi avançada a data nem regravada a partida pelo jogo. A confirmação cobre carregamento e exibição do perfil; não testa a manutenção do valor após evolução da carreira.

## Alteração binária

- Origem: `game/user data/games/save_inicial.cm4`.
- Jogador: Clodoaldo, UID 2001642, Fortaleza EC.
- Campo: `passing`.
- Valor convertido: **13 → 20**.
- Byte com sinal: **-17 → 26**.
- CA preservada: **142**.
- Offset absoluto: **20622032**.
- Tamanho preservado: **35054444 bytes**.
- Exatamente **um byte** diferente; todos os demais bytes preservados.

SHA-256 do original: `1e2280f838e8516d4da69c3b15faf01df9ec7cd085a03f56a58e0f516894d3b4`.

SHA-256 da cópia: `975b316edd6527e947115e8614ebc6ba9570869a90850c01f3109583a9e63c75`.

O passe 13 é o valor decodificado do arquivo inicial. A interface do treinador pode mostrar outro valor estimado/oculto, conforme registrado na investigação anterior; o alvo 20 foi observado diretamente nesta cópia.

## Inversa e limites

O escritor enumera os 256 bytes com sinal e procura aqueles cuja conversão, com a CA e a posição atuais, resulta exatamente no alvo 1–20. Escolhe o mais próximo do byte original. Não modifica CA nem outros atributos para compensar o alvo. Um alvo impossível é recusado.

A implementação suporta os 25 campos relativos de `RELATIVE_ATTRIBUTES`, incluindo passe e outros técnicos, campos de goleiro e os relativos comuns. A conferência visual desta edição cobre apenas passe de jogador de linha. Os testes cobrem os dois modos de conversão para CA de 0 a 200, inclusive alvos impossíveis.

Uma tentativa com nome de arquivo longo falhou no menu do jogo, que apresentou o nome truncado. O mesmo conteúdo binário carregou com o nome curto acima. O CLI agora limita o nome sem extensão a 26 caracteres e exige `.cm4`. A cópia longa de teste foi removida.
