# c2 - cli

CLI para inicializar proyectos Python e importar módulos reutilizables.

```bash
c2 <COMMAND>
```

## Comandos

| Comando | Descripción |
|--------|-------------|
| `init` | Inicializa un proyecto Python corriendo `'uv init'` y agrega `.gitignore`, `justfile` y `.github/` |
| `config` | Configura valores persistentes del CLI (ej. `owner` por defecto) |
| `release` | Release completo: aumentar version, generar changelog, generar tag, commit y push |
| `import` | Copia un módulo reutilizable al proyecto actual (`timer`, `logger`, `api`...) |
| `format` | Llama a ruff para formatear código de Python de todo el repositorio |
| `help` | Muestra ayuda |

## Instalación

### Directa

**Windows (PowerShell):**
```powershell
irm https://raw.githubusercontent.com/c2-circulo-desarrolladores/c2-cli/main/install.ps1 | iex
```

**Linux/macOS:**
```sh
curl -fsSL https://raw.githubusercontent.com/c2-circulo-desarrolladores/c2-cli/main/install.sh | sh
```

**Con Cargo desde Github**
```bash
cargo install --git https://github.com/c2-circulo-desarrolladores/c2-cli
```

**Con Cargo localmente**
```bash
cargo install --path .
```

## Uso

### Configurar el CLI
Guarda un `owner` por defecto para no tener que pasarlo cada vez con `--owner` en `init`:

```bash
c2 config --owner c2-circulo-desarrolladores
```
Ver la configuración actual:
```bash
c2 config --show
```
La configuración se guarda en el directorio de configuración del sistema (ej. `%APPDATA%\c2\c2_cli\config.toml` en Windows, `~/.config/c2_cli/config.toml` en Linux/macOS).

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
