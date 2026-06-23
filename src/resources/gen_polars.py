import polars as pl
import os


uri = os.getenv("DATABASE_URL")
storage_options = {
    "account_name": os.getenv("AZURE_STORAGE_ACCOUNT_NAME"),
    "account_key": os.getenv("AZURE_STORAGE_ACCOUNT_KEY"),
}

df = pl.read_csv(
    "/FY13toFY23_06-26-2024.txt",
    separator="\t",        
    quote_char='"',        
    has_header=True,
    infer_schema_length=10000
)
# column_names = [
# ]


pl.Config.set_tbl_cols(-1)       # Muestra todas las columnas (-1 significa sin límite)
pl.Config.set_tbl_rows(-1)       # Muestra todas las filas
pl.Config.set_fmt_str_lengths(100) # Que no corte los textos largos dentro de las celdas
print(df.head())