from __future__ import annotations

from pathlib import Path

import polars as pl

# ============================================================
# 0. CONFIGURACIÓN GENERAL / CONSTANTES
# ============================================================
FILE_NAME = "your_file.xlsx"
EXCEL_ENGINE = "fastexcel"

SCRIPT_DIR = Path(__file__).parent
INPUT_FILE = SCRIPT_DIR / FILE_NAME
OUTPUT_FILE = INPUT_FILE.with_name(f"{INPUT_FILE.stem}_clean.xlsx")

# ============================================================
# 1. FUNCIONES PRINCIPALES
# ============================================================
def clean_dataframe(df: pl.DataFrame) -> pl.DataFrame:
    # Your logic here
    return df


def main() -> None:
    df = pl.read_excel(
        INPUT_FILE,
        engine=EXCEL_ENGINE,
    )

    df = clean_dataframe(df)

    df.write_excel(OUTPUT_FILE)

    print(f"✅ Archivo exportado correctamente en: {OUTPUT_FILE}")


# ============================================================
# 2. EJECUCIÓN DIRECTA
# ============================================================
if __name__ == "__main__":
    main()
