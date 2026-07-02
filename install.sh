#!/bin/sh
set -e

# --------------------------
# Configuración
# --------------------------
REPO="c2-circulo-desarrolladores/c2-cli"
BINARY_NAME="c2"
INSTALL_DIR="${HOME}/.local/bin"

# --------------------------
# Detectar OS y arquitectura
# --------------------------
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Linux)
    case "$ARCH" in
      x86_64) ASSET="c2-linux-x86_64" ;;
      aarch64|arm64) ASSET="c2-linux-aarch64" ;;
      *)
        echo "Error: arquitectura no soportada: $ARCH" >&2
        exit 1
        ;;
    esac
    ;;
  Darwin)
    case "$ARCH" in
      arm64) ASSET="c2-macos-aarch64" ;;
      x86_64)
        echo "Error: c2-cli ya no publica build para Mac Intel (x86_64)." >&2
        echo "Si tu Mac es Apple Silicon (M1/M2/M3/M4) pero ves este error," >&2
        echo "probablemente estás corriendo una terminal bajo Rosetta." >&2
        echo "Abre una terminal nativa ARM64 e intenta de nuevo." >&2
        exit 1
        ;;
      *)
        echo "Error: arquitectura no soportada: $ARCH" >&2
        exit 1
        ;;
    esac
    ;;
  *)
    echo "Error: sistema operativo no soportado: $OS" >&2
    echo "Si usas Windows, corre en PowerShell:" >&2
    echo "  irm https://raw.githubusercontent.com/${REPO}/main/install.ps1 | iex" >&2
    exit 1
    ;;
esac

URL="https://github.com/${REPO}/releases/latest/download/${ASSET}"

echo ""
echo "Descargando c2..."
echo "$URL"
echo ""

# --------------------------
# Directorio temporal
# --------------------------
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

TMP_BIN="${TMP_DIR}/${BINARY_NAME}"

if command -v curl >/dev/null 2>&1; then
  curl -fsSL "$URL" -o "$TMP_BIN"
elif command -v wget >/dev/null 2>&1; then
  wget -q "$URL" -O "$TMP_BIN"
else
  echo "Error: se necesita curl o wget para instalar c2." >&2
  exit 1
fi

chmod +x "$TMP_BIN"

# --------------------------
# Instalar
# --------------------------
mkdir -p "$INSTALL_DIR"
mv "$TMP_BIN" "${INSTALL_DIR}/${BINARY_NAME}"

echo ""
echo "✓ c2 instalado correctamente en ${INSTALL_DIR}/${BINARY_NAME}"

# --------------------------
# Verificar / avisar sobre el PATH
# --------------------------
case ":${PATH}:" in
  *":${INSTALL_DIR}:"*)
    echo "✓ ${INSTALL_DIR} ya está en tu PATH."
    ;;
  *)
    echo ""
    echo "⚠ ${INSTALL_DIR} no está en tu PATH."
    echo "Agrega esta línea a tu ~/.bashrc, ~/.zshrc o equivalente:"
    echo ""
    echo "    export PATH=\"\$HOME/.local/bin:\$PATH\""
    echo ""
    echo "Y luego reinicia la terminal o corre: source ~/.bashrc (o ~/.zshrc)"
    ;;
esac

echo ""
echo "Comprueba con:"
echo ""
echo "    c2 --version"