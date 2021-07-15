[![Crates.io](https://img.shields.io/crates/v/export-ffxiv-macros.svg)](https://crates.io/crates/export-ffxiv-macros/)
[![Apache-2.0](https://img.shields.io/github/license/carriejv/export-ffxiv-macros)](https://github.com/carriejv/export-ffxiv-macros/blob/master/LICENSE/)
[![Build Status](https://github.com/carriejv/export-ffxiv-macros/workflows/CIBuild/badge.svg?branch=master)](https://github.com/carriejv/export-ffxiv-macros/actions?query=workflow%3ACIBuild)

# Export FFXIV Macros

A simple tool that leverages [libxivdat](https://github.com/carriejv/libxivdat) to dump Final Fantasy XIV macros to [TOML](https://toml.io/en/).

Libxivdat could theoretically support writing back to `MACRO.DAT` files. I might support that at some point.

# Usage

`export-ffxiv-macros` -> Exports all character-specific and system-wide macros located in the usual places. (`$HOME/Documents/My Games/Final Fantasy XIV - A Realm Reborn/`)

`export-ffxiv-macros path/to/MACRO.DAT path/to/out.toml` -> Exports a specific file.

## Mac / Linux Caveats

This will search your actual home directory when run without args. If you sandbox your wine user directories, you'll need to run the windows app in wine or manually point the native app at the correct files.

# Output Sample

```toml
[[macro]]
title = 'Low Spell Fx'
icon = 'Number0'
lines = [
    '/bfx party simple',
    '/bfx other off',
    '/soundeffectsparty 50',
    '/soundeffectsother 20',
    '/ambientsounds 20',
]
text = '/bfx party simple /bfx other off /soundeffectsparty 50 /soundeffectsother 20 /ambientsounds 20'
```

# Contributing

Contributions are always welcomed. Please ensure code passes `cargo test `, `cargo clippy`, and `rustfmt -v --check **/*.rs` before making pull requests.