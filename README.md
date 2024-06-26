# LuaFmt - Lua Formatter / Pretty-Printer

A tool for formatting Lua code, written in Rust. Features:
* provides [format options](configuration.md);
* supports reading configuration options from the `.luafmt*.lua` in the source file directory. If file is not found, it recursively searches the parent directories for the configuration file;
* can update lua files in place (`-i` option), process multiple files/directories or all files in a directory and possible subdirectories (`-r` option);
* supports partial formatting of only a few consecutive top-level statements (option `--line_range=30:40`).
* plugin for `vim`: [2lx/vim-luafmt](https://github.com/2lx/vim-luafmt)

## Installing from source

Requirements:
* [`rustup`](https://www.rust-lang.org/tools/install), installation on CentOS: 
```
$ curl https://sh.rustup.rs -sSf | sh
$ export PATH=$PATH:$HOME/.cargo/bin
```

Building and installation:
```
$ cd TEMP
$ git clone git@github.com:2lx/luafmt.git .
$ cd luafmt
$ cargo build --release
```
Then add `{PATH}/target/release` to `$PATH`.

## Usage

```
luafmt [-ivr] [configuration options] {sources/directories}
```

[Default config](.luafmt.lua) (put it in your project or any parent directory).

There are several supported types of `LuaFmt` configuration options format. The examples set the string value `" "` for `hint_table_construtor` option (it may depend on your shell command interpreter):
* `luafmt "--hint_table_constructor= " FILES`
* `luafmt --hint_table_constructor=" " FILES`
* `luafmt --hint_table_constructor=\  FILES`

## Formatting features
* ...

## Limitations
* supports only valid lua scripts.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
