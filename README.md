# PureScript Iris for Zed

[Zed](https://zed.dev) extension for [Iris](https://iris-lang.com), a superset of [PureScript](https://www.purescript.org/). Not published on [zed.dev/extensions](https://zed.dev/extensions) yet.

Note: This extension supports the [Iris language server](https://iris-lang.com/docs/language-server/) only. For full [PureScript](https://www.purescript.org/) language support (file types, grammar, syntax highlighting etc.), the [PureScript extension](https://github.com/zed-extensions/purescript) is **required**.

## Setup

1. Install the [PureScript extension](https://github.com/zed-extensions/purescript): open the extension page (`Ctrl/Cmd-Shift-X`), search for `PureScript`, click `Install`.
2. Install this extension as dev extension (until it's published):
   - Install [Rust via rustup](https://www.rust-lang.org/tools/install).
   - Clone this repository.
   - In `Zed` open the extension page (`Ctrl/Cmd-Shift-X`), click `Install Dev Extension` and select the cloned folder.
3. Turn off `purescript-language-server` in Zed's global (`~/.config/zed/settings.json`) or project (`.zed/settings.json`) settings. Otherwise it runs next to Iris and you get duplicate diagnostics, hovers and completions.

   ```json
   {
     "languages": {
       "PureScript": {
         "language_servers": ["iris", "!purescript-language-server", "..."]
       }
     }
   }
   ```

4. To create and run any Iris project make sure [Node](https://nodejs.org/en/download) and [Spago](https://github.com/purescript/spago) are installed, see Iris -> Getting started -> [Before you begin](https://iris-lang.com/docs/getting-started/#before-you-begin).

## Iris binary

The extension uses the first of these:

1. **`lsp.iris.binary.path`** from your settings (see [Custom binary](#custom-binary)).
2. **`iris` on your `PATH`**, see Iris -> [Installation](https://iris-lang.com/docs/installation/).
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

`arguments` is required, because Zed passes no default arguments with a custom `path`. Run `iris lsp --help` to see all available options.

Note: Without `path`, you can still set `arguments` (they replace the default `["lsp", "--stdio"]`) and `env` (added to your shell environment).

## Settings

`lsp.iris.settings` is passed to Iris as its `iris.server` settings, and changes apply without restarting the server. For example, to update diagnostics while you type:

```json
{
  "lsp": {
    "iris": {
      "settings": {
        "diagnostics": { "onChange": true }
      }
    }
  }
}
```

For all settings and their defaults, see Iris -> Language server -> [Settings](https://iris-lang.com/docs/language-server/#settings).

## Formatting

Iris formats PureScript files through the language server, see Iris -> Formatting -> [Format in your editor](https://iris-lang.com/docs/formatting/#format-in-your-editor). To make it Zed's formatter for PureScript:

```json
{
  "languages": {
    "PureScript": {
      "formatter": "language_server"
    }
  }
}
```

## Logs

To see the messages between Zed and Iris, select `RPC Messages` in the LSP Logs panel (`debug: open language server logs`).

## Development

After changes, run `zed: rebuild dev extension`. Start Zed with `zed --foreground` to see the extension's log output.

See Zed -> [Developing Extensions](https://zed.dev/docs/extensions/developing-extensions).

## License

[MIT](LICENSE)
