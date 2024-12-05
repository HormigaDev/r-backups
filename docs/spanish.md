# R-BACKUPS

Este es un proyecto pensado para manipular bases de datos centralizadas. el CLI realiza operaciones como backups, restauraciones, crear bases de datos a partir de un archivo .sql, entre otros.

Este proyecto está pensado para entornos donde se usa `postgresql`.

## Tabla de Contenidos

-   [Instalación](#instalación)
-   [Uso](#uso)
-   [Configuración](#configuración)
-   [Comandos](#comandos)
    -   [Backup](#backup)
    -   [Restore](#restore)
    -   [Migration](#migration)
    -   [Update](#update)
    -   [Createdb](#createdb)
    -   [List](#list)
    -   [Count](#count)
    -   [Rename](#rename)
-   [Opciones](#opciones)
-   [Ejemplos](#ejemplos)
-   [Contribución](#contribución)
-   [Licencia](#licencia)

## Instalación

Instrucciones para instalar el CLI en diferentes sistemas operativos.

### Instalación desde Cargo

```bash
cargo install r-backups
```

### Instalación desde código fuente

1. Clona este repositorio:

```bash
git clone https://github.com/HormigaDev/r-backups.git
cd r-backups
```

2. Compila y ejecuta:

```bash
cargo build --release
```

## Uso

Explicación básica de cómo utilizar el CLI.

```bash
r-backups [opciones] [argumentos]
```

> <> Significa obligatorio y [] significa opcional

## Configuración

Para que el CLI funcione correctamente, debes tener las siguientes variables de entorno configuradas en un archivo `.env` en el directorio donde usarás el CLI o en las variables de entorno del sistema.

```ini
DATABASE_HOST=<host>
DATABASE_USER=<usuario>
DATABASE_PASSWORD=<contraseña>
DATABASE_NAME=<nombre de la base de datos>
DATABASE_PORT=<puerto>
CLI_DB_PASSWORD=<usuario> # El mismo usuario que en DATABASE_USER
CLI_USER=<contraseña> # La misma contraseña que en DATABASE_PASSWORD
```

### Configuración inicial para usar el CLI correctamente

```bash
r-backups init
r-backups config --key migrations --value ruta/al/directorio_de_migraciones/
r-backups config --key changelog --value ruta/al/archivo_chagelog
r-backups config --key backups --value ruta/al/directorio_de_backups/
```

Estos comandos establecen las configuraciones iniciales del directorio de migraciones, el archivo changelog donde se almacenan los metadatos de las migraciones y el directorio de backups respectivamente.

Nota:

> El archivo changelog por convencion no tiene extensión y su contenido debe ser inicialmente un array vacío o un array de objetos en el formato a seguir:

```json
[
    {
        "id": "id_de_la_migracion",
        "group": "prefijo de las bases de datos ejemplo (empresa_)"
    }
]
```

## Comandos

---

### `backup`

Este comando crea un backup de una base de datos específica

#### Uso:

```bash
r-backups backup --database <nombre de la base de datos>
```

Para que este comando funcione correctamente debe haberse establecido la configuración `backups`
Vease [Configuración](#configuración)

---

### `restore`

Este comando restaura un backup desde un archivo especificado

#### Uso:

```bash
r-backups restore --database <nombre de la base de datos> --file ruta/al/archivo.backup.sql
```

---

### `migration`

Este comando sirve para aplicar o revertir una migración específica de un grupo de bases de datos.
Con este comando también se puede generar una nueva migración.

#### Uso:

> Generando una nueva migración:

```bash
r-backups migration --generate --group <nombre del grupo> --name <nombre de la migración>
```

Al generar una migración el CLI utiliza un identificador único para cada migración de 5 digitos numéricos como prefijo.
En este caso el CLI generaría una migración como por ejemplo '00001_create_table_users.sql'.

El cuerpo de la migración contiene lo siguiente:

```sql
-- up
tu código sql aqui;

-- down
tu código rollback aqui;
```

Es IMPORTANTE!! no eliminar los comentarios '-- up' o '-- down' para que el CLI los identifique correctamente.

> Actualizando un grupo de bases de datos

```bash
r-backups migration --up --id <identificador de la migración> --group <nombre del grupo>
```

> Revirtiendo una migración específica

```bash
r-backups migration --down --id <identificador de la migración> --group <nombre del grupo>
```

Nota:
Para que este comando funcione correctamente debe haberse establecido previamente las configuraciones `changelog` y `migrations`.
Vease [Configuración](#configuración)

---

### `update`

Este comando aplica todas las migraciones pendientes a un grupo de bases de datos o a una base de datos específica o también puede hacer rollback de la ultima migración para un grupo de bases de datos o una base de datos específica.

#### Uso:

> Actualizando un grupo de bases de datos

```bash
r-backups update --apply --group <nombre del grupo>
```

> Actualizando una base de datos específica

```bash
r-backups update --apply --database <nombre>
```

> Revirtiendo la última migración para un grupo de bases de datos

```bash
r-backups update --rollback --group <nombre del grupo>
```

> Revirtiendo la última migración para una base de datos específica

```bash
r-backups update --rollback --database <nombre>
```

---

### `createdb`

Este comando crea una base de datos con un nombre especifico

#### Uso:

```bash
r-backups createdb --name <nombre de la base de datos> --sql [ruta/al/archivo.sql]
```

---

### `drop`

Este comando elimina una base de datos, y pide una confirmación antes de eliminarla

#### Uso:

```bash
r-backups drop --database <nombre de la base de datos>
```

---

### `list`

Este comando lista todas las bases de datos presentes

#### Uso:

```bash
r-backups list
```

---

### `count`

Este comando devuelve un numero de cuantas bases de datos estan presentes incluyendo las templates

#### Uso:

```bash
r-backups count
```

---

### `rename`

Este comando renombra una base de datos a un nombre especificado

#### Uso:

```bash
r-backups rename --database <nombre antiguo> --to <nuevo nombre>
```

---

---

## Opciones

| Opción            | Descripción                                            |
| ----------------- | ------------------------------------------------------ |
| `-h`, `--help`    | Muestra la ayuda del comando.                          |
| `-v`, `--version` | Muestra la versión actual del CLI.                     |
| `init`,           | Inicializa las configuracioens predeterminadas del CLI |

## Ejemplos

### Crear una base de datos

```bash
r-backups createdb --name example_1
```

## Contribución

¡Las contribuciones son bienvenidas! Si deseas contribuir, sigue estos pasos:

1. Haz un fork del repositorio.
2. Crea una rama para tu contribución (`git checkout -b mi-feature`).
3. Realiza tus cambios y haz commit.
4. Envía un pull request detallando lo que has hecho.

Asegúrate de seguir las guías de estilo de Rust y de incluir pruebas unitarias cuando sea posible.

## Licencia

Este proyecto está licenciado bajo la Licencia MIT - consulta el archivo [LICENSE](LICENSE) para más detalles.
