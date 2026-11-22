# Rust: núcleo e CLI

Estas instruções complementam o `AGENTS.md` da raiz para as duas crates desta pasta.

## Responsabilidades

- Mantenha regras de formato, conversão e edição em `cm0304-core`. A CLI deve consumir essas regras, sem duplicar offsets ou políticas.
- Use os leitores e abstrações binárias existentes. Valide limites, comprimentos, referências e discriminadores antes de acessar registros.
- Resolva entidades por seus identificadores e referências; nomes não identificam unicamente jogadores, contratos ou clubes.
- Preserve a separação entre bytes brutos, valores convertidos e projeções do plano. A inversa dos atributos relativos depende da CA e da classificação de posição.
- Em pessoas com dupla função, preserve os dados compartilhados e os efeitos entre jogador e staff. Confira a regra de CA usada por cada conversão.

## Escrita transacional

- Integre novos campos ao fluxo de transações, validação e relatório existente; evite caminhos paralelos de gravação.
- Preserve rejeição de conflitos, referências compartilhadas indevidas, valores inválidos e alvos não representáveis.
- Em remoções estruturais, ajuste o contêiner pelas abstrações existentes e preserve registros não selecionados. Mantenha clara a referência dos offsets registrados nos relatórios.
- Preserve temporário no diretório de destino, sincronização, releitura e finalização sem sobrescrita.
- Mantenha comandos experimentais de laboratório identificados como tais. Não exponha uma variante como suportada apenas porque a cópia foi relida pelo próprio parser.
- Ao mudar JSON de planos ou saídas da CLI, confira os consumidores no Tauri, frontend e ferramentas de paridade.

## Validação

Use testes direcionados em `cm0304-core/tests` para a regra alterada e execute `cargo test --offline` para core/CLI. Para mudanças no contrato com o desktop, inclua a verificação do workspace descrita na raiz.

Novas operações binárias devem verificar os valores relidos, a preservação da origem e dos dados não envolvidos e a rejeição de entradas inválidas. Use diretórios temporários e destinos novos; não sobrescreva fixtures reais. Não substitua evidência de evolução no jogo por um round-trip no mesmo leitor.
