# Zed extension for PureScript Iris

> [!NOTE]
> Not published on [zed.dev/extensions](https://zed.dev/extensions) yet.

[Zed](https://zed.dev) extension for [Iris](https://iris-lang.com), a superset of [PureScript](https://www.purescript.org/).

It supports the [Iris language server](https://iris-lang.com/docs/language-server/) only. For full PureScript language support (file types, grammar, syntax highlighting etc.), the [PureScript extension](https://github.com/zed-extensions/purescript) is still **required**.

## Setup

1. Install the [PureScript extension](https://github.com/zed-extensions/purescript): `Zed` -> `Extensions` -> Search `PureScript` -> Install
2. Install Iris extension as dev extension (until it's published, see [#1](https://github.com/sectore/purescript-iris-zed/issues/1)):
   - Make sure [Rust](https://www.rust-lang.org) is installed on your machine
   - Clone this repository
   - `Zed` -> `Extensions` -> `Install Dev Extension` -> select the cloned folder
3. Enable the Iris language server and disable the PureScript one:

   ```json
   {
     "languages": {
       "PureScript": {
         "language_servers": ["iris", "!purescript-language-server", "..."]
       }
     }
   }
   ```

## Iris binary

The extension uses the first of these:

1. **`lsp.iris.binary.path`** from your settings (see [Custom binary](#custom-binary)).
2. **`iris` on your `PATH`**, see [Iris docs](https://iris-lang.com/docs) -> [Installation](https://iris-lang.com/docs/installation/).
3. **A download** of the [latest Iris release](https://github.com/purefunctor/purescript-iris/releases), stored in Zed's extension directory.

## Custom binary

```json
{
  "lsp": {
    "iris": {
      "binary": {
        "path": "/path/to/iris",
        "arguments": ["lsp", "--stdio"]
      }
    }
  }
}
```

`arguments` is required, because Zed passes **no** default arguments with a custom `path`. Run `iris lsp --help` to see all available options.

Note: Without `path`, you can still set `arguments` (they replace the default `["lsp", "--stdio"]`) and `env` (added to your shell environment).

## Settings

Example: update diagnostics while you type and format via Iris:

```json
{
  "lsp": {
    "iris": {
      "settings": {
        "diagnostics": { "onChange": true }
      }
    }
  },
  "languages": {
    "PureScript": {
      "formatter": "language_server"
    }
  }
}
```

- All settings and their defaults: [Iris docs](https://iris-lang.com/docs) -> Language server -> [Settings](https://iris-lang.com/docs/language-server/#settings).
- Formatting: [Iris docs](https://iris-lang.com/docs) -> Formatting -> [Format in your editor](https://iris-lang.com/docs/formatting/#format-in-your-editor).

## Logs

To see the messages between Zed and Iris, in Zed select `RPC Messages` in the LSP Logs panel (`debug: open language server logs`).

## Development

See [Zed docs](https://zed.dev/docs/) -> [Developing Extensions](https://zed.dev/docs/extensions/developing-extensions).

## License

[MIT](LICENSE)
