# Desenvolvimento Rust + Tauri

[Current development guide in English](DEVELOPMENT.md)

## Estrutura

- `crates/cm0304-core`: leitor binário, contêiner, banco, atributos e transações de edição.
- `crates/cm0304-cli`: inspeção, pesquisa, exportação e execução de planos JSON.
- `apps/desktop/src-tauri`: aplicativo Tauri e sessão de edição em memória.
- `apps/desktop/ui`: interface TypeScript/CSS, compilada para `dist`.
- `src`, `tests`, `tools`: prova de conceito JavaScript e verificadores de comparação.

## Comandos

```powershell
npm ci
npm run build:desktop
./target/debug/cm0304-desktop.exe
cargo test
cargo clippy --workspace --all-targets -- -D warnings
npm run verify:parity
```

O desenvolvimento exige Rust, ferramentas de compilação C++ do Windows, WebView2 e Node.js. `npm ci` instala o TypeScript e a CLI do Tauri nas versões do lockfile. `npm run build:desktop` usa Cargo offline. Em uma máquina nova, execute `cargo fetch` com acesso à rede antes desse build. `npm run build:installer` usa a CLI local do Tauri. O primeiro empacotamento pode baixar as ferramentas do NSIS. O executável instalado usa WebView2 e dispensa Node.js.

Os testes com amostras reais usam os arquivos em `game/user data/games`. Eles não são incluídos no repositório. A falta das amostras causa falha nesses testes, em vez de sucesso sem executá-los.

A interface usa português do Brasil por padrão. Ao adicionar um texto visível, mantenha o original em português e inclua a tradução em `apps/desktop/ui/en.ts`. Use `t()` nos textos renderizados pelo TypeScript. O HTML inicial é traduzido ao carregar a janela. Preserve identificadores, nomes da carreira e chaves persistidas ao traduzir seus rótulos. A regressão de interface percorre os dois idiomas em janelas de 1280, 960 e 760 pixels de largura.

## Validação da versão 1.1.0

- No WebView2 com IPC real, o editor alterou os salários de dois jogadores em sequência e manteve a aba **Contratos**. A busca na revisão abriu o salário do empréstimo de Liam Chilvers para correção. A revisão manteve o filtro e gravou duas variantes. A tentativa com nome repetido foi recusada. A variante voltou a aparecer no histórico com o SHA-256 correspondente ao arquivo gravado.
- A amostra original e a origem de teste preservaram o SHA-256 `1e2280f838e8516d4da69c3b15faf01df9ec7cd085a03f56a58e0f516894d3b4`. A variante corrigida tem SHA-256 `329f72bfe2e6e8f5df4ecb851326ed36c82975a406f9159ee8d5f868390b1aa8`.
- O CM0304 4.1.5 abriu a variante, avançou de 19 a 20 de julho de 2003, regravou a cópia de teste e retomou a carreira na nova data. A releitura do arquivo regravado confirmou £478/semana no empréstimo de Liam Chilvers, £523/semana no clube proprietário e £1.701/semana para Quim.
- O ZIP para Windows x64 contém o instalador, a licença, os avisos de terceiros e o roteiro. O 7-Zip verificou o ZIP e o NSIS. O executável extraído tem versão 1.1.0 e coincide com o build de release. O SHA-256 do ZIP é `c2c5a6729c4ef458b923f30de13381f152f28dcde10567c49b630e9560c93c97`. O SHA-256 do instalador sem assinatura Authenticode é `691cc7f507b9b6561571567e34cf8ade541aa6737fb1f513c485d3f9b4439ddc`.
- Passaram 95 testes Rust em release no workspace, 12 testes JavaScript e 39 cenários de interface no Chrome com IPC simulado. A compilação da interface e do desktop, a formatação Rust e o Clippy também passaram.

## Publicação no GitHub

O CI Windows executa build da interface, testes com IPC simulado, formatação e Clippy, além dos testes Rust e JavaScript independentes do jogo. A suíte completa e a paridade usam os saves privados de `game/user data/games` e são executadas localmente antes da tag.

Configure a variável `TAURI_UPDATER_PUBLIC_KEY` e os secrets `TAURI_SIGNING_PRIVATE_KEY` e `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` no GitHub Actions. Guarde uma cópia protegida da chave privada e da senha para as próximas versões. A chave pública acompanha o instalador e o Tauri verifica a assinatura antes de instalar qualquer atualização.

Alinhe a versão nos manifests do Rust, npm e Tauri. A tag estável `v<versão>` dispara o workflow que gera o ZIP x64, o SHA-256, o instalador NSIS, a assinatura e o `latest.json`. O workflow cria uma release em rascunho. Confira os artefatos e o teste nativo antes de publicar a release estável como a mais recente. Os instaladores consultam `releases/latest/download/latest.json` desse repositório.

O editor consulta o manifesto ao abrir e baixa a atualização em segundo plano. Se o usuário abrir um save ou iniciar outra operação nesse intervalo, a instalação fica para uma próxima abertura do editor. Somente uma instância do aplicativo pode ficar ativa por usuário no Windows.

No Windows, execute `node tools/check-windows-updater.js` para testar duas versões com WebView2, NSIS, uma chave temporária e uma cópia de `save_inicial.cm4`. O aplicativo de teste usa nome, identificador e pasta de instalação próprios. O teste verifica o adiamento com uma carreira aberta, a atualização após reabrir e a preservação da preferência de tema e dos hashes do save.

O ensaio local de 1.1.0 para 1.1.1 concluiu a atualização e reabriu o editor com IPC nativo, o tema retrô e a carreira na lista de recentes. A instalação aguardou a próxima abertura quando havia um save ativo. Os SHA-256 da amostra e da cópia de teste ficaram intactos. A versão 1.1.1 desse ensaio foi servida em loopback e usou uma chave exclusiva do teste.

## Planos de edição

```json
[
  { "uid": 2001642, "attribute": "passing", "value": 20 },
  { "uid": 9888, "attribute": "pace", "value": 19 }
]
```

```powershell
cargo run -p cm0304-cli -- edit "origem.cm4" "nova_copia.cm4" "plano.json"
```

Campos repetidos, não suportados, sem alteração efetiva ou não representáveis são recusados. A transação é validada integralmente antes da gravação. A origem e qualquer destino existente são preservados. O temporário é escrito no diretório de destino, sincronizado, relido e finalizado sem sobrescrita.

Na interface, alterações de um mesmo campo substituem a intenção anterior na sessão; o plano final contém uma intenção por UID/campo. Desfazer/refazer opera sobre esses planos. A cópia usa o snapshot aberto e recusa uma origem modificada externamente durante a sessão.

Para combinar campos e remoções estruturais, use `apply-plan` com um objeto:

```json
{
  "edits": [{ "uid": 356, "attribute": "state.morale", "value": 20 }],
  "healInjuries": [356],
  "clearSuspensions": [5640794]
}
```

```powershell
cargo run --offline -p cm0304-cli -- apply-plan "origem.cm4" "nova_copia.cm4" "transacao.json"
cargo run --offline -p cm0304-cli -- health "origem.cm4" "5640794"
cargo run --offline -p cm0304-cli -- health "origem.cm4" "uid:1873"
cargo run --offline -p cm0304-cli -- health-cases "origem.cm4"
```

`health` mostra os registros brutos de uma entidade; `health-cases` inventaria as variantes estruturais presentes, com contagens e até três exemplos por grupo. Os discriminadores brutos não representam automaticamente duração ou categoria. Consulte `reports/v0.5-suspensions.md` para os casos conferidos no jogo e as variantes ainda pendentes. Remover suspensão preserva os cartões; remover lesão preserva condição e ritmo.

As consultas `players`, `career` e `health` aceitam `uid:<número>` para igualdade exata. Sem o prefixo, a pesquisa é textual e pode retornar UIDs que apenas contenham os dígitos informados.

## Diagnóstico de staff

Para antecipar os efeitos de uma CA de staff, sem gravar o save:

```powershell
cargo run --offline -p cm0304-cli -- staff-ability "origem.cm4" 4950 100 preserveValues
cargo run --offline -p cm0304-cli -- staff-ability "origem.cm4" 4950 100 preserveRelative
```

A política é obrigatória. Preservar valores exige inversa exata; preservar bytes mostra atributos recalculados. Esta é uma prévia de diagnóstico, ainda sem escrita de CA. Exemplos e verificação em `reports/v0.6-staff-ability.md`.

Para escrita experimental de CA/PA e reputação no CLI, `apply-plan` aceita `staffHeaderEdits`; exemplo em `reports/v06-staff-header-plan.json`. Esse grupo usa o cabeçalho da função de staff e inclui os efeitos de CA na revisão. O Tauri prepara cabeçalho e atributos juntos por `stage_staff_edits`, usando `Transaction::merge_staff`; campos não enviados preservam as intenções anteriores. Limites e pendências nativas em `reports/v0.6-staff-header.md`. O comando `staff-ability` permanece apenas uma prévia.

```powershell
cargo run --offline -p cm0304-cli -- staff "origem.cm4" "Braga"
cargo run --offline -p cm0304-cli -- staff "origem.cm4" "uid:4950"
```

Leitura separada do modelo de jogadores, com vínculos de clube/seleção, atributos brutos e convertidos. Oito atributos profissionais usam CA de staff ou, na dupla função, CA de jogador; os demais são diretos. A conversão possui 24 observações de controle em `reports/v06-staff-conversion-observed.json`.

A escrita experimental no núcleo aceita `staffEdits` em `apply-plan`, com UID, `attribute` e `value` de 1 a 20. Os 11 campos permitidos estão em `staff_editor::EDITABLE`; CA/PA e personalidade ainda não têm escrita de staff. O plano de exemplo está em `reports/v06-staff-edit-plan.json`. O relatório separa `staffChanges` das alterações de jogador e mantém offsets da origem, antes de remoções estruturais. Campos desconhecidos, repetidos, referências compartilhadas e alvos não representáveis são recusados. A interface prepara essas edições por `stage_staff_edits`, com revisão e histórico compartilhados. A escrita permanece experimental até passar pelo ciclo de carregamento, conferência, evolução e recarga no jogo.

O frontend de consulta também pode ser verificado em navegador isolado com IPC controlado; comandos e limites da prova em `reports/v0.6-staff-ui.md`. Isso não substitui a conferência no aplicativo Tauri com save real.

Na dupla função, alterar CA de jogador também pode recalcular atributos profissionais. O relatório transacional inclui esses efeitos derivados mesmo sem mudança no byte de staff e consolida alvos explícitos contra a origem. A política de jogador preserva seu grupo de atributos; use alvos profissionais para ajustar esse outro grupo. Verificação em `reports/v0.6-dual-staff-review.md`.

`staffEdits` também aceita `personality.adaptability`, `ambition`, `loyalty`, `pressure`, `professionalism`, `sportsmanship`, `temperament` e `controversy`, todos com prefixo `personality.` e valores1–20. O registro é comum à pessoa. Referências compartilhadas entre pessoas e intenções conflitantes entre os grupos de jogador/staff são recusadas. Plano de exemplo em `reports/v06-staff-personality-plan.json`, evidência e pendências em `reports/v0.6-staff-personality.md`.

## Consulta de clubes

```powershell
cargo run --offline -p cm0304-cli -- clubs "origem.cm4" Fortaleza
cargo run --offline -p cm0304-cli -- clubs "origem.cm4" uid:1488
```

O núcleo preserva id/UID do contêiner e resolve os membros por referências, sem associar por nome. O Tauri oferece `search_clubs` (páginas de 50) e `club_detail` (links de elenco e staff/direção). O grupo profissional pode conter a mesma pessoa do elenco na dupla função. A consulta de elenco atual é distinta de propriedade contratual; finanças e infraestrutura não têm escrita nesta etapa. Evidências e pendências em `reports/v0.6-clubs-navigation.md`.

## Testes da interface

Para rodar a regressão da interface no Windows, instale as dependências com `npm ci` e execute `npm run test:ui` com o Microsoft Edge instalado. O comando compila o frontend e executa a suíte em Edge headless, com respostas simuladas aos comandos Tauri. A instalação, os diálogos do Windows e a gravação real de saves são verificados pelos testes nativos.

Com o Google Chrome instalado, também é possível rodar a mesma suíte usando `$env:CM0304_UI_BROWSER = 'chrome'` antes do comando.

## Referências de Tauri

- [Comandos Rust](https://v2.tauri.app/develop/calling-rust/)
- [Configuração do aplicativo](https://v2.tauri.app/reference/config/)
- [Diálogos nativos](https://v2.tauri.app/plugin/dialog/)

## Instalador Windows para testes da v0.8

Execute `npm run build:installer` na raiz do repositório. O primeiro build precisa de acesso à rede para obter o CLI Tauri e as ferramentas de empacotamento NSIS. O instalador é gerado em `target/release/bundle/nsis/` e usa a versão definida em `apps/desktop/src-tauri/tauri.conf.json`.

O computador de destino usa o Microsoft Edge WebView2 Runtime. Rust e Node.js são dependências de compilação. Os saves e arquivos do CM0304 ficam fora do instalador. Valide o pacote em uma instalação de teste, abrindo um save, revisando uma edição e gravando e recarregando a cópia. O roteiro em Windows limpo é opcional.

Para revisar o conteúdo e gerar o hash do instalador local, use o 7-Zip e o PowerShell. Ajuste o nome abaixo à versão que acabou de empacotar.

```powershell
$installer = 'target/release/bundle/nsis/CM0304 Save Editor_0.9.0-beta.6_x64-setup.exe'
& 'C:/Program Files/7-Zip/7z.exe' l $installer
Get-FileHash -LiteralPath $installer -Algorithm SHA256
Get-AuthenticodeSignature -LiteralPath $installer | Select-Object Status
```

A listagem deve conter o executável do editor e os componentes do NSIS. Confira a ausência de `.cm4`, arquivos do CM0304 e bancos de dados do jogo antes de distribuir.

A prévia `0.5.0-alpha.26` diferencia prestações ausentes de formato indisponível. Na instalação isolada, o save inicial mostrou nenhuma prestação para o Braga. Uma cópia de teste com a assinatura da tabela de prestações alterada em um byte ainda abriu o perfil do clube e exibiu formato indisponível. O instalador e o ZIP passaram no 7-Zip. SHA-256 do instalador `5C933CAD697612EC103809275BCE7683950CE063693028561D80D73B55E4DBB6`. SHA-256 do ZIP `BB13E690164056E01CFF191A79C7F3E310A00E17B3E0AD5CBE1DBE47CDBD62A3`.

A prévia `0.5.0-alpha.27` protege o fechamento durante a gravação de cópia. O 7-Zip aprovou o instalador e o ZIP, e a listagem do NSIS mostrou o executável do editor e seis recursos do instalador. SHA-256 do instalador `4F709CA294E9148CEE31E00B7A049BFEAB8A3EF4EFA8F76A3AB2BE032E8DED8F`. SHA-256 do ZIP `57481C1D693B50E524AFBE5FC012CA395CB68DEE7A5A1EE99DA5D1D5EBF06F60`. A interação com a janela durante a gravação ainda aguarda ensaio nativo.

No Windows de desenvolvimento, a instalação silenciosa da alpha.27 em uma pasta isolada produziu o executável com versão `0.5.0-alpha.27` e SHA-256 `7790759DE71C0B9FDB00732C2A3C31E4E029940DFE40AE49A0E96B3168CC2B50`, igual ao build de release e ao executável extraído diretamente do NSIS. A desinstalação retornou código zero, removeu o executável e preservou um arquivo de controle criado depois da instalação.

Com o executável extraído do NSIS em um perfil WebView2 isolado, a interface abriu o save inicial, preparou Cruzamentos 11 → 20 para Clodoaldo e gravou `fechamento.cm4` pelo diálogo nativo. O controle Cancelar gravação estava visível na observação anterior à tentativa de fechar a janela. Na observação posterior, a janela continuava aberta e o aplicativo informou que a cópia estava validada. Como a gravação pode ter terminado entre as duas observações, o ensaio ainda precisa isolar o instante em que o evento de fechamento chega ao Tauri. A releitura Rust confirmou Cruzamentos 20 e o SHA-256 da origem permaneceu igual. SHA-256 da cópia `39F7600F8460C176CD76D950479BFE7471DB8E496CD067177612B21CD8FC7AF1`.

A prévia `0.5.0-alpha.25` mostra as prestações registradas nas fichas do clube comprador e do vendedor. A instalação isolada abriu `h.cm4` e confirmou Quínio no Loures com 17 prestações de valor bruto 330 a pagar ao Braga. Na ficha do Braga, as mesmas 17 prestações aparecem a receber do Loures. O instalador e o ZIP passaram no 7-Zip. SHA-256 do instalador `C5228D54466BA30805DF18026389008F60DD4CDB738ED4707E963ECE700903D9`. SHA-256 do ZIP `F34649AEF377AAA384CFB4AA559E93589452CB7D1377D908DDB55736B75AC984`.

A prévia `0.5.0-alpha.24` inclui a otimização do filtro por nacionalidade. O ensaio com IPC real confirmou os mesmos totais e UIDs da primeira página em cinco consultas. O 7-Zip verificou o instalador e o ZIP. SHA-256 do instalador `F9531A7CF42B7D9CE7380540A7C1804031C7EC91DE6E627FF9EDB21F21B93E30`. SHA-256 do ZIP `A0BF269BED8F197C3266F1FB0BBB303F63C443E6F38A8B81AFDABF11E418F7FB`. [Medição](../reports/v0.8-performance.md)

A prévia `0.5.0-alpha.23` mostra as cláusulas de revenda registradas na ficha do clube comprador. A instalação isolada abriu `h.cm4` e exibiu Quínio · 20% para Sporting Clube de Braga na ficha do Loures. O instalador e o ZIP passaram no 7-Zip. SHA-256 do instalador `552E649EB8B492113650927854D7F14BEA116229F02000F902B2F573D9318991`. SHA-256 do ZIP `0B02CDE29C2382E346B896DE44C17F862905120B1695225A4CE70F5E8F9E52A4`.

A prévia `0.5.0-alpha.22` informa quando o diário do save só permite consultar a situação de mercado. No editor instalado, `3.cm4` (duas propostas aceitas) abriu a ficha de Quínio com a mensagem e sem seletor. Em `k.cm4`, o backend retornou a transição 5 → 7. O instalador e o ZIP passaram na verificação do 7-Zip. SHA-256 do instalador `1947CAB72D4427B436D8472D607742DD03EDB8450646C8CA684D0B4AFBD8C41D`. SHA-256 do ZIP `F008D16C0841F8CDC19B7DD4D6B9B7821134CCF9BAA02381362A809374E1B788`.

A prévia `0.5.0-alpha.21` traz mensagens de gravação em português e mantém a exclusão da lista padrão desabilitada após carregar um perfil. No editor instalado, um processo com acesso de escrita ao save provocou a mensagem de arquivo em uso. A tentativa deixou o destino vazio. Depois de fechar o arquivo, o mesmo plano gravou uma variante idêntica à referência de Passe 20 de Clodoaldo. O SHA-256 da origem foi preservado. O 7-Zip verificou o instalador e o ZIP. SHA-256 do instalador `9B2A19E50F31CD7393A2DD5974563631B2DD9B46DB55C40B042C4F2DA7016EA1`. SHA-256 do ZIP `04B1DB007BB1F63E366B63B712EDCE0400FD60442F6911CCF5B0C1C84DA97489`.

A prévia `0.5.0-alpha.20` impede que outro processo grave ou renomeie o save de origem durante a conferência final e a publicação da variante no Windows. A instalação de teste gravou Passe 13 → 20 para Clodoaldo, reabriu a variante pelo histórico e confirmou o valor pelo backend do Tauri. A origem conservou o SHA-256, a variante coincidiu com a referência e a desinstalação terminou com sucesso. O 7-Zip verificou o instalador e o ZIP. SHA-256 do instalador `AA5D6C4841813448D96303C706BDF8ACBD7C83A28185F86EA402FC38697C1FBF`. SHA-256 do ZIP `4D3FC3F53AF9295137DEB605E3513A1D789DD63FE1770E70B79344B6C9518128`.

A prévia `0.5.0-alpha.19` inclui o rótulo acessível do seletor de listas de observação. A regressão confirmou a exclusão e o cancelamento de uma lista e a navegação por teclado nas abas do jogador. O 7-Zip verificou o ZIP e o instalador sem assinatura Authenticode. A instalação isolada produziu o mesmo executável do build release, que iniciou com o PATH restrito ao Windows e foi removido pela desinstalação. SHA-256 do instalador `5BE885E55DC59AA17F5A2A50A032F523EAF6891D95AADB23B7E3F3D472198AD3`. SHA-256 do ZIP `2EA1A4CC6F03B758478D87298E10582AD6DAAB0F59C4277842D69F25B5E7475C`.

A prévia `0.5.0-alpha.18` inclui listas de observação por nome e recupera os jogadores da lista anterior. O 7-Zip validou o instalador NSIS x64 e o ZIP. A instalação em pasta isolada, a inicialização do editor e a desinstalação passaram no Windows de desenvolvimento. O instalador sem assinatura Authenticode tem SHA-256 `420906432C8FDDA49A741D9144F10EEEC31A0161C71567F3C6D7811631706493`. O ZIP tem SHA-256 `949A782EA49A3DE754DB54EB1E008B51EA8F9275ADC4F7BBC713B53C87D7C1E6`.

No Windows de desenvolvimento, a alpha.18 instalada abriu uma cópia descartável do save inicial, criou a lista Fortaleza, adicionou Clodoaldo e gravou Passe 13 → 20. A origem conservou o SHA-256 e a variante coincidiu com a referência já validada. Após reiniciar o executável release com o mesmo perfil WebView2, o histórico reabriu a variante, a lista Fortaleza continuou ativa e a leitura pelo IPC confirmou Passe 20. A desinstalação preservou a variante.

A prévia `0.5.0-alpha.17` permite retirar um vínculo da prévia de contratos em lote. O caso de Clodoaldo e Leandro foi conferido no Tauri e no CM0304 após avanço, gravação e recarga. O novo build também compartilha as chaves fixas dos perfis e reduz o pico de memória observado de 845,4 para 736,5 MiB. O instalador NSIS x64 sem assinatura Authenticode tem SHA-256 `2266790968BE1FE6C506D8C358C4F38FB55E08D052C9659393C2C7DD2FD1A91B`. O ZIP tem SHA-256 `40546567B9D53E3CF47B5B52F904C6ADB3B13DE0D3868ED3A4CB3F45730E7E5C`. O 7-Zip verificou ambos. Instalação isolada, abertura e desinstalação passaram no Windows de desenvolvimento.

A prévia `0.5.0-alpha.16` permite abrir o histórico antes de carregar um save e exportar relatórios arquivados. O JSON da gravação informa quantas entradas e bytes foram acrescentados ao diário de mercado. O instalador NSIS x64 sem assinatura Authenticode tem SHA-256 `EE225C9DBAFFF889EFEE09522BADFF0F99BC1B35D4C4D710DD210A1EF0E6E074`. O ZIP tem SHA-256 `CF42881CF33C56554A45CAC2C55DBB1654634B97ED766C0AEF6163ADC8D0031B`. O 7-Zip verificou os dois arquivos. O instalador foi gerado pelo comando npm com a CLI local do Tauri. A instalação isolada, a abertura da janela e a desinstalação foram conferidas no Windows de desenvolvimento.

A prévia `0.5.0-alpha.15` mostra a descrição completa da opção de mercado, valida a transição ao adicionar à revisão e oferece a edição apenas para UIDs inequívocos. A navegação de staff para jogador usa o UID e conserva a seleção na lista de staff. O instalador NSIS x64 sem assinatura Authenticode tem SHA-256 `8C8EB70383D98A3B7D2DC15FCBC70C925AD6AFFFC46F8B59F868646D66623F31`. O ZIP tem SHA-256 `1342EAD93A2C5456EE10054C99BE0038ACCB7FC283CC39FE0889CC6E9C19DFA4`. O 7-Zip verificou os dois arquivos. No Windows de desenvolvimento, o executável instalado em pasta isolada coincidiu com o build release, abriu a janela do editor e foi removido pela desinstalação.

A prévia `0.5.0-alpha.14` amplia a edição de situação de mercado para jogadores como Quínio, do Braga B, com contrato do clube controlado. A variante gravada pelo Tauri passou por carregamento, avanço, regravação e recarga no CM0304 4.1.5. O instalador NSIS x64 sem assinatura Authenticode tem SHA-256 `EF2323FE1DA5D069B770C236A057F2E1B8DD71E5DEFCEF145234389293AFC70C`. O ZIP tem SHA-256 `A44A8D98F64321C99BA231FFFB6B7DF8CBB4AD6BF62C2E75344350CCA285F6BA`. O 7-Zip validou os dois arquivos. No Windows de desenvolvimento, a instalação em pasta isolada produziu o mesmo executável do build release e a desinstalação o removeu. A validação em Windows x64 limpo segue pendente.

A prévia `0.5.0-alpha.13` inclui a edição de situação de mercado em contratos mapeados, com estado anterior e novo e contagem de entradas do diário na revisão. O Tauri gravou a restrição de empréstimo de Quim, que o jogo confirmou na ficha. O instalador NSIS x64, sem assinatura Authenticode, tem SHA-256 `913F29CF2CF7E6CD1531EE6D2023960B43A8389D1BC008CC5CCF93B7487B8B88`. O ZIP tem SHA-256 `6C422BBCAB0151BA61996C046BDBD4CB49958FC877F3B114469DF37D8FF6AE89`. O 7-Zip verificou os dois arquivos e listou apenas o executável e seis componentes NSIS no instalador. No Windows de desenvolvimento, o executável instalado em pasta isolada coincidiu com o build em release. A instalação abriu o save inicial e mostrou Quim no grupo do Sporting Clube de Braga, os dois estados de mercado e uma nova entrada no diário. Desfazer esvaziou a revisão e Refazer restaurou a alteração. A desinstalação removeu o executável e os hashes da origem e da variante anterior permaneceram iguais.

A prévia `0.5.0-alpha.12` inclui a troca de camisas na ficha do clube. Os 32 cenários de interface passaram. O Tauri gravou uma cópia com dois bytes alterados, idêntica à variante validada em partida no CM0304. O instalador NSIS x64, sem assinatura Authenticode, tem SHA-256 `DCAA05D674928B31D482863910CBAC45A484028544CCE04032546DAA31A5AEC5`. O ZIP de teste tem SHA-256 `67F50241769C73103A929E825BA152C3B0897FA1D5E4E9B1843BA1499D879654`. O 7-Zip aprovou os dois arquivos e listou apenas o executável e seis componentes NSIS no instalador. No Windows de desenvolvimento, a instalação silenciosa em pasta isolada produziu um executável com o mesmo SHA-256 do build em release. O aplicativo instalado abriu o save do Braga e exibiu o formulário de troca de camisas. A desinstalação removeu o executável e preservou os hashes da origem e da variante. O teste em Windows limpo permanece pendente.

A prévia `0.5.0-alpha.11` bloqueia as ações de resultados enquanto uma busca está pendente ou vazia e acrescenta Abrir origem ao histórico. O instalador NSIS x64, sem assinatura Authenticode, tem SHA-256 `A6A86308A0C1318E252BE3D7ED5FF64A91CBD69C8CD058F4298672D3EDF94429`. O ZIP de teste tem SHA-256 `FB1A18E2B0A356F7E91FBAF4191851FF347C534EB06F6E7F0B844952EFF6334F`. Ambos passaram na verificação de integridade. No Windows de desenvolvimento, o executável instalado em pasta isolada coincidiu com o build em release. Com o PATH limitado ao Windows, o editor gravou Passe 20 para Clodoaldo, reabriu a variante e voltou ao save inicial com Passe 13 pelo histórico. A origem conservou o SHA-256 e a cópia coincidiu com a referência de Passe 20. A desinstalação removeu o executável de teste e preservou os saves.

A prévia `0.5.0-alpha.10` inclui busca exata de jogadores com `uid:` e decodificação por UID na comparação individual e na prévia de lotes. O instalador NSIS x64, sem assinatura Authenticode, tem SHA-256 `D3D7FAFF705641812ABF1A6935E6953EA5A855EF46F99598299180FA4D5F7A16`. O ZIP de teste tem SHA-256 `FCF41F392C781A89AFCCB45D2D5D93DD6DFB15CA28F3C98D9CAB211EF2864A2D`. O 7-Zip validou os dois arquivos e listou apenas o executável e seis componentes NSIS no instalador. Em instalação silenciosa isolada no Windows de desenvolvimento, o executável coincidiu com o build em release. Com o PATH restrito aos diretórios do Windows, abriu o save inicial e confirmou busca por UID exato, resultado vazio para UID ausente e erro para UID inválido. A busca por nome voltou a funcionar após o erro. A desinstalação removeu o executável e o save de origem conservou seu hash.

A prévia `0.5.0-alpha.9` protege a retomada de planos quando o save de origem mudou no disco. O instalador NSIS x64, sem assinatura Authenticode, tem SHA-256 `9A2A10C6CCE4E3049862709BD6A517DB5E2F463C11DA818D0D12A83C50D76F6D`. O ZIP de teste tem SHA-256 `C043BF0AC357B86CC8878B169187D63DE330C568260DFEBD338F7A8AA8DCD9E9`. Ambos passaram na verificação de integridade. O teste Rust de retomada de sessão e os 31 cenários Playwright passaram. O executável em release também abriu uma cópia descartável do save inicial. Após trocar o conteúdo do arquivo no disco, Retomar plano recusou o JSON pelo diálogo nativo e manteve a revisão vazia. Depois de restaurar os bytes originais, o mesmo JSON voltou à revisão com Passe de Clodoaldo de 13 para 20.

A prévia `0.5.0-alpha.8` inclui cancelamento de comparação individual, mensagens com UID e arquivo para jogadores ausentes e correções na lista de jogadores. O instalador NSIS x64 tem SHA-256 `57C5B78D6DF15E65328883BDB3325A20E397CDBAFBB7867861B4303EE6C09C1F`. O ZIP de teste tem SHA-256 `B46064EFDCA6720E53369ED79C1306FDFF87555C69DB8E998EEACA1ECD73BC15`. O 7-Zip validou o instalador e encontrou apenas o executável do editor e seis componentes NSIS. O pacote inclui a licença MIT, avisos de terceiros e o roteiro de teste. No Windows de desenvolvimento, a instalação silenciosa em pasta isolada gerou um executável idêntico ao build em release. Com o PATH limitado ao Windows, o editor abriu o save inicial, gravou Passe 20 para Clodoaldo e reabriu a variante pelo histórico. A variante coincidiu por SHA-256 com a referência de Passe 20, e a origem conservou seu hash. A desinstalação removeu o executável de teste e preservou a variante. O Windows usado já tinha WebView2.

A prévia `0.5.0-alpha.7` tem SHA-256 `63813BC2ABB6E7A4F8A325A3AF28109651793517286A5AF8A6A31A1D99D431BE` e está sem assinatura Authenticode. Em Windows de desenvolvimento, o executável instalado foi iniciado com o PATH limitado a `System32` e ao diretório do Windows. O editor abriu o save inicial, gravou uma variante com Passe 20 para Clodoaldo, reabriu a variante pelo histórico e preservou o hash da origem. O hash da variante coincidiu com a cópia de referência e a desinstalação removeu o executável de teste. O Windows usado já tinha WebView2 e ferramentas de desenvolvimento instalados.

A prévia `0.5.0-alpha.6` tem SHA-256 `747D2B2916B25D217D3DEB652BC6030CA9E85D1CD83050111973057826AC2A9B` e está sem assinatura Authenticode. No Windows de desenvolvimento, o executável instalado coincidiu com o build em release. O teste nativo abriu o save de laboratório e mostrou a restrição de empréstimo de Michel e a disponibilidade de Clodoaldo na comparação. A desinstalação removeu o executável de teste.

A prévia `0.5.0-alpha.5` tem SHA-256 `280B86B91F0BE5EAECA4AC216F096F4D2127FC07A37B363CCFEC0B25C1FC2250` e está sem assinatura Authenticode. No Windows de desenvolvimento, o aplicativo instalado mostrou os totais conferidos no CM0304 nos dois checkpoints do Fortaleza. O save inicial mostrou o saldo básico para consulta, sem formulário de edição financeira. O executável instalado coincidiu com o build em release, o 7-Zip validou o pacote e a desinstalação removeu o executável de teste.

A prévia `0.5.0-alpha.4` tem SHA-256 `BB11316CD50AEB4951CCF6A97FB7D298054BEC39CB165BC479BAEBB953D0CB7F` e está sem assinatura Authenticode. O 7-Zip validou a integridade do pacote. No Windows de desenvolvimento, o aplicativo instalado alterou Passe de Clodoaldo de 13 para 20, gravou uma variante e a reabriu pelo histórico. A ficha exibiu 20 após a recarga e o hash da origem permaneceu igual. A variante teve SHA-256 `975B316EDD6527E947115E8614EBC6BA9570869A90850C01F3109583A9E63C75`, igual ao da cópia de referência. A desinstalação removeu o executável da pasta de teste. O arquivo de avisos da alpha.4 é idêntico ao da alpha.3.

A prévia `0.5.0-alpha.3` tem SHA-256 `3D7113C9293410762E50E5D135BB3F99D687043485D0B57DF46C729EE0C287C4` e está sem assinatura Authenticode. O 7-Zip validou o arquivo e listou o executável e seis recursos NSIS. No Windows de desenvolvimento, a instalação silenciosa gravou um executável com o mesmo hash do build em release. O aplicativo instalado abriu os dois saves pelos recentes e carregou a ficha de Clodoaldo em ambos. A desinstalação removeu o executável da pasta de teste. O empacotamento usou o cache local do NSIS após uma falha de DNS na tentativa de download e emitiu o aviso de metadados do atualizador.

O Tauri e o NSIS geraram o instalador x64 da prévia `0.5.0-alpha.2`, com SHA-256 `DAD2079473645036923CC50B35BA6CEB6DCC753D1A3BA13AF9958EDCDC8D2DE6`. O 7-Zip confirmou a integridade e listou somente `cm0304-desktop.exe` e seis recursos do instalador. O build emitiu um aviso sobre os metadados usados pelo atualizador do Tauri.

No Windows de desenvolvimento, o pacote passou instalação silenciosa sem atalhos em uma pasta temporária do projeto. O registro do usuário exibiu a versão `0.5.0-alpha.2`. O SHA-256 do executável instalado coincidiu com o do binário em release. O executável instalado abriu a janela **CM0304 Save Editor**. A desinstalação removeu o executável e o registro da instalação de teste.

### Verificação opcional em Windows limpo

Use uma máquina ou VM Windows x64 sem instalação anterior do editor e sem Rust ou Node.js. Leve o instalador e, separadamente, uma cópia do save de teste. Para conferir a carga e a evolução da carreira, use o CM0304 4.1.5.

1. Registre a versão do Windows, a presença do WebView2 antes da instalação e o SHA-256 do instalador. Se o WebView2 estiver ausente, observe se o instalador consegue obtê-lo e concluir a instalação.
2. Abra o editor pelo menu Iniciar e carregue a cópia do save de teste. Escolha um jogador de campo. Registre o hash da origem, o nome e clube do jogador e o valor atual de um atributo técnico editável, como Passe.
3. Escolha um valor diferente que o editor aceite para o atributo, confira na revisão e grave uma nova cópia. Reabra a cópia no editor e confira o valor escolhido. Compare o hash da origem com o registrado no passo anterior.
4. Carregue a cópia no CM0304, confira o jogador, avance sete dias, grave com outro nome e recarregue. Registre qualquer recálculo observado.
5. Desinstale o editor e confirme a remoção do executável e a preservação dos saves. Anote a etapa e o texto de qualquer erro.

Esta verificação é opcional para a v1.0.

### Beta em outra carreira

Extraia o ZIP e execute o instalador x64 no Windows. O editor usa o Microsoft Edge WebView2 Runtime. Depois de abrir o aplicativo, confira a versão mostrada no rodapé. Para testar a abertura no editor, use um save `.cm4` gravado sem compactação no CM0304 4.1.5.

Use um save próprio do CM0304 4.1.5 e anote a data da carreira, o clube e as ligas ativas. Separe um jogador de campo, um membro da comissão e um clube com orçamento editável. Registre os valores mostrados no editor e no jogo antes de editar. Use uma cópia de trabalho e conserve o save de origem.

| Caso | O que executar | Resultado a registrar |
|---|---|---|
| Jogador | Altere um atributo técnico e outro de um grupo diferente. Se alterar CA, escolha a política de conversão no formulário. | Valores anteriores e novos, incluindo os efeitos indiretos mostrados na revisão. |
| Carreira | Edite salário e vencimento no vínculo escolhido. Quando houver empréstimo, identifique o clube de cada vínculo. Se houver lesão ou suspensão suportada, prepare a remoção de uma ocorrência. | Alterações no vínculo ou ocorrência selecionados e valores dos demais vínculos após recarga. |
| Disciplina | Se a variante removeu uma suspensão, confira a ficha no primeiro carregamento e após a próxima partida do clube. Se o jogo gerar nova suspensão para esse jogador, grave antes de ele cumpri-la e recarregue. | Ausência da punição removida na abertura. Se houver nova punição, registre a data, a competição e o que o jogo mostra após recarregar. |
| Vencimentos do clube | Na ficha do clube, abra **Contratos do clube**, compare dois prazos e filtre por tipo de vínculo. Repita após preparar uma alteração de salário ou vencimento. | Quantidade de vínculos e soma semanal dos salários exibidos antes e depois da revisão. |
| Comissão e clube | Edite um atributo de staff e um orçamento do clube. Use **Clubes** para navegar entre a ficha, o elenco e a comissão. | Atributo e orçamento antes e depois. Se o orçamento estiver indisponível, registre o clube e a mensagem exibida. |
| Lote | Selecione pelo menos três jogadores, aplique um ajuste de recuperação e retire um deles da prévia. Adicione o lote, desfaça e refaça antes de salvar. | Jogadores incluídos, excluído e valores finais. A revisão deve manter as alterações das outras áreas. |
| Lote de comissão | Na ficha do clube, abra **Comparar comissão técnica → Editar comissão em lote**. Selecione dois membros, prepare um atributo profissional e confira a prévia antes de incluir o lote. | Membros, valores anteriores e novos e resultado após recarga. Se houver pessoa com dupla função, confira também sua ficha de jogador. |
| Mercado | Na aba **Contratos** de um jogador com vínculo do clube controlado, selecione uma transição disponível em **Situação de mercado** e grave uma variante. | Estado antes e depois, clube do vínculo e presença de propostas já abertas. Confira a situação do jogador na abertura e após a evolução no CM0304. Se o seletor estiver ausente, registre a situação que aparece para consulta. |
| Listas de observação | Crie uma lista em **Jogadores**, adicione um jogador, ative o filtro de observação e reabra o editor. Alterne entre a lista nova e a padrão. | Nome da lista ativa após reinício, resultado do filtro por lista e confirmação de que a seleção de jogadores é independente. |
| Gravação | Salve uma variante, reabra pelo histórico, compare o elenco com a origem e confira o hash da origem. | Hashes, valores após recarga e diferenças apresentadas. Registre qualquer mudança fora da revisão. |
| Retomada do plano | Guarde o plano, feche o editor e reabra o save original. Use **Retomar plano** e confira a revisão. Experimente abrir o mesmo plano com outro save, sem edições pendentes. | Quantidade de alterações recuperadas, valores antes e depois e mensagem exibida com o save diferente. |
| Responsividade | Anote quanto tempo o editor leva para abrir o save, retornar uma busca combinada, abrir a revisão de um lote e confirmar a gravação. Anote a quantidade de jogadores do lote. | Tempos aproximados, estado mostrado durante a espera e se o editor voltou a responder após concluir ou cancelar a operação. |
| Troca de save | Com uma alteração na revisão e outra no formulário, clique em **Abrir save** e escolha **Voltar** na confirmação. Repita, escolha **Continuar** e cancele no seletor de arquivos. | **Voltar** mantém o seletor fechado. Após cancelar no seletor, confira a mesma carreira, a alteração na revisão e o valor no formulário. |
| Outras confirmações | Ao sair de uma ficha com alterações no formulário, escolha **Voltar**. Faça o mesmo ao tentar substituir um plano pendente ou excluir uma lista de observação criada para o teste. | A ficha e seus campos continuam disponíveis, a revisão não muda e a lista continua selecionada. |
| Fechamento do editor | Com um campo em edição ainda fora da revisão, use o botão de fechar da barra de título e escolha **Voltar**. Repita com uma alteração adicionada à revisão. Por fim, confirme **Fechar editor** numa sessão de teste. | O diálogo aparece nos dois casos. **Voltar** preserva os valores e **Fechar editor** fecha a janela. |
| Ciclo no jogo | Abra a variante no CM0304, confira os valores, avance sete dias ou até a próxima partida, grave com outro nome e recarregue no jogo e no editor. | Datas e eventos ocorridos, valores persistidos, recálculos e mensagens de erro. |

Para cada linha, registre **passou**, **falhou** ou **indisponível na amostra**. Quando falhar, inclua o passo, os valores esperado e observado e a mensagem exibida. O teste independente é opcional para a v1.0. Use outra amostra para cobrir casos indisponíveis se for ampliar a validação.

Inclua no relatório a versão do editor, a versão do jogo, a versão do Windows e a data inicial da carreira. Para comparar o SHA-256 da origem antes e depois, rode o mesmo comando PowerShell nos dois momentos, substituindo o caminho pelo do arquivo original.

```powershell
Get-FileHash -LiteralPath 'C:\caminho\carreira.cm4' -Algorithm SHA256
```

O relatório pode ser compartilhado sem anexar o save. Registre os hashes, as etapas e os valores necessários para reproduzir o resultado.

### Evidências locais do beta

O CM0304 abriu a variante de três campos do beta.13 em 20/09/2003 às 0h. Artur Jorge e Pedro Costa mostraram Passe 14, Técnica 12 e moral **Muito Bom** (18 na releitura). Após sete dias, o jogo gravou `28.cm4`, SHA-256 `66daa25a11edcd16971fe4c670882dd545e3ab8d56bb489b11bf94e6ceac726e`. Após reiniciar e retomar a carreira em 27/09 às 0h, ambos mantiveram Passe 14 e Técnica 12, e a moral passou para **Excelente** (19) durante a simulação. A releitura confirmou CA 117 e 91, respectivamente, e o SHA-256 da variante de entrada permaneceu igual.

No beta.13 empacotado, a comparação da comissão do Braga filtrou **Treinador adjunto**, ordenou treino de campo e abriu a ficha de Rui Águas. O WebView2 mostrou Rui (UID 4950, valor 12) seguido de José Manuel (741470, valor 8), os mesmos UIDs e valores do leitor Rust. No mesmo save, o editor mostrou seis linhas na prévia de um lote de Passe, Técnica e Moral para Artur Jorge e Pedro Costa. A gravação `lote3campos.cm4` (SHA-256 `41eed3d61c63361e58647d41134a38687ff1ac6b176d2fe686d964cdf99d34dc`) foi relida com Passe 14, Técnica 12 e Moral 18 para ambos. A CA de cada jogador ficou igual e só mudaram os seis bytes esperados. O SHA-256 da origem permaneceu `9014002d73d5db1e3861e141f1746c6053f05ddd0b70dbc1ff1a013851e6f5a8`.

Em uma compilação local posterior ao pacote beta.10, o WebView2 recusou um `.cm4` reduzido a 8.192 bytes e conservou o save ativo, a revisão com dois ajustes de Passe e o rascunho de Técnica. O teste anterior à correção encontrou um rascunho de jogador que permanecia ativo mesmo após confirmar **Descartar** na navegação para a revisão. Com a correção, a confirmação limpou o rascunho e permitiu gravar sem novo pedido de descarte. A cópia resultante teve o mesmo SHA-256 da cópia de controle que continha apenas os dois ajustes de Passe, `ed0371b7d10fa57fd5e0036a703741a39f2235d67cc507a2b371b37f738eb79e`. A origem permaneceu intacta. A regressão também passou na suíte de interface com IPC simulado.

No beta.10 empacotado, **Clubes → Braga → Editar jogadores em lote → Copiar atributos** usou Fredrik (UID 1873) como origem e selecionou somente Passe. A prévia e a revisão registraram 10 → 14 para Artur Jorge (1848) e Pedro Costa (726086). O WebView2 gravou `copiapasse14.cm4` por IPC real, SHA-256 `ed0371b7d10fa57fd5e0036a703741a39f2235d67cc507a2b371b37f738eb79e`. O leitor confirmou Passe 14 em ambos com as respectivas CAs 117 e 91, Técnica 11 e 10, e contratos e estado iguais aos da entrada. Houve apenas duas diferenças binárias, nos bytes de Passe dos destinos. O perfil de Fredrik permaneceu intacto. O SHA-256 da origem permaneceu `9014002d73d5db1e3861e141f1746c6053f05ddd0b70dbc1ff1a013851e6f5a8`.

O WebView2 nativo do beta.10 preparou duas operações separadas para Quim, condição a 70,01% e ritmo a 95,03%. A revisão mostrou as duas. Em **Operações desta sessão**, retirar a primeira devolveu a condição ao valor da origem e preservou o ajuste de ritmo. **Desfazer** restaurou as duas e **Refazer** retirou novamente a primeira. A variante `somente-ritmo9503.cm4` (SHA-256 `5fea1a94ad5fd5781a9189f108cd1ca2d4a33355fafabe56d00c54c984c38c36`) manteve a condição em 97,00% e gravou ritmo a 95,03%. Só mudaram os bytes de ritmo, nos offsets 15.502.321 e 15.502.322. O SHA-256 da origem permaneceu igual.

No executável extraído do NSIS `0.9.0-beta.10`, o WebView2 carregou uma cópia de `24.cm4`. Em **Clubes → Braga → Editar jogadores em lote**, o teste selecionou Quim (UID 1855), guardou e recarregou um preset de condição a 70,01% e conferiu a prévia e a revisão por IPC real. **Salvar variante na pasta do save** gravou `condicao7001.cm4`, SHA-256 `ba8b6eae6a2c03d39693f1ea2464569d71fe27d8772d46e264534805ccb734db`. A releitura Rust confirmou 9.700 → 7.001 no campo de condição e manteve o ritmo em 10.000. A comparação binária encontrou apenas os dois bytes de condição alterados, nos offsets 15.502.325 e 15.502.326. O SHA-256 da origem permaneceu `9014002d73d5db1e3861e141f1746c6053f05ddd0b70dbc1ff1a013851e6f5a8`.

Na alpha.29, o núcleo Rust gravou uma variante de controle com o empréstimo de Fredrik ao Braga a £2.000 por semana e vencimento em 17/09/2003, exatamente 60 dias após a data do save inicial. A releitura de todos os vínculos do Braga encontrou zero empréstimos nos filtros de 30 e 60 dias na origem. Na cópia, encontrou zero em 30 dias e um em 60, com subtotal de £2.000 por semana. O contrato de Fredrik com o Porto ficou idêntico e a origem preservou seu SHA-256. No executável extraído do NSIS, com WebView2 e IPC nativos, a ficha do Braga confirmou lista vazia em 30 dias e um vínculo em 60 dias, com Fredrik, tipo Empréstimo, vencimento em 17/09/2003 e subtotal de £2.000 por semana. O hash do executável extraído coincidiu com o do build em release.

A partir do painel do Braga, o atalho do vencimento abriu o cartão de empréstimo de Fredrik. A revisão alterou o salário semanal de £2.000 para £2.001. Ao retornar ao painel, a lista de 90 dias conservou um vínculo e passou a exibir subtotal de £2.001. A folha projetada mensal passou de £147.548 para £147.552. Em seguida, o diálogo nativo gravou `braga-prazos-2001.cm4`. A releitura Rust confirmou £2.001 por semana e o mesmo vencimento no Braga, com o contrato do Porto intacto. A comparação binária encontrou um byte alterado no offset 12148800. SHA-256 da cópia `d271a917cf1a9ab34ddd4a1a1c597a5ac443a66b522799a0054070a0fe70d152`. O save carregado preservou o SHA-256 `d542a7f6fa4123894a8647b4764fe98e19bac08dea0a6aea5135a60dfda99662`.

### Procedência e licenças do pacote

O executável instalado do beta.7, iniciado com o PATH restrito a `%WINDIR%\System32` e `%WINDIR%`, abriu o save inicial, gravou Passe 20 para Clodoaldo e reabriu a cópia com o valor esperado pelo IPC real. `node`, `cargo` e `rustc` estavam indisponíveis nesse PATH. A desinstalação removeu o executável e preservou os SHA-256 da origem e da variante. SHA-256 da variante `975B316EDD6527E947115E8614EBC6BA9570869A90850C01F3109583A9E63C75`. O ensaio ocorreu no Windows de desenvolvimento, com WebView2 já presente.

No beta.7, o IPC real aceitou o cancelamento de uma gravação mista ainda ativa. A chamada de gravação retornou sem cópia, a pasta de teste conservou apenas o arquivo de origem e o perfil do WebView2, e a revisão permaneceu idêntica. A gravação seguinte da mesma revisão produziu o SHA-256 esperado `BDBE596213F48872BDF418EE45AAC693A114CB735DAE44EF7716C81C9AA6A665`. O hash da origem permaneceu igual. O ensaio acionou o cancelamento por IPC.

Em um segundo ensaio com o executável de release beta.7, o botão de fechar da barra de título foi acionado durante a gravação por IPC real. O Tauri emitiu `save-close-blocked` enquanto a segunda gravação estava ativa. A janela permaneceu aberta, a gravação terminou e a cópia teve o mesmo SHA-256 da variante da CLI. A origem também conservou seu hash.

No executável de release beta.7 com WebView2 isolado e IPC real, o save inicial abriu em 437 ms e uma gravação mista levou 1.344,5 ms. Em uma segunda gravação, o IPC `close_editor` retornou “Aguarde a gravação da cópia antes de fechar o editor” enquanto `save_copy` ainda estava pendente. A gravação terminou e o fechamento pelo mesmo comando funcionou em seguida. As duas cópias coincidiram com o SHA-256 da CLI e a origem permaneceu intacta. O ensaio acionou o comando IPC diretamente. A verificação do botão da barra de título durante a gravação continua pendente.

Na gravação de um plano válido, a CLI beta.7 aplicou Passe 20 a Clodoaldo e removeu a lesão de Peter Hoekstra na mesma cópia. A releitura independente confirmou os dois resultados e o hash da origem permaneceu igual. A transação alterou um byte e removeu 38. SHA-256 da cópia `BDBE596213F48872BDF418EE45AAC693A114CB735DAE44EF7716C81C9AA6A665`.

Em uma pasta isolada do Windows de desenvolvimento, o instalador do beta.7 produziu um executável com SHA-256 `7E6B9918B9D10DA6F36951571CD6243F6CD4711528DDFFACD9F3B1A1750B396A`, igual ao build em release. O desinstalador removeu o executável e preservou uma cópia de save pelo SHA-256. A tentativa dentro do sandbox retornou um erro de realocação de DLL ao iniciar o instalador. A repetição autorizada fora do sandbox concluiu a instalação e a desinstalação.

O `0.9.0-beta.7` recusa planos JSON com operações desconhecidas. O ensaio da CLI misturou uma edição válida e uma operação desconhecida e confirmou a recusa sem publicação de cópia. O empacotador conferiu as quatro entradas do ZIP por SHA-256. SHA-256 do instalador `AEE388EAAEF531E08DAC941048A3078E99DD895C63C98DA5F8E650E50F13BC11`. SHA-256 do ZIP `AC761D57D5E14CDDAAB176F2B764521AD4F8E713C3E21CA74479FB92004A4934`.

O `0.9.0-beta.6` inclui a substituição de blocos com validação de fronteiras e conferência direta do payload. O 7-Zip aprovou o instalador NSIS e o ZIP. SHA-256 do instalador `D6F4913E196DCD824F7EE8387E3320E0DE556F07276D84E70B5BF90458009988`. SHA-256 do ZIP `86AF78E517F0D09E4E6BFE08899262D3E6E99458FCA45850B2DB4F771353474D`.

O executável do beta.6 extraído do NSIS coincidiu com o build de release, SHA-256 `1B80EAC86AE82B9BF90A223ABB88D79E616C7A9ABBE1E63354187779A7A4BADE`. `dumpbin /HEADERS` confirmou a arquitetura x64. `dumpbin /DEPENDENTS` listou bibliotecas de sistema e APIs do Universal CRT, sem importação direta de `node.dll`, `vcruntime*.dll` ou `msvcp*.dll`. O aplicativo usa o WebView2 em execução. O NSIS está sem assinatura Authenticode, como indicado no roteiro do pacote. A instalação em Windows limpo segue pendente.

No Windows de desenvolvimento, a instalação isolada do beta.6 produziu o executável com o mesmo SHA-256. Com o `PATH` restrito a `%WINDIR%\System32` e `%WINDIR%`, sem `node`, `cargo` nem `rustc` disponíveis nesse ambiente de execução, o aplicativo abriu a janela e encerrou normalmente pelo botão de fechar. A desinstalação removeu o executável e preservou uma cópia do save de teste por SHA-256. A verificação do ZIP também confirmou seus quatro arquivos, a licença MIT e os avisos gerados pelo `Cargo.lock`.

No executável extraído do NSIS beta.6, com IPC nativo, a ficha de Peter Hoekstra mostrou entorse no tornozelo. A remoção foi adicionada à revisão e gravada em `beta6-hoekstra.cm4`. A releitura Rust confirmou a lista vazia, e a comparação verificou 38 bytes removidos de `game_db.dat`, os outros 86 blocos intactos e o SHA-256 da origem preservado. SHA-256 da cópia `ccd816f4b515e1d4ccb3c29b0b15075ab71d50347cb3c62d72c3b1942e2e822f`.

A comparação com `tauri_heal_v05.cm4`, que o CM0304 abriu e mostrou Hoekstra sem lesões, encontrou apenas o byte de moral de Hoekstra, 10 na cópia do beta.6 e 20 na referência. Repetir a remoção junto com moral 20 pela CLI beta.6 reproduziu o SHA-256 da referência, `5d03bbb02d1f7c472cb3bd57f1fd5ecc6fd05177a3edf5410e5661920fccda68`, com a origem intacta. [Conferência anterior no jogo](../reports/v0.5-transactions.md)

O `0.9.0-beta.5` protege também os rascunhos ao fechar a janela. O 7-Zip aprovou o instalador NSIS e o ZIP. SHA-256 do instalador `37AAE0EAA89ED412AFA8DD9B7055F5461FECF76C157041065124F8E37DAF7886`. SHA-256 do ZIP `742132C30DC8CCD5C11FDBB4CEDF7D8D4EED91D62A06577AB225F00CD777F078`.

No WebView2 do executável extraído do NSIS beta.5, o botão de fechar do Windows abriu o diálogo com Cruzamentos 15 de Sandy Martens ainda no formulário e a revisão vazia. **Voltar** preservou o rascunho. O botão voltou a abrir o diálogo com a alteração já na revisão, e **Fechar editor** encerrou a janela. O save de controle conservou seu hash. O ensaio com gravação ativa permanece pendente.

Uma cópia de laboratório de 236.381.109 bytes, com um bloco opaco adicional, foi aberta pelo beta.5. O diálogo nativo gravou Cruzamentos 15 de Sandy Martens. A releitura confirmou o atributo, 87 blocos intactos e somente `game_db.dat` alterado. SHA-256 da cópia `e1ca3630401d85a218f50fc5c7e1c98c02c38993be8ef13c426b058ed8db4971`; a origem foi preservada. Apesar do arquivo maior, o fechamento chegou depois da gravação e abriu a confirmação para o plano pendente. A verificação do fechamento com gravação ativa ainda requer uma janela de tempo controlada.

A regressão Rust do desktop verifica a regra usada pelo evento de fechamento e pelo comando final da janela. O sinal de cancelamento mantém a tarefa ativa até que o trabalhador libere a gravação. A regra só libera o fechamento depois disso. O teste controla o estado da tarefa em memória.

O `0.9.0-beta.4` amplia o diálogo do editor para rascunhos, planos, listas, grupos, presets e buscas salvas. O 7-Zip aprovou o instalador NSIS e o ZIP. SHA-256 do instalador `1F3ACD41C71390A8739BA4C5A7A38EB9BC6706508CD26CFFC6E60050618FA038`. SHA-256 do ZIP `7432CF6F07B7405726E89DA8DF016E2183DE1E5663DCF2D2A0FEA151973EBFE8`.

No Windows de desenvolvimento, a instalação silenciosa do beta.4 em pasta isolada produziu o executável de release com SHA-256 `49FBBBC213410ABDC976EF323060D4B53EE1B8A6AA47DBE6BDC4D0FB0D4D61A8`. A desinstalação removeu o executável e preservou o hash da cópia de controle `.cm4`.

No WebView2 do executável extraído do NSIS beta.4, selecionar Clubes com Cruzamentos 15 de Sandy Martens ainda no formulário exibiu **Descartar rascunhos?**. **Voltar** manteve a ficha e Cruzamentos 15. **Descartar** abriu Clubes e, ao voltar à ficha, Cruzamentos retornou a 14. O executável extraído coincidiu com o build de release por SHA-256.

O mesmo beta.4 recusou uma variante de laboratório com contêiner válido e versão interna de `game_db.dat` alterada de 147 para 148. O aviso em português mostrou a versão encontrada e a esperada. A carreira ativa ficou aberta com Drible 15 de Sandy Martens no formulário. O save da carreira e a variante recusada conservaram seus hashes.

O `0.9.0-beta.3` traz o diálogo de confirmação antes do seletor de arquivos. O 7-Zip aprovou o instalador NSIS e o ZIP. SHA-256 do instalador `DED61688B3C5700587380990D1C50C1901A0D6A5086EAADC623711DB1DEAB473`. SHA-256 do ZIP `200D7330421F29EDF940FFE80374984D014DC46CF37B9D47822B2EB2C8BEEF93`.

No executável extraído do NSIS beta.3, o WebView2 exibiu **Abrir outro save?** antes de abrir o seletor. **Voltar** manteve o seletor fechado. Na segunda tentativa, **Continuar** abriu o seletor nativo e Esc cancelou a abertura. A carreira `braga-prazos-2001.cm4` continuou aberta com Cruzamentos 15 de Sandy Martens na revisão e Drible 15 no formulário. O executável extraído tinha a mesma versão e o mesmo SHA-256 do build de release.

O `0.9.0-beta.2` reúne as correções de troca de save e inclui o roteiro para conferi-las. O 7-Zip aprovou o instalador NSIS e o ZIP. SHA-256 do instalador `A9FB34823349208964620D475F3C780D13E49671DD290784D370585BBF5C659B`. SHA-256 do ZIP `206D613E417AE8246605D882D2C6439DFB13E6B8AB35C7539110516BEC687919`.

No WebView2 nativo do beta.2, cancelar a abertura preservou Cruzamentos 15 de Sandy Martens na revisão e Drible 15 como rascunho no formulário. O seletor nativo abriu sem exibir a confirmação esperada. O código em desenvolvimento substitui a confirmação por um diálogo do editor. A regressão da interface validou que **Voltar** mantém o seletor fechado e **Continuar** permite a abertura, preservando a sessão se ela falhar ou for cancelada. Falta repetir o ensaio no próximo executável empacotado.

O primeiro pacote de estabilização é o `0.9.0-beta.1`. O instalador NSIS e o ZIP passaram na verificação do 7-Zip. SHA-256 do instalador `637DA21EE6912495309A71C22DF43C89155E85DFD45968F203A07868B4CFF315`. SHA-256 do ZIP `5B60963E72BFFCA90602EF320FE12ED04D90E35608FC71C429704A41E028E360`. A versão do executável foi conferida pelos metadados do Windows.

No Windows de desenvolvimento, o instalador silencioso do beta.1 gerou em pasta isolada o mesmo executável do build de release, com SHA-256 `BC9A9BB9659781EF5D06CCC5ED4D56B91BB421072FEEA5A7B6EBC2FE7B7620CC`. A desinstalação removeu o executável e preservou a cópia de teste do save com o mesmo SHA-256. A conferência do ZIP validou a lista de quatro arquivos, a licença MIT e os avisos de terceiros. A listagem do NSIS confirmou apenas o executável do editor e seis recursos de instalação.

Na alpha.29, a verificação do ZIP confirmou seu SHA-256 `693072A84D8C060965A982687ED387D9981DACBF38A84E3BCF187DB211289DEE`, a licença MIT idêntica à do repositório e os avisos reproduzíveis pelo `Cargo.lock`. O NSIS tem SHA-256 `B3A4160776490086D384D10789335155E316290C86BFFA8E9B04121EF7FBF295`. O ZIP tem quatro arquivos, e o NSIS contém somente o executável do editor e seis componentes de instalação.

Na alpha.27, o ZIP foi conferido contra a lista exata de quatro arquivos esperados. A licença MIT nele tem SHA-256 `F6310BD40C037B4709F4A993E9824FA601BADB94A1B971C933796D0DF38EBEED`, igual à licença no repositório. Os avisos de terceiros coincidiram com os gerados pelo `Cargo.lock` e pelos textos suplementares verificados. Dentro do NSIS, o executável corresponde ao build de release por SHA-256 e os seis arquivos restantes são componentes do instalador. A inspeção também confirmou que os saves locais permanecem fora dos arquivos rastreados pelo Git.

Na auditoria da alpha.22, os avisos dentro do ZIP coincidiram com os gerados a partir do `Cargo.lock` para Windows x64. As 227 dependências externas do inventário têm licença declarada. O ZIP contém somente o instalador, os avisos, o roteiro e a licença MIT. O 7-Zip listou apenas o executável do editor e seis recursos do NSIS dentro do instalador.

O ícone do editor é gerado por `tools/make-icon.js`. O instalador NSIS inspecionado contém o executável do editor e os componentes do instalador. O jogo, os saves e os arquivos de pesquisa ficam fora do pacote e do Git. A referência de engenharia de formato é creditada em [Investigação do banco interno](../research/FORMAT.md).

O inventário preliminar do pacote Windows x64 foi levantado com `cargo metadata --offline --locked --format-version 1 --filter-platform x86_64-pc-windows-msvc`, seguindo as dependências normais a partir de `cm0304-desktop`. O grafo inclui 227 crates externas, todas com um campo de licença declarado no manifesto. As declarações incluem MIT, Apache-2.0, Unicode-3.0, MPL-2.0, BSD-3-Clause, Zlib e alternativas. Esse grafo também pode incluir crates usadas durante compilação, como macros procedurais.

Para reproduzir o inventário, execute `node tools/windows-dependency-licenses.js` na raiz. A saída JSON identifica nomes, versões, origem, licenças declaradas e arquivos de aviso encontrados no pacote local. Cinco crates ficaram sem arquivo de aviso identificado no conteúdo do pacote ou no manifesto: `alloc-stdlib`, `selectors`, `webview2-com`, `webview2-com-macros` e `webview2-com-sys`. A busca recursiva encontrou três avisos adicionais nas demais dependências.

Para reunir os textos encontrados em um arquivo de revisão, execute `node tools/windows-dependency-licenses.js --notices > target/windows-third-party-review.md`. A saída inclui as 227 crates, os textos locais de 222 delas e uma indicação de revisão pendente nas cinco restantes. Revise esse arquivo junto com os avisos do instalador antes de preparar o material de distribuição.

Use as fontes a seguir para os cinco pacotes sem texto no cache. As revisões vieram dos arquivos `.cargo_vcs_info.json` dos pacotes fixados no Cargo.lock.

| Pacote | Fonte para o aviso |
|---|---|
| `alloc-stdlib` 0.3.0 | [Texto BSD-3-Clause da revisão publicada](https://github.com/dropbox/rust-alloc-no-stdlib/blob/0a81fd6928ea3b33c8cd484aa4575d50ffb98012/LICENSE) |
| `selectors` 0.38.0 | [Manifesto da revisão publicada](https://github.com/servo/stylo/blob/572ecba2d1600e7c3d490586692a209faf703baa/selectors/Cargo.toml) e [texto MPL-2.0 da Mozilla](https://www.mozilla.org/en-US/MPL/2.0/) |
| `webview2-com` e `webview2-com-sys` 0.39.1 | [Texto MIT da revisão publicada](https://github.com/wravery/webview2-rs/blob/edc2caf886175ccaebe86078c9cfe1ae2a187328/LICENSE) |
| `webview2-com-macros` 0.8.1 | [Texto MIT da revisão publicada](https://github.com/wravery/webview2-rs/blob/dffa41a8a46d3f5565eefbff2de57d38d399f158/LICENSE) |

O cache local do empacotador contém `NSIS/COPYING`, que declara os termos do NSIS e de seus módulos de compressão. O SHA-256 do arquivo é `388357c1215ff403c5ebde3a5ecd273e68f8b79a579996775245d1ee65442aba`. O plug-in `nsis_tauri_utils.dll` extraído do instalador alpha.3 tem SHA-1 `D0C502F45DF55C0465C9406088FF016C2E7E6817`, igual ao da versão 0.5.2 fixada pelo Tauri CLI 2.9.6. O repositório do plug-in publica os textos MIT e Apache-2.0. [Versão e hash no empacotador](https://github.com/tauri-apps/tauri/blob/tauri-cli-v2.9.6/crates/tauri-bundler/src/bundle/windows/nsis/mod.rs#L2010) · [Licenças do plug-in](https://github.com/tauri-apps/nsis-tauri-utils#license)

O arquivo `AVISOS-DE-TERCEIROS-0.5.0-alpha.3.md` foi gerado ao lado do instalador, com SHA-256 `8D28204109EF3BC2A3004A9546CA33CE7EA26F2A33879CE224D47F71DA0F0781`. Ele contém as 227 entradas Rust e os textos do NSIS e do plug-in Tauri. Para reproduzir, reúna os arquivos abaixo em `target/notice-supplements` e execute `node tools/windows-dependency-licenses.js --release-notices target/notice-supplements`. O gerador confere o SHA-256 dos arquivos suplementares e interrompe a geração se um texto estiver ausente ou divergente.

| Nome local | Origem do texto |
|---|---|
| `alloc-stdlib.txt` | [Dropbox](https://raw.githubusercontent.com/dropbox/rust-alloc-no-stdlib/0a81fd6928ea3b33c8cd484aa4575d50ffb98012/LICENSE) |
| `selectors.txt` | [Mozilla](https://www.mozilla.org/media/MPL/2.0/index.815ca599c9df.txt) |
| `webview2.txt` | [webview2-rs](https://raw.githubusercontent.com/wravery/webview2-rs/edc2caf886175ccaebe86078c9cfe1ae2a187328/LICENSE) |
| `webview2-macros.txt` | [webview2-rs](https://raw.githubusercontent.com/wravery/webview2-rs/dffa41a8a46d3f5565eefbff2de57d38d399f158/LICENSE) |
| `nsis-copying.txt` | Arquivo `NSIS/COPYING` do cache do empacotador |
| `nsis-tauri-utils-mit.txt` | [MIT do plug-in 0.5.2](https://raw.githubusercontent.com/tauri-apps/nsis-tauri-utils/nsis_tauri_utils-v0.5.2/LICENSE_MIT) |
| `nsis-tauri-utils-apache.txt` | [Apache-2.0 do plug-in 0.5.2](https://raw.githubusercontent.com/tauri-apps/nsis-tauri-utils/nsis_tauri_utils-v0.5.2/LICENSE_APACHE-2.0) |

O código do editor usa a [licença MIT](../LICENSE). Os arquivos de referência Delphi consultados trazem créditos e instruções nos respectivos readmes e permanecem fora do pacote.

Para gerar o ZIP para os testadores depois de compilar o instalador, execute `pwsh -NoProfile -File tools/package-windows-beta.ps1` na raiz. O script usa a versão da configuração do Tauri, inclui o instalador, o roteiro, a licença do editor e os avisos de terceiros, verifica o SHA-256 de cada entrada do ZIP e publica o pacote e o arquivo `.zip.sha256` em `target/beta`. Distribua os dois arquivos juntos. A opção `-Output` seleciona outra pasta de saída quando já existir um pacote para a mesma versão.

Com uma versão estável na configuração do Tauri, o mesmo comando gera `CM0304-Save-Editor-<versão>-windows-x64.zip` e o checksum em `target/distribution`. O ZIP inclui um guia curto de instalação, uso e recuperação do save original. A auditoria do instalador final continua prevista no roadmap.

Em uma cópia isolada do código do beta.13 com os manifests em 1.0.0, o Tauri compilou e gerou um NSIS x64. O empacotador aprovou a versão nos metadados. O 7-Zip confirmou a integridade do ZIP e do NSIS, e o executável extraído coincidiu por SHA-256 com o binário de release. O pacote de ensaio incluiu instalador, guia, avisos e licença MIT. SHA-256 do ZIP de ensaio `01904C4E2F1689E49ADB00E33184A61F4F69E39DC01220E7AAE69E8DEAEB46B3`. O NSIS estava sem assinatura Authenticode e o aviso do Tauri sobre o marcador usado pelo atualizador continuou presente. A versão do projeto permaneceu em beta.13.

O NSIS de ensaio reutilizou a pasta isolada do teste anterior do beta.13. O executável instalado coincidiu com o extraído do NSIS. No WebView2, ele abriu uma cópia do save, comparou a comissão do Braga e gravou Passe, Técnica e Moral em lote para dois jogadores. A cópia teve SHA-256 `41eed3d61c63361e58647d41134a38687ff1ac6b176d2fe686d964cdf99d34dc` e a origem manteve seu hash. A desinstalação removeu o executável e preservou o save usado no ensaio anterior. A auditoria do pacote oficial está registrada nos critérios de lançamento do roadmap.

O ensaio foi repetido com o código do beta.16 incluindo a otimização de lotes. O ZIP de ensaio tem SHA-256 `dc28b253613c9c6f509c19a488d0694476dbc3eaef3bc052fd15b37fea64c877` e o NSIS sem assinatura tem SHA-256 `354fc1cf462d46444f097dbca56f2f2a674101b9329147f5e2cde990c5730b2d`. O 7-Zip aprovou os dois arquivos. O binário extraído, a licença MIT e os avisos coincidiram com as referências. No WebView2, o binário repetiu o lote do Braga com o mesmo SHA-256 da cópia citada acima e o save de origem intacto. A versão do projeto continua em beta.16.

Com o rodapé dinâmico do beta.17, o ensaio estável gerou o ZIP SHA-256 `18fa786ea2750879eed7494fecd0f35d6fd75e7ec35b573104b24d3b11aa6509` e o NSIS sem assinatura SHA-256 `c0296cd8404b111aed805fec384ecfcdeb1b597454feb1411d5cff849e62c1f8`. A auditoria conferiu a integridade, o executável, a licença MIT e os avisos. O binário extraído exibiu Editor 1.0.0, gravou o mesmo lote de referência e preservou a origem. O pacote está na pasta isolada de ensaio; a distribuição para testadores permanece em beta.17.

Para conferir um ZIP recebido no PowerShell, execute na pasta em que estão o pacote e o arquivo de hash.

```powershell
$zip = 'CM0304-Save-Editor-0.9.0-beta.6-beta-x64.zip'
$expected = ((Get-Content -LiteralPath "$zip.sha256" -Raw) -split "\s+")[0]
$actual = (Get-FileHash -LiteralPath $zip -Algorithm SHA256).Hash
if ($actual -ne $expected) { throw 'O SHA-256 do ZIP diverge do publicado.' }
```

A validação da publicação com o arquivo de hash foi realizada com o instalador da alpha.19. O ZIP reempacotado em `target/beta-with-checksums` tem SHA-256 `8022D8950F59551C619739E7C33FCBC597D331617957CCAD0A01AD54A810C4C3`. O instalador continua com o SHA-256 registrado acima.

O roteiro dentro do ZIP também fornece um comando PowerShell que compara o instalador extraído com o SHA-256 esperado. O comando foi executado a partir do roteiro da alpha.24 reempacotada em `target/beta-verificado`, com SHA-256 `09B7BA70FC59FBCE7F55D0EC62F26F4B6BA3019DB64F22E2870A6D70D05C7F3B` para o ZIP. O executável é o mesmo da alpha.24 original.
