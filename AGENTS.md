# Magnesium — instruções para agentes de código

> Leia este arquivo antes de alterar o projeto. Este documento descreve decisões do desenvolvedor, **não** autoriza implementar tarefas por conta própria. O código atual do repositório é a fonte da verdade para nomes, dependências e caminhos: confira-o antes de propor ou modificar qualquer coisa.

## Sobre o projeto

- **Magnesium** é um aplicativo desktop de telemetria para os jogos de Fórmula 1 da EA, começando pelo **F1 25**.
- Stack escolhida: **Tauri**, **Rust** no backend e **TypeScript** no frontend. Esta escolha é deliberada após experiências com Java, C#, Avalonia, WinUI, GTK, Qt etc. **Não sugira migrar de framework sem que isso seja solicitado.**
- Objetivo de plataforma: **Windows e Linux**, na medida em que os recursos usados forem compatíveis.
- O desenvolvedor **gosta de criar e controlar o frontend**. O layout é planejado no **Figma**. Ajude principalmente com boilerplate, parsers, integrações e partes trabalhosas, quando solicitadas. Não substitua designs, crie telas inteiras ou reestruture UX sem autorização.
- Visão de interface: **dashboard principal “one page”**, reunindo as informações mais relevantes sem aprofundar todos os detalhes; outras páginas poderão mostrar dados específicos. Há ícones próprios para compostos de pneus.
- Prefira alterações pequenas, explícitas, fáceis de revisar; **não transforme uma tarefa simples em uma abstração genérica desnecessária**.

## Estado e limites de escopo

- Existem/foram preparados structs do protocolo de telemetria F1 25 em Rust, organizados em **arquivos separados por Packet ID**, com `header.rs`, `parser.rs` e `mod.rs`. Localize-os no repositório e confirme seus nomes antes de editar.
- O **listener UDP** está ou estará sendo desenvolvido pelo próprio usuário. Não o reescreva, nem decida a arquitetura no lugar dele sem pedido explícito.
- O plano atual é receber datagramas UDP em uma **thread dedicada** com `std::thread` e `std::net::UdpSocket`, identificar o pacote pelo cabeçalho, decodificar em um struct e encaminhar o conteúdo ao frontend.
- A comunicação Rust → frontend deve usar mecanismos do **Tauri (Events e, apenas quando fizer sentido, Channels)**. A camada TypeScript receberá eventos e alimentará **signals** para estado reativo. Confira qual biblioteca de signals está realmente instalada; não pressuponha Preact, Solid, Angular ou outra lib.
- O objetivo inicial do encaminhamento é transmitir o pacote **decodificado em JSON** para que o frontend decida o que utilizar. Cada tipo tem seu próprio nome de evento (ex.: `udp_carstatus`, `udp_cartelemetry`). Preserve a nomenclatura existente e peça confirmação antes de renomear eventos públicos.
- A preferência é evitar um `match` com código repetido: um dispatcher explícito e pequeno é aceitável, com função auxiliar genérica para parse/emit. Não crie registro dinâmico, framework de plugins ou macros complexas sem necessidade demonstrada.

## Protocolo F1 25 — formato 2025

- **Formato alvo:** `packet_format == 2025`, protocolo UDP com **little-endian** e dados **packed**. **Não implementar o Season Pack / formato 2026 automaticamente.**
- O `PacketHeader` / `F1Header` possui **29 bytes**, no começo do **payload UDP** (offset `0x00` do datagrama recebido pelo socket). Na captura de loopback analisada, ele começava no offset `0x20` porque havia cabeçalhos de rede; **não use `0x20` dentro do listener**.
- O campo `packet_id` ocupa **um byte no offset `0x06`** do datagrama. Faça leitura/validação antes do despacho.
- O protocolo 2025 define **16 Packet IDs (0–15)**; alguns payloads contêm arrays fixos de **22 carros**. O índice do carro do jogador vem do header, não é sempre zero.
- Exemplos importantes: **ID 6 = CarTelemetryData**; **ID 7 = CarStatusData**; **ID 3 = EventData**, com código de evento ASCII de 4 bytes e detalhes variáveis.
- Os structs usam `#[repr(C, packed)]`, tipos primitivos Rust (`u8`, `u16`, `f32` etc.) e arrays de comprimento fixo. Mantenha structs de cada tipo em **arquivos separados**, com seus tipos auxiliares próximos, e importe/reexporte através de `mod.rs`.
- `PacketId` pode ser um `#[repr(u8)] enum` **para IDs validados**; porém o campo bruto recebido no `F1Header` deve continuar **`u8`**, e só depois se converte usando `TryFrom<u8>`. Um byte desconhecido não é um enum Rust válido.
- **Não** use cast de `&[u8]` para `&T` packed. Para leitura direta de um struct de valores primitivos, use uma função segura na API, que valida comprimento e emprega `read_unaligned` de forma encapsulada e justificada, ou biblioteca apropriada já existente. A leitura por cópia presume uma arquitetura little-endian; documente/valide essa premissa se necessário.
- Não crie referências a campos potencialmente desalinhados de structs `packed` (`E0793`). Copie o valor para variável local antes de formatar, serializar ou emprestar; atenção especial a `serde::Serialize` e arrays grandes.
- **Valide o comprimento real retornado por `recv_from()`**; o slice utilizável é `&buffer[..size]`. Um buffer de **29 bytes** truncaria e descartaria o restante de cada datagrama UDP; use capacidade suficiente para o maior pacote suportado e trate truncamento.
- `recv_from()` é bloqueante em `std::net::UdpSocket`, por isso fica na thread dedicada. O listener requer um loop para receber pacotes sucessivos. Projete parada/encerramento limpos se a tarefa solicitar ciclo de vida.
- Nos pacotes de tamanho fixo, valide o comprimento esperado **antes** de interpretar; os eventos do ID 3 variam de tamanho e precisam de tratamento separado.
- Fonte primária para layouts: **EA SPORTS F1 25 UDP specification**, edição 2025, não a de 2026.

### Mapa de IDs

| ID | Tipo |
|---:|---|
| 0 | Motion |
| 1 | Session |
| 2 | Lap Data |
| 3 | Event |
| 4 | Participants |
| 5 | Car Setups |
| 6 | Car Telemetry |
| 7 | Car Status |
| 8 | Final Classification |
| 9 | Lobby Info |
| 10 | Car Damage |
| 11 | Session History |
| 12 | Tyre Sets |
| 13 | Motion Ex |
| 14 | Time Trial |
| 15 | Lap Positions |

## Captura de referência / testes

- Há uma captura parcial de **TL2 em Melbourne, pilotando Antonelli/Mercedes**, usada durante o desenvolvimento. Foram extraídos arquivos `.bin` com payloads UDP e CSVs de telemetria; **eles podem não estar no repositório**, então procure-os antes de usá-los e não dependa deles para compilar.
- A captura revelou o jogador no **índice 7** naquele momento. **Isso é dado de teste, nunca hardcode.**
- Testes úteis: cabeçalho de 29 bytes; formato/versão e ID; `CarStatusData` com 55 bytes por carro e `PacketCarStatusData` com **1239 bytes**; leituras de `float` little-endian; limites do índice; eventos curtos e de tamanho variável; datagramas truncados/IDs desconhecidos.
- Evite adicionar capturas `.pcapng` muito grandes ao controle de versão sem pedido explícito.

## Como colaborar neste repositório

1. **Primeiro inspecione o código real** (`Cargo.toml`, `tauri.conf.json`, módulos Rust, `package.json` e frontend). Não presuma nomes de arquivos ou dependências com base apenas neste documento.
2. Ao receber uma tarefa, faça **somente o escopo pedido**. Preserve estruturas, nomes, design do Figma e partes que o desenvolvedor decidiu escrever por conta própria.
3. Prefira soluções idiomáticas, simples, divididas em módulos pequenos, em vez de um arquivo monolítico. Siga o estilo já existente (nomes em `snake_case` em Rust, tipos explícitos, comentários úteis sobre bytes/layout).
4. Para mudanças em structs, acrescente/ajuste verificações de `size_of::<T>()`; para alterações na leitura/roteamento, escreva testes pequenos sobre bytes sintéticos ou fixtures existentes.
5. Execute `cargo fmt`, `cargo check` e os testes relevantes **se as ferramentas e dependências estiverem disponíveis**. Não declare que testou se não conseguiu executar. Se não puder, explique o que falta.
6. Antes de adicionar crates, bibliotecas JS, padrões de eventos ou estados globais, veja se o projeto já tem uma solução. Evite dependências apenas para reduzir poucas linhas.
7. Em respostas, explique sucintamente **o que mudou, por quê, como testar e quais pontos ainda precisam da decisão do desenvolvedor**. Não invente requisitos.

## Preferências explícitas do desenvolvedor

- **“Eu desenho, você gera o boilerplate.”** Priorize assistência sob demanda; não tome a autoria da experiência visual.
- **Tauri é a stack definitiva do Magnesium neste momento.** Não reabra debates sobre Avalonia, Java, C#, Electron ou Neutralino, salvo pedido.
- **Threading** foi escolhido conscientemente para o listener UDP; não substitua por Tokio só por preferência técnica.
- **Signals** no frontend e **eventos tipados por categoria de pacote** são a direção desejada.
- **Um arquivo por tipo de pacote**. Nada de juntar 16 structs de telemetria em um único `.rs` gigante.
- **Sem comandos do Git**. Nada de commitar sem avisar, de fazer push/merge sem log. Apenas commite ou rode interações com o git quando for pedido.