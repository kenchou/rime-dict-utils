# rime-dict-utils

Rime 输入法字典文件条目消重。

## Build

```sh
cargo build
```

## Usage

```sh
Usage: rime-dict-utils [OPTIONS]

Options:
  -c, --config <FILE>  Sets a custom config file
  -h, --help           Print help
  -V, --version        Print version
```

example:

```sh
rime-dict-utils -c /path/to/rime/luna_pinyin/luna_pinyin.sgplus2.dict.yaml > /tmp/luna_pinyin/luna_pinyin.sgplus2.dict.yaml
```
