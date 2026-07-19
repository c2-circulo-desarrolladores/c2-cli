## [0.9.0] - 2026-07-15

### 🚀 Features

- **(init)** Modifies body in cliff.toml to show commit hash and hyperlink in changelog ([8921dc2…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/8921dc24fcc2bb318ce734a27f95f836494f07c5))
- **(init)** Cliff.toml Captures any prefix text before #number (closes o relates) and adds backticks to text inside <> ([1128a51…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/1128a51f9924a14d335da8f87abdc7e3a7f51039))
- **(init)** Removes .vscode folder as functionality is enforced by pre-commit ([7ec1f14…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/7ec1f14206e5b395472085b81e15d9b10e0a19a8))
- **(init-r)** Adds new draft command init-r (not tested) (closes [#48](https://github.com/c2-circulo-desarrolladores/c2-cli/issues/48)) ([867bc92…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/867bc929368d90852b988e49da5530f2bfa0f5ca))
- **(init)** Init includes test workflow ([8d6b078…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/8d6b07881186d3281a9191963c91a5f075ccbea5))

### 🚜 Refactor

- Separates into deployable and executable traits (closes [#27](https://github.com/c2-circulo-desarrolladores/c2-cli/issues/27)) ([6fc751b…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/6fc751b557bb0c95eaa03128a304f17a8cbebc81))
- Deployables use new traits ([6228a45…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/6228a457796e48834df8f8e61637b6f1d433c1cb))

### 📚 Documentation

- Updates Changelog to show commit hashes ([5c1461d…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/5c1461d62a621ed98b605500e2708d9f710e122a))

### ⚙️ Miscellaneous Tasks

- Adds include_dir dependency ([8ed2cba…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/8ed2cba932de04e2fd4ae21d270a41e4293a81ae))
- Deletes unused files and updates main modules ([f0dd983…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/f0dd9838160e5e61ec4d4561b71a7a9e9c8c22dc))

## [0.8.3] - 2026-07-12

### 🐛 Bug Fixes

- **(api)** Changes dependency httpx -> httpx2 (closes [#45](https://github.com/c2-circulo-desarrolladores/c2-cli/issues/45)) ([3eb07ab…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/3eb07ab88cf9e7f9166584c8ce7ef6ecf1693947))
- **(api)** Api_wrapper includes a module doc with instructions; keeps original exceptions ([56d1bb3…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/56d1bb3d54010d360d4397e4fc8de63818c6cfb8))
- **(api)** Deletes dependency from ensure_logger and uses generic logger ([47775c5…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/47775c5b291ab411494a7ceec1bb927a1a443f47))

## [0.8.2] - 2026-07-12

### 🐛 Bug Fixes

- **(init)** Fixes cliff.toml to properly skip version bumps ([09a654d…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/09a654d0c65df82bc33b91ccd4d69c0574b67125))

### ⚙️ Miscellaneous Tasks

- Regenerate changelog with proper urls and skipping release bumps ([0ef9866…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/0ef9866f045427e47384da5bd1e39e4da3d99bfc))

## [0.8.1] - 2026-07-12

### 🐛 Bug Fixes

- **(init)** Fixes issue with cliff.toml not generating valid urls for issues ([6d880c8…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/6d880c803109a220a7d41c9bdbc7a85cf79e2b02))

### 📚 Documentation

- Changes command format-py -> format in README ([2b5a9d2…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/2b5a9d2f55a269fdca428cc1c6689ca28268c95b))

## [0.8.0] - 2026-07-12

### 🚀 Features

- **(init)** Adds .vscode/ folder after to init command ([cb2ce80…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/cb2ce80e6dd1733caade3242ab13fee6db756608))
- **(init)** Updates .gitignore to exclude .vscode/ ([1ddc8a2…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/1ddc8a22ff7333a1ac01e261926ad640771fbede))
- **(init)** Updates .gitignore to exclude .vscode/ ([d175770…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/d17577095ccbff1bfd0be97a7e5409d31e555911))
- **(init)** Adds pre-commit to init command to lint before committing (closes [#41](https://github.com/c2-circulo-desarrolladores/c2-cli/issues/41)) ([aa1353c…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/aa1353c53090c94f361b7214d8a67512acdfaad8))

### 🐛 Bug Fixes

- Cliff.toml now ignores changelog-related commits in changelog ([8524280…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/8524280818dfd018ae73dc18bcb62a55ffb469ce))
- **(release)** Fixes typo in release commit ([06d7f8b…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/06d7f8b90889de427dad327531730adc5be76982))
- **(release)** Release workflow uses git-cliff for release message ([6c48850…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/6c488506da469edf66c855f7755a8bddb809c8a9))
- **(release)** Fixes typo in commit message ([929f42b…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/929f42b50e689ce3d3938266501fb777227fd82e))

### ⚙️ Miscellaneous Tasks

- Updates cliff.toml ([bdce8a4…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/bdce8a45ad87f86b1f0a67eebd5fc16585c39b97))

## [0.7.1] - 2026-07-10

### 🚀 Features

- **(init)** Replaces `<REPO>` with current dir name (relates to [#35](https://github.com/c2-circulo-desarrolladores/c2-cli/issues/35)) ([beab56c…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/beab56c903b2c78642f03c602a586ae06ccaab70))
- **(init)** Adds optional argument to insert OWNER to parse repo url in cliff.toml (closes [#35](https://github.com/c2-circulo-desarrolladores/c2-cli/issues/35)) ([675d2d4…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/675d2d44dd5984f2c5e6434dfad5e509c42e8d42))

## [0.7.0] - 2026-07-10

### 🚀 Features

- **(init)** Replaces `<REPO>` with current dir name (relates to [#35](https://github.com/c2-circulo-desarrolladores/c2-cli/issues/35)) ([d2aec94…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/d2aec94cec9c744b812ff41500ed5d7385f9b63e))
- **(init)** Adds optional argument to insert OWNER to parse repo url in cliff.toml (closes [#35](https://github.com/c2-circulo-desarrolladores/c2-cli/issues/35)) ([fdec7d2…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/fdec7d2957254b93cf480048cba8135fcf900b9f))

### 🐛 Bug Fixes

- **(init)** Format_check workflow does not include isort and autoflake check anymore ([64cd70b…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/64cd70b812bfb707e4980fd1534c312d5b454eb9))
- **(init)** Migrates to src layout for new projects ([b646c16…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/b646c168b9826edd3ac138e616b938b460a3ac05))
- **(init)** Fixes typo y hatchling block ([6ca7e8a…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/6ca7e8a80529e8f4e7170a2f50bd687dd9ee9d07))
- Fixes conflicts in init.rs ([e6ec1e5…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/e6ec1e5ebac5b745fd270d90a5a31e9f9d9caced))
- **(format)** Changed name for command format-py -> format ([bcf0fb7…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/bcf0fb7d2ece5319e5a7ad298eeadb20842a5c37))
- **(init)** Resolves conflict with execute just ([b89a387…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/b89a3876bca2a0795152c0da73e94c48cd1eab9a))

### ⚙️ Miscellaneous Tasks

- Fixes conflicts ([b26ab37…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/b26ab37020e7857c120ab031e322d91ee88b085d))

## [0.6.2] - 2026-07-07

### 🐛 Bug Fixes

- Fixes release.yml ([7e901f2…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/7e901f238f67727e4fa501b13fe3bd0e4dbc9bf0))

## [0.6.1] - 2026-07-07

### 🚀 Features

- **(init)** Activates commit preprocessor pattern for cliff.toml (closes [#30](https://github.com/c2-circulo-desarrolladores/c2-cli/issues/30)) ([3caca20…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/3caca208a4eb4950878531bca6de7f1b01b28139))
- **(init)** Init adds ruff config block to pyproject.toml ([426ccce…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/426cccee470a89ab845c441cc7135e0be472d78f))
- **(init)** Removes autoflake and isort dependencies (closes [#31](https://github.com/c2-circulo-desarrolladores/c2-cli/issues/31)) ([65e90b6…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/65e90b6574ffd8ac42ef0d71ffc60301cb82ae3e))

### 📚 Documentation

- Update README.md with new commands ([b19ecf1…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/b19ecf1bdc97a32b2ac73e56f3cf6f7d7db6d586))

### ⚙️ Miscellaneous Tasks

- Updates release.yml so it shows latest changelog entry ([1d853cd…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/1d853cd15b87a1a5b0d086bc6734368b0a9c33b3))

## [0.6.0] - 2026-07-05

### 🚀 Features

- **(general)** Adds --version flag (closes [#20](https://github.com/c2-circulo-desarrolladores/c2-cli/issues/20)) ([a23e75d…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/a23e75d52b515dc86f840eedb9abfede5af11d1b))
- **(io)** Adds FileParser struct to read and write to files ([37e4e21…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/37e4e21ad0b24cc21e1e87df75683a13bb8f0b19))
- **(init)** Init command writes [tool.commitizen] block to pyproject.toml (closes [#17](https://github.com/c2-circulo-desarrolladores/c2-cli/issues/17)) ([1e7a062…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/1e7a0622d6f260c800d42882bd37dc9844027b68))
- **(init)** Init command writes hatchling block to pyproject.toml (closes [#22](https://github.com/c2-circulo-desarrolladores/c2-cli/issues/22)) ([e8c7ed5…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/e8c7ed597769d4dd03a9ffbac544899e6d6be828))

### 🚜 Refactor

- Moves deployables to own modules ([5769b0e…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/5769b0e60e1f761a7d7707c3b7ab92ca56ada1cc))

## [0.5.0] - 2026-07-03

### 🚀 Features

- **(general)** Includes just as dependency and delegates it to run commands (closes [#23](https://github.com/c2-circulo-desarrolladores/c2-cli/issues/23)) ([7725359…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/772535968026da44eb96415c6d54553a6aa4567c))
- **(init)** Init includes a README.md template (closes [#8](https://github.com/c2-circulo-desarrolladores/c2-cli/issues/8)) ([e900bf6…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/e900bf6e18673d96dadd53f7f974bb780b3b7e5b))

## [0.4.1] - 2026-07-02

### 🐛 Bug Fixes

- Adds blank space after new version for changelog ([d583a83…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/d583a8387cc84e34108c14cdd041e769b77ea62e))

## [0.4.0] - 2026-07-02

### 🚀 Features

- **(init)** Adds CODEOWNERS file (closes [#16](https://github.com/c2-circulo-desarrolladores/c2-cli/issues/16)) ([641aae8…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/641aae8d5e4bbe9bde9f0d7e0aca8529f5fb3c2c))
- **(init)** Adds cliff.toml to init command ([cae6728…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/cae67283469397823958fe5a5ae46f752f79d418))
- **(release)** Adds release command ([7f468a9…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/7f468a94a4650a0b51e538bcabc0e7f1ebd3e93f))

### 🐛 Bug Fixes

- **(init)** Release workflow only generates changelog for current tag range ([d800bc3…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/d800bc3bfb7808ec998f41d95df678f4931296c0))
- **(format)** Isort skips .gitignore ([635fe40…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/635fe40034b10cd5baf5c1fd9d07ed7cd650b2d8))
- Simplified msg when running commands ([3e75345…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/3e753450f0a710508301627d2be7997c9151e0b4))
- **(init)** Updates msg for init and adds git-cliff as dev dependency ([03f74fd…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/03f74fd86f908d45837e7be5d5dd47a4417356b7))
- Updates cliff.toml for project and init ([c7607c4…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/c7607c4423a7c577550e78d2c25a3b858376183e))

### 📚 Documentation

- Adds instructions to install binary directly ([85f32ee…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/85f32ee2ef62e1986d26b63c03efb19ea1f99637))

### ⚙️ Miscellaneous Tasks

- Adds scripts to install binary directly ([f115680…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/f1156804ab003d7c265f58614f775ed1849bc16f))

## [0.3.2] - 2026-06-29

### 🚀 Features

- **(init)** Init adds pytest as dev dependency ([be6b7a7…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/be6b7a7813ba81205fe0757aa58824cd8e4d1fb1))

### 🐛 Bug Fixes

- **(init)** Cleans justfile of unused commands ([32872bb…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/32872bbff050f8b702216f04d739be64262b529b))

### 📚 Documentation

- Adds direct installation instructions ([5777f79…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/5777f79d486b572cadc9ee074aac0996cd10cade))

### ⚙️ Miscellaneous Tasks

- Updates changelog ([4d95c69…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/4d95c69fe8b2c6d4f9a3c8a30c477776040dbf10))
- Updates release to generate binaries for mac and linux ([398b2ff…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/398b2ffc09fe54af0879cdf1629af2bb03106296))
- Updates format for changelog generation ([93940ec…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/93940ec92878a26729b03162e90a79461f7febcb))

## [0.3.1] - 2026-06-28

### ⚙️ Miscellaneous Tasks

- Fixes release.toml so it does not show unreleased in changelog ([cc65488…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/cc65488236df530047d4f450914f029c43116601))

## [0.3.0] - 2026-06-28

### 🚀 Features

- Adds format-py command ([3eb8640…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/3eb8640c9c6eb1819de99761e4d61374aeb3fb9f))

### ⚙️ Miscellaneous Tasks

- Adds git-cliff to release.toml ([acee0af…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/acee0af9d69b61ec022d2c89a825a8288754c38b))

## [0.2.0] - 2026-06-28

### 🚀 Features

- **(init)** Init also adds dev dependencies ([19e370d…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/19e370d1fc289d187a56ade60d48d5e99ddebe63))

### 🐛 Bug Fixes

- Updates release workflow ([879ab41…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/879ab41ed7c3451cacceed18c84f150abdb4a60a))
- Fixes Cargo.toml edition ([c6daab8…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/c6daab86093d2fb95968053dff2986d1140976a0))

### 📚 Documentation

- Adds local installation guide ([e1eed51…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/e1eed51682c000c5ff6802e9f9624600e784b7bd))

### ⚙️ Miscellaneous Tasks

- Add git-cliff configuration ([dae9906…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/dae9906276fcce8c96391d46fac42aa65a257469))
- Updates changelog ([5d97aeb…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/5d97aeb0495e21cb4f4b111080c8b0f8a928af21))
- Update Cargo.toml ([fb71754…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/fb71754e17b280a18bc23cc3a576f82d96c514f8))
- Adds release.toml ([9444eeb…](https://github.com/c2-circulo-desarrolladores/c2-cli/commit/9444eeb44847c0efc1033ff520f598e4f8228eec))

## [0.1.0] - 2026-06-23

