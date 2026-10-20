# Configuration file for the Sphinx documentation builder.
#
# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

project = "<REPO>"
copyright = "2026, <OWNER>"
author = "<OWNER>"
release = "0.2.3"

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

import os
import sys

# sys.path.insert(0, os.path.abspath("../src"))

sys.path.insert(0, os.path.abspath(".."))

directories = [os.path.abspath("..")]

# # List files in the directories
# for directory in directories:
#     print(f"Modules in {directory}:")
#     for root, dirs, files in os.walk(directory):
#         for file in files:
#             if file.endswith('.py'):
#                 print(f"  {file}")


extensions = [
    "sphinx.ext.todo",
    "sphinx.ext.viewcode",
    "sphinx.ext.autodoc",  # This enables autodoc for automodule
    "sphinx.ext.napoleon",
    "sphinx_automodapi.automodapi",
    "sphinx.ext.autosummary",
    "sphinx_togglebutton",
]


autodoc_default_options = {
    "member-order": "bysource",
    "show-inheritance": True,
    "undoc-members": True,
    "no-member": True,
}

autosummary_context = {
    "skipmethods": ["__init__", "__new__", "__del__"],
    "show_module_summary": False,
}


def setup(app):
    def prettify(name):
        overrides = {"core": "Core", "resources": "Recursos"}
        return overrides.get(name, name.replace("_", " ").title())

    app.connect(
        "builder-inited",
        lambda app: app.builder.templates.environment.filters.update(pretty=prettify),
    )


templates_path = ["_templates"]
exclude_patterns = []

# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_theme = "pydata_sphinx_theme"
html_static_path = ["_static"]

add_module_names = False
autosummary_generate = True
autosummary_imported_members = False
autosummary_generate_overwrite = True


# Un parámetro por línea cuando la firma pasa de N caracteres
maximum_signature_line_length = 60

# Literal['a', 'b', 'c'] → 'a' | 'b' | 'c' (Sphinx >= 7.2)
python_display_short_literal_types = True

# Tipos fuera de la firma, en la sección Parameters
autodoc_typehints = "description"
autodoc_typehints_format = "short"

html_add_permalinks = None
html_context = {
    "default_link_target": "_blank",
    "default_mode": "dark",
    "navbar_end": ["navbar-icon-links"],
}


html_css_files = [
    "css/custom.css",  # Asegúrate de que este archivo exista en _static/css/
]
