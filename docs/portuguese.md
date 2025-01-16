# R-BACKUPS

Este projeto é projetado para gerenciar bancos de dados centralizados. O CLI executa operações como backups, restaurações, criação de bancos de dados a partir de um arquivo `.sql` e muito mais.

Este projeto foi pensado para ambientes onde se usa `postgresql`.

## Índice

-   [Instalação](#instalação)
-   [Uso](#uso)
-   [Configuração](#configuração)
-   [Comandos](#comandos)
    -   [Backup](#backup)
    -   [Restore](#restore)
    -   [Migration](#migration)
    -   [Update](#update)
    -   [Createdb](#createdb)
    -   [List](#list)
    -   [Count](#count)
    -   [Rename](#rename)
-   [Opções](#opções)
-   [Exemplos](#exemplos)
-   [Contribuição](#contribuição)
-   [Licença](#licença)

## Instalação

Instruções para instalar o CLI em diferentes sistemas operacionais.

### Instalar via Cargo

```bash
cargo install r-backups
```

### Instalar a partir do código-fonte

1. Clone este repositório:

```bash
git clone https://github.com/HormigaDev/r-backups.git
cd r-backups
```

2. Compile e execute:

```bash
cargo build --release
```

## Uso

Explicação básica de como usar o CLI.

```bash
r-backups [opções] [argumentos]
```

> <> significa obrigatório e [] significa opcional

## Configuração

Para que o CLI funcione corretamente, você deve ter as seguintes variáveis de ambiente configuradas em um arquivo `.r-backups` no diretório onde usará o CLI ou nas variáveis de ambiente do sistema.

```ini
DATABASE_HOST=<host>
DATABASE_USER=<usuário>
DATABASE_PASSWORD=<senha>
DATABASE_NAME=<nome do banco de dados>
DATABASE_PORT=<porta>
BACKUPS_DIR=caminho/para/diretorio_backups/
MIGRATIONS_DIR=caminho/para/diretorio_migrations/
CHANGELOG_FILE_PATH=caminho/para/arquivo_chagelog
```

### Configuração inicial para usar o CLI corretamente

```bash
r-backups init
```

Nota:

> O arquivo changelog, por convenção, não possui extensão e seu conteúdo deve ser inicialmente um array vazio ou um array de objetos no seguinte formato:

```json
[
    {
        "id": "id_da_migração",
        "group": "prefixo do banco de dados, por exemplo (empresa_)"
    }
]
```

## Comandos

---

### `backup`

Este comando cria um backup de um banco de dados específico.

#### Uso:

```bash
r-backups backup --database <nome_do_banco_de_dados>
```

Para que este comando funcione corretamente, a configuração `backups` deve estar definida.  
Veja [Configuração](#configuração).

---

### `restore`

Este comando restaura um backup a partir de um arquivo especificado.

#### Uso:

```bash
r-backups restore --database <nome_do_banco_de_dados> --file caminho/para/arquivo_backup.sql
```

---

### `migration`

Este comando é usado para aplicar ou reverter uma migração específica para um grupo de bancos de dados.  
Também pode ser usado para gerar uma nova migração.

#### Uso:

> Gerar uma nova migração:

```bash
r-backups migration --generate --group <nome_do_grupo> --name <nome_da_migração>
```

Ao gerar uma migração, o CLI usa um identificador único para cada migração, consistindo em um prefixo numérico de 5 dígitos.  
Por exemplo, o CLI pode gerar uma migração chamada `00001_create_table_users.sql`.

O corpo da migração contém o seguinte:

```sql
-- up
seu código SQL aqui;

-- down
seu código de rollback aqui;
```

É IMPORTANTE não excluir os comentários `-- up` ou `-- down` para que o CLI possa identificá-los corretamente.

> Atualizar um grupo de bancos de dados:

```bash
r-backups migration --up --id <id_da_migração> --group <nome_do_grupo>
```

> Reverter uma migração específica:

```bash
r-backups migration --down --id <id_da_migração> --group <nome_do_grupo>
```

Nota:  
Para que este comando funcione corretamente, as configurações `changelog` e `migrations` devem estar definidas.  
Veja [Configuração](#configuração).

---

### `update`

Este comando aplica todas as migrações pendentes para um grupo de bancos de dados ou um banco de dados específico.  
Também pode reverter a última migração para um grupo de bancos de dados ou um banco de dados específico.

#### Uso:

> Atualizar um grupo de bancos de dados:

```bash
r-backups update --apply --group <nome_do_grupo>
```

> Atualizar um banco de dados específico:

```bash
r-backups update --apply --database <nome_do_banco_de_dados>
```

> Reverter a última migração para um grupo de bancos de dados:

```bash
r-backups update --rollback --group <nome_do_grupo>
```

> Reverter a última migração para um banco de dados específico:

```bash
r-backups update --rollback --database <nome_do_banco_de_dados>
```

---

### `createdb`

Este comando cria um banco de dados com um nome especificado.

#### Uso:

```bash
r-backups createdb --name <nome_do_banco_de_dados> --sql [caminho/para/arquivo.sql]
```

---

### `drop`

Este comando exclui um banco de dados e exige confirmação antes da exclusão.

#### Uso:

```bash
r-backups drop --database <nome_do_banco_de_dados>
```

---

### `list`

Este comando lista todos os bancos de dados existentes.

#### Uso:

```bash
r-backups list
```

---

### `count`

Este comando retorna o número de bancos de dados presentes, incluindo templates.

#### Uso:

```bash
r-backups count
```

---

### `rename`

Este comando renomeia um banco de dados para um novo nome especificado.

#### Uso:

```bash
r-backups rename --database <nome_antigo> --to <novo_nome>
```

---

## Opções

| Opção             | Descrição                                  |
| ----------------- | ------------------------------------------ |
| `-h`, `--help`    | Exibe as informações de ajuda do comando.  |
| `-v`, `--version` | Exibe a versão atual do CLI.               |
| `init`            | Inicializa o CLI com configurações padrão. |

## Exemplos

### Criar um banco de dados

```bash
r-backups createdb --name example_1
```

## Contribuição

Contribuições são bem-vindas! Se você deseja contribuir, siga estas etapas:

1. Faça um fork do repositório.
2. Crie um branch para sua contribuição (`git checkout -b minha-feature`).
3. Faça suas alterações e faça commit.
4. Envie um pull request detalhando o que você fez.

Certifique-se de seguir os guias de estilo do Rust e incluir testes unitários sempre que possível.

## Licença

Este projeto está licenciado sob a Licença MIT. Veja o arquivo [LICENSE](LICENSE) para mais detalhes.
