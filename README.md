# R-BACKUPS

Un CLI destinado para proyectos que usan postgresql en entornos Linux.

## Instalación

Para instalar el CLI hay 2 formas.

1. Instalar desde cargo

```bash
cargo install r-backups
```

2. Clonar el repositorio

```bash
git clone git@github.com:HormigaDev/r-backups.git
cd r-backups
```

Ahora construye el binario

```bash
cargo build --release
sudo cp target/release/r-backups /usr/local/bin/
```

## Comandos

Para ver la ayuda del CLI puedes usar el siguiente comando.

```bash
r-backups help
```
