$ErrorActionPreference = "Stop"

# --------------------------
# Configuración
# --------------------------
$Repo = "c2-circulo-desarrolladores/c2-cli"
$Binary = "c2.exe"
$InstallDir = Join-Path $env:LOCALAPPDATA "Programs\c2"

# --------------------------
# TLS 1.2 explícito (compatibilidad con Windows/PowerShell más viejos)
# --------------------------
[Net.ServicePointManager]::SecurityProtocol = [Net.ServicePointManager]::SecurityProtocol -bor [Net.SecurityProtocolType]::Tls12

# --------------------------
# Detectar arquitectura
# --------------------------
switch ($env:PROCESSOR_ARCHITECTURE) {
    "ARM64" { $Asset = "c2-windows-aarch64.zip" }
    default { $Asset = "c2-windows-x86_64.zip" }
}

$Url = "https://github.com/$Repo/releases/latest/download/$Asset"

Write-Host ""
Write-Host "Descargando c2..."
Write-Host $Url
Write-Host ""

# --------------------------
# Directorio temporal
# --------------------------
$Temp = Join-Path $env:TEMP ("c2-" + [guid]::NewGuid())
New-Item -ItemType Directory -Force -Path $Temp | Out-Null

$Zip = Join-Path $Temp $Asset

try {
    Invoke-WebRequest -Uri $Url -OutFile $Zip -UseBasicParsing
}
catch {
    Remove-Item -Path $Temp -Recurse -Force -ErrorAction SilentlyContinue
    throw "No se pudo descargar $Url. Verifica que el asset exista en la última release. Detalle: $_"
}

Expand-Archive -Path $Zip -DestinationPath $Temp -Force

# --------------------------
# Buscar el exe
# --------------------------
$Exe = Get-ChildItem -Path $Temp -Filter $Binary -Recurse | Select-Object -First 1

if (-not $Exe) {
    Remove-Item -Path $Temp -Recurse -Force -ErrorAction SilentlyContinue
    throw "No encontré $Binary dentro del ZIP."
}

# --------------------------
# Instalar
# --------------------------
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
Copy-Item $Exe.FullName (Join-Path $InstallDir $Binary) -Force

# --------------------------
# Limpieza
# --------------------------
Remove-Item -Path $Temp -Recurse -Force -ErrorAction SilentlyContinue

# --------------------------
# Agregar al PATH (usuario)
# --------------------------
$CurrentPath = [Environment]::GetEnvironmentVariable("Path", "User")

if ($CurrentPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$CurrentPath;$InstallDir", "User")
    Write-Host ""
    Write-Host "✓ Se agregó $InstallDir al PATH."
}
else {
    Write-Host "✓ El PATH ya estaba configurado."
}

# Refresca el PATH de la sesión actual (para poder probar sin abrir otra terminal)
$env:Path += ";$InstallDir"

Write-Host ""
Write-Host "✓ c2 instalado correctamente en $InstallDir"
Write-Host ""
Write-Host "Comprueba con:"
Write-Host ""
Write-Host "    c2 --version"
Write-Host ""
Write-Host "(Si el comando no se reconoce, abre una nueva terminal para recargar el PATH.)"