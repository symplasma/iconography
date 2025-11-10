# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.1.2 (2025-11-10)

### New Features

 - <csr-id-1e6d63f96c7de4d851c301b20938bb3b6dbc86f9/> Make load_icon_textures and IconInfo public

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Make load_icon_textures and IconInfo public ([`1e6d63f`](https://github.com/symplasma/iconography/commit/1e6d63f96c7de4d851c301b20938bb3b6dbc86f9))
</details>

## v0.1.1 (2025-11-10)

### New Features

 - <csr-id-765bb73ba3c09f563c0da4d1958f9c76a7ac1d7e/> Add method to return all icons

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 41 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release iconography v0.1.1 ([`a87fbc6`](https://github.com/symplasma/iconography/commit/a87fbc6b9b4d558bc025b45ea25c0857672896b1))
    - Add rust crates design doc ([`8a6e25c`](https://github.com/symplasma/iconography/commit/8a6e25c67e288e0a8e34283d804e496e7fb1952b))
    - Exclude design docs from crate ([`7dd13e5`](https://github.com/symplasma/iconography/commit/7dd13e51b93cc39514fd8c50a5845c3a413c8beb))
    - Add method to return all icons ([`765bb73`](https://github.com/symplasma/iconography/commit/765bb73ba3c09f563c0da4d1958f9c76a7ac1d7e))
</details>

## v0.1.0 (2025-09-29)

<csr-id-19c2f0688e8d9a9823f0e459d62eb5d4b28846db/>
<csr-id-1b7fafca1156f29ac2e7fdeb8e515453191885b8/>
<csr-id-c7c12b9b8faf9801e69236a7ef9e73dd88bdbb69/>
<csr-id-821e0db8ee8b781e818b05e9fbb9de3889218a3e/>

### Chore

 - <csr-id-19c2f0688e8d9a9823f0e459d62eb5d4b28846db/> initialize Rust project with Cargo

### New Features

 - <csr-id-958386d677431e21241d5926ac4cdc3328b2f6f3/> improve UI responsiveness during icon loading
 - <csr-id-6691895a263efbccc1d4e077f6d1477b06e60245/> add `chunks` method to `IconCache` to support icon grid rendering
 - <csr-id-7a0286c38302ac4345ed4b1321ee11bb205f53da/> add hover text with icon path in icon viewer
 - <csr-id-388986fe17005c1461408e185dcc9b1f8d206041/> add debug logging for image loading in load_icon_image
 - <csr-id-02e2468ec2dbc72d0f101a086b5cef62e41f7c10/> Add tracing support with file logging to XDG cache directory
 - <csr-id-8248728b3dc384a36a6cd90f4d7629c75d44d3a5/> add handling for xpm files with placeholder image support
 - <csr-id-17dba9d8db5eff8f911da4ea2858ac6983c00b0f/> add toolbar with dark mode toggle and icon size selector
 - <csr-id-68948f437d3e72eda4e459e22a12822350e613a2/> add NixOS-specific icon search paths for better icon discovery
 - <csr-id-d444b74b70a60b6b36a6275c562d656af44aed33/> add shell.nix for Rust development with X11 and OpenGL dependencies
 - <csr-id-b581cbffd73d370551c1588ebf16dd180063528a/> create icon viewer app with egui and eframe

### Bug Fixes

 - <csr-id-45e6c600de2827649cb77c47b4a78cff34e139cc/> replace `on_hover_text_at_pointer` with `on_hover_text`
 - <csr-id-71baf206446c4f3d243d38115b9757c435a71513/> resolve type inference errors in hover text conversion
 - <csr-id-56f82d0c9ff8a76e530fc7396c9233fd633aa7ba/> Update SVG rendering to use latest resvg library API
 - <csr-id-7c8e1259e56b071815ac4721a70210877e674e72/> correct usvg and resvg tree rendering method calls
 - <csr-id-ee4ddaa728ca24382819feeade8b86d503ab689d/> resolve library API usage and UI response handling errors

### Refactor

 - <csr-id-1b7fafca1156f29ac2e7fdeb8e515453191885b8/> simplify hover tooltip logic for icons and labels
 - <csr-id-c7c12b9b8faf9801e69236a7ef9e73dd88bdbb69/> replace deprecated egui tooltip with modern API
 - <csr-id-821e0db8ee8b781e818b05e9fbb9de3889218a3e/> update deprecated egui function calls

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 56 commits contributed to the release over the course of 1 calendar day.
 - 19 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release iconography v0.1.0 ([`d84eb83`](https://github.com/symplasma/iconography/commit/d84eb837ed99fb2c90d20288e7ba429c13a35049))
    - Add auto-generated changelog ([`7314209`](https://github.com/symplasma/iconography/commit/7314209b3f3577eb86819bd0d104990345ca9eac))
    - Update cargo metadata ([`cbbce69`](https://github.com/symplasma/iconography/commit/cbbce6944fe6c2a384de71a57401fcd0ba4f9e4c))
    - Update README.md ([`f1e254f`](https://github.com/symplasma/iconography/commit/f1e254f68da9402bc898a1769a087dd2ede18476))
    - Add license, BSD-3-Clause ([`bd262c1`](https://github.com/symplasma/iconography/commit/bd262c1f97e18049ebc30ed90ca4dcf9fba2caaf))
    - Change project name to Iconography ([`1056e1f`](https://github.com/symplasma/iconography/commit/1056e1f53bb75fb255b1abc7b2f7736e4233fb66))
    - Fix hang on close ([`0daf467`](https://github.com/symplasma/iconography/commit/0daf467bc4e4ecb1e36595622285281270036351))
    - Show progress and stop when done ([`5e61847`](https://github.com/symplasma/iconography/commit/5e61847f9fd9d93e8c165807e0a4dbb7752bf33e))
    - Improve UI responsiveness during icon loading ([`958386d`](https://github.com/symplasma/iconography/commit/958386d677431e21241d5926ac4cdc3328b2f6f3))
    - Load icons in separate thread ([`de21bf1`](https://github.com/symplasma/iconography/commit/de21bf10bcdc0fab4e01e38dbe9bf2533dc7ea13))
    - Add `chunks` method to `IconCache` to support icon grid rendering ([`6691895`](https://github.com/symplasma/iconography/commit/6691895a263efbccc1d4e077f6d1477b06e60245))
    - Move icon vec into IconCache ([`4502f0c`](https://github.com/symplasma/iconography/commit/4502f0cead032425c9f5af1030316b56c4591ce9))
    - Refactor icon loading ([`2daef12`](https://github.com/symplasma/iconography/commit/2daef12a5cbc4ffb23c81ac877da17559346ed20))
    - Add debug message and upgrade another ([`c0739fc`](https://github.com/symplasma/iconography/commit/c0739fc5db8bb95ef8e3051c322c4e1b48940ab9))
    - Fix hover text ([`5b71443`](https://github.com/symplasma/iconography/commit/5b71443734b1cb1cffd5baf3be7c2285ab08f9d3))
    - Replace `on_hover_text_at_pointer` with `on_hover_text` ([`45e6c60`](https://github.com/symplasma/iconography/commit/45e6c600de2827649cb77c47b4a78cff34e139cc))
    - Add hover text with icon path in icon viewer ([`7a0286c`](https://github.com/symplasma/iconography/commit/7a0286c38302ac4345ed4b1321ee11bb205f53da))
    - Add troubleshooting TODO ([`6406508`](https://github.com/symplasma/iconography/commit/640650852124d8a9ca11f9c58af35ccc96ad4669))
    - Show icon type ([`15f20ca`](https://github.com/symplasma/iconography/commit/15f20caea3400b47cee3bc15ad0da27c168968b9))
    - Refactor `render_icon` function ([`bee43e0`](https://github.com/symplasma/iconography/commit/bee43e0056ffeae5c05e231753b9937eecd9ed50))
    - Add loaded icons count ([`b1b5f49`](https://github.com/symplasma/iconography/commit/b1b5f493e756e19fe8ebb3e75ae8e4b611de37ac))
    - Add MAX_ICON_SEARCH_DEPTH ([`d406d1b`](https://github.com/symplasma/iconography/commit/d406d1bf3a8133714afb518999ea8d4b6653ed80))
    - Add additional paths to search ([`7acb0e7`](https://github.com/symplasma/iconography/commit/7acb0e78a551342aa3521b58bcec11c695454938))
    - Format code with `cargo fmt` ([`14da207`](https://github.com/symplasma/iconography/commit/14da20702e9f9cc52ab0a0f704e1d647b4189f31))
    - Add debug logging for image loading in load_icon_image ([`388986f`](https://github.com/symplasma/iconography/commit/388986fe17005c1461408e185dcc9b1f8d206041))
    - Add tracing support with file logging to XDG cache directory ([`02e2468`](https://github.com/symplasma/iconography/commit/02e2468ec2dbc72d0f101a086b5cef62e41f7c10))
    - Change Rust edition ([`4f21672`](https://github.com/symplasma/iconography/commit/4f216721b481adf0b3baac25045955b03e05d5ca))
    - Add handling for xpm files with placeholder image support ([`8248728`](https://github.com/symplasma/iconography/commit/8248728b3dc384a36a6cd90f4d7629c75d44d3a5))
    - Show icon loading error messages ([`0445f1d`](https://github.com/symplasma/iconography/commit/0445f1d86019032ad90db48796061035a9941dc6))
    - Refactor update function ([`8194e55`](https://github.com/symplasma/iconography/commit/8194e55fd2ef90689597ee123e04d721af3273b8))
    - Update eframe and egui ([`b8372cf`](https://github.com/symplasma/iconography/commit/b8372cffd149b0012eaefa75a477ff58cd15eb60))
    - Resolve type inference errors in hover text conversion ([`71baf20`](https://github.com/symplasma/iconography/commit/71baf206446c4f3d243d38115b9757c435a71513))
    - Simplify hover tooltip logic for icons and labels ([`1b7fafc`](https://github.com/symplasma/iconography/commit/1b7fafca1156f29ac2e7fdeb8e515453191885b8))
    - Remove alsa audo libs ([`262efcd`](https://github.com/symplasma/iconography/commit/262efcde542de5eba1ead0b1b5590cc261eedf7f))
    - Copy `shell.nix` from Free-launch ([`6c4d0b5`](https://github.com/symplasma/iconography/commit/6c4d0b5878e9d9c9576f2ada3abc1987a06080f8))
    - Replace deprecated egui tooltip with modern API ([`c7c12b9`](https://github.com/symplasma/iconography/commit/c7c12b9b8faf9801e69236a7ef9e73dd88bdbb69))
    - Update deprecated egui function calls ([`821e0db`](https://github.com/symplasma/iconography/commit/821e0db8ee8b781e818b05e9fbb9de3889218a3e))
    - Update SVG rendering to use latest resvg library API ([`56f82d0`](https://github.com/symplasma/iconography/commit/56f82d0c9ff8a76e530fc7396c9233fd633aa7ba))
    - Update image and SVG crates ([`a6fe035`](https://github.com/symplasma/iconography/commit/a6fe0353ce98e12ccac2f4507acc2c2c11796611))
    - Upgrade compatible dependencies ([`ff0a982`](https://github.com/symplasma/iconography/commit/ff0a98280a01c79f89bfbc515d4bebea625b8c0a))
    - Refactor code into library and bin ([`2ebc97f`](https://github.com/symplasma/iconography/commit/2ebc97f50401b2cdc68603377fae4cf72042bb12))
    - Format code with `cargo fmt` ([`0f7b366`](https://github.com/symplasma/iconography/commit/0f7b36691d080442f4da762d9402641f290c2816))
    - Add toolbar with dark mode toggle and icon size selector ([`17dba9d`](https://github.com/symplasma/iconography/commit/17dba9d8db5eff8f911da4ea2858ac6983c00b0f))
    - Add NixOS-specific icon search paths for better icon discovery ([`68948f4`](https://github.com/symplasma/iconography/commit/68948f437d3e72eda4e459e22a12822350e613a2))
    - Format code with `cargo fmt` ([`626e4ca`](https://github.com/symplasma/iconography/commit/626e4ca593b6ca00fa006cbaadba33915873ed82))
    - Add Cargo.lock file ([`f1a2f6d`](https://github.com/symplasma/iconography/commit/f1a2f6dc41d9d0d1e946b6136b32b45ce6177a5d))
    - Update .gitignore file ([`410d3a5`](https://github.com/symplasma/iconography/commit/410d3a5bffd5e1ca4e6e480fbdfa15529a64c061))
    - Add shell.nix for Rust development with X11 and OpenGL dependencies ([`d444b74`](https://github.com/symplasma/iconography/commit/d444b74b70a60b6b36a6275c562d656af44aed33))
    - Correct usvg and resvg tree rendering method calls ([`7c8e125`](https://github.com/symplasma/iconography/commit/7c8e1259e56b071815ac4721a70210877e674e72))
    - Resolve library API usage and UI response handling errors ([`ee4ddaa`](https://github.com/symplasma/iconography/commit/ee4ddaa728ca24382819feeade8b86d503ab689d))
    - Create icon viewer app with egui and eframe ([`b581cbf`](https://github.com/symplasma/iconography/commit/b581cbffd73d370551c1588ebf16dd180063528a))
    - Initialize Rust project with Cargo ([`19c2f06`](https://github.com/symplasma/iconography/commit/19c2f0688e8d9a9823f0e459d62eb5d4b28846db))
    - Add app overview ([`69f06f4`](https://github.com/symplasma/iconography/commit/69f06f40a189289c547aaf8cc98a5824aa433369))
    - Add design docs ([`3ee040d`](https://github.com/symplasma/iconography/commit/3ee040daddea632727fa7b38695e29d97c52a627))
    - Add Aider ignores ([`12eaf12`](https://github.com/symplasma/iconography/commit/12eaf127805c9ca01f00b762e2650543bb26705a))
    - Initial Commit ([`cbd9f0e`](https://github.com/symplasma/iconography/commit/cbd9f0e300f179714620b83b0df15df7b7b257e9))
</details>

