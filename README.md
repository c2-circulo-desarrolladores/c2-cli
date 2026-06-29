# c2 - cli

CLI para inicializar proyectos Python e importar módulos reutilizables.

```bash
c2 <COMMAND>
```

## Comandos

| Comando | Descripción |
|--------|-------------|
| `init` | Inicializa un proyecto Python corriendo `'uv init'` y agrega `.gitignore`, `justfile` y `.github/` |
| `import` | Copia un módulo reutilizable al proyecto actual (`timer`, `logger`, `api`...) |
| `help` | Muestra ayuda |

## Instalación

### Directa
```sh
curl -fsSL https://raw.githubusercontent.com/c2-circulo-desarrolladores/c2-cli/main/install.sh | sh
```

### Con Cargo desde Github
```bash
cargo install --git https://github.com/c2-circulo-desarrolladores/c2-cli
```

### Con Cargo localmente
```bash
cargo install --path .
```
## Uso

### Inicializar un proyecto

```bash
c2 init
```

### Importar un módulo

```bash
c2 import timer
c2 import logger
c2 import api
```

## Roadmap

- [ ] Fix `recursive_add` — mostrar rutas relativas al root, no absolutas