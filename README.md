# Haxe language support for Zed

## Installation

To use dev Zed extensions, you will need to have [Rust compiler installed](https://rustup.rs/).

You will also need to `git clone` this repository.

> [!TIP]  
> If you're reading this with a web browser,
> you might also click the green `< > Code` button at the top of the page
> and choose `Download ZIP.`

In the `Extensions` panel (`Ctrl+Shift+X`), click `Install Dev Extension` in the top right corner.
Choose the location of the downloaded directory.

> [!IMPORTANT]  
> This process might take a few minutes.  
> If this is your first time installing dev extensions,
> Zed may need to download its WASM toolchain (roughly 500MiB).  
> If you're not sure if the installation process is still ongoing,
> consult the editor logs at `Ctrl+Shift+P` > `zed: open log`.

## Usage

The syntax highlighting should appear immediately.

The backing language server should be downloaded automatically in the background.

To make the LSP use a specific `.hxml` configuration, create a `.zed/settings.json` file in your project root:

```json
{
  "lsp": {
    "haxe-language-server": {
      "initialization_options": {
        "displayArguments": ["DevEnv.hxml"]
      }
    }
  }
}
```

<details>
  <summary>Additional settings you may want to pass to the language server: (non-exhaustive)</summary>

  [(reference)](https://github.com/vshaxe/haxe-language-server/blob/9c3114de15bfd8096833ee50aab131459347e3f7/src/haxeLanguageServer/Configuration.hx#L134)

  ```json
  {
    "lsp": {
      "haxe-language-server": {
        "initialization_options": {
          "displayServerConfig": {
            "path": "/your/custom/haxe/here"
          }
        },
        "settings": {
          "haxe": {
            "buildCompletionCache": true,
            "displayHost": "127.0.0.1",
            "displayPort": 6001
          }
        }
      }
    }
  }
  ```
</details>

## Credits

- [tree-sitter grammar by vantreeseba](https://github.com/vantreeseba/tree-sitter-haxe),
MIT-licensed
- [haxe-language-server by the vshaxe team](https://github.com/vshaxe/haxe-language-server), MIT-licensed
