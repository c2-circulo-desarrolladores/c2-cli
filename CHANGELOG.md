## [0.8.1] - 2026-07-12

### 🐛 Bug Fixes

- **(init)** Fixes issue with cliff.toml not generating valid urls for issues

### 📚 Documentation

- Changes command format-py -> format in README

## [0.8.0] - 2026-07-12

### 🚀 Features

- **(init)** Adds .vscode/ folder after to init command
- **(init)** Updates .gitignore to exclude .vscode/
- **(init)** Updates .gitignore to exclude .vscode/
- **(init)** Adds pre-commit to init command to lint before committing ([#41](https://github.com/c2-circulo-desarrolladores/c2-cli/issues/41))

### 🐛 Bug Fixes

- Cliff.toml now ignores changelog-related commits in changelog
- **(release)** Fixes typo in release commit
- **(release)** Release workflow uses git-cliff for release message
- **(release)** Fixes typo in commit message

### ⚙️ Miscellaneous Tasks

- Updates cliff.toml

## [0.7.1] - 2026-07-10

### 🚀 Features

- **(init)** Replaces <REPO> with current dir name (relates to #35)
- **(init)** Adds optional argument to insert OWNER to parse repo url in cliff.toml ([#35](https://github.com/c2-circulo-desarrolladores/c2-cli/issues/35))

## [0.7.0] - 2026-07-10

### 🚀 Features

- **(init)** Replaces <REPO> with current dir name (relates to #35)
- **(init)** Adds optional argument to insert OWNER to parse repo url in cliff.toml ([#35](https://github.com/c2-circulo-desarrolladores/c2-cli/issues/35))

### 🐛 Bug Fixes

- **(init)** Format_check workflow does not include isort and autoflake check anymore
- **(init)** Migrates to src layout for new projects
- **(init)** Fixes typo y hatchling block
- Fixes conflicts in init.rs
- **(format)** Changed name for command format-py -> format
- **(init)** Resolves conflict with execute just

### ⚙️ Miscellaneous Tasks

- Fixes conflicts

## [0.6.2] - 2026-07-07

### 🐛 Bug Fixes

- Fixes release.yml

## [0.6.1] - 2026-07-07

### 🚀 Features

- **(init)** Activates commit preprocessor pattern for cliff.toml ([#30](https://github.com/c2-circulo-desarrolladores/c2-cli/issues/30))
- **(init)** Init adds ruff config block to pyproject.toml
- **(init)** Removes autoflake and isort dependencies ([#31](https://github.com/c2-circulo-desarrolladores/c2-cli/issues/31))

### 📚 Documentation

- Update README.md with new commands

### ⚙️ Miscellaneous Tasks

- Updates release.yml so it shows latest changelog entry

## [0.6.0] - 2026-07-05

### 🚀 Features

- **(general)** Adds --version flag ([#20](https://github.com/c2-circulo-desarrolladores/c2-cli/issues/20))
- **(io)** Adds FileParser struct to read and write to files
- **(init)** Init command writes [tool.commitizen] block to pyproject.toml ([#17](https://github.com/c2-circulo-desarrolladores/c2-cli/issues/17))
- **(init)** Init command writes hatchling block to pyproject.toml ([#22](https://github.com/c2-circulo-desarrolladores/c2-cli/issues/22))

### 🚜 Refactor

- Moves deployables to own modules

## [0.5.0] - 2026-07-03

### 🚀 Features

- **(general)** Includes just as dependency and delegates it to run commands ([#23](https://github.com/c2-circulo-desarrolladores/c2-cli/issues/23))
- **(init)** Init includes a README.md template ([#8](https://github.com/c2-circulo-desarrolladores/c2-cli/issues/8))

## [0.4.1] - 2026-07-02

### 🐛 Bug Fixes

- Adds blank space after new version for changelog

## [0.4.0] - 2026-07-02

### 🚀 Features

- **(init)** Adds CODEOWNERS file ([#16](https://github.com/c2-circulo-desarrolladores/c2-cli/issues/16))
- **(init)** Adds cliff.toml to init command
- **(release)** Adds release command

### 🐛 Bug Fixes

- **(init)** Release workflow only generates changelog for current tag range
- **(format)** Isort skips .gitignore
- Simplified msg when running commands
- **(init)** Updates msg for init and adds git-cliff as dev dependency
- Updates cliff.toml for project and init

### 📚 Documentation

- Adds instructions to install binary directly

### ⚙️ Miscellaneous Tasks

- Adds scripts to install binary directly

## [0.3.2] - 2026-06-29

### 🚀 Features

- **(init)** Init adds pytest as dev dependency

### 🐛 Bug Fixes

- **(init)** Cleans justfile of unused commands

### 📚 Documentation

- Adds direct installation instructions

### ⚙️ Miscellaneous Tasks

- Updates changelog
- Updates release to generate binaries for mac and linux
- Updates format for changelog generation

## [0.3.1] - 2026-06-28

### ⚙️ Miscellaneous Tasks

- Fixes release.toml so it does not show unreleased in changelog

## [0.3.0] - 2026-06-28

### 🚀 Features

- Adds format-py command

### ⚙️ Miscellaneous Tasks

- Adds git-cliff to release.toml

## [0.2.0] - 2026-06-28

### 🚀 Features

- **(init)** Init also adds dev dependencies

### 🐛 Bug Fixes

- Updates release workflow
- Fixes Cargo.toml edition

### 📚 Documentation

- Adds local installation guide

### ⚙️ Miscellaneous Tasks

- Add git-cliff configuration
- Updates changelog
- Update Cargo.toml
- Adds release.toml

## [0.1.0] - 2026-06-23

