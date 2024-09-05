# Haxe language support for Zed

## Usage

Install the extension.

The syntax highlighting should appear immediately.

The backing language server should be downloaded automatically in the background.

To make the LSP use a specific `.hxml` configuration, create a `.zed/settings.json` file in your project root:

```json
{
  "lsp": {
    "haxe-language-server": {
      "settings": {
        "configuration-file": "DevEnv.hxml"
      }
    }
  }
}
```

## Credits

- [tree-sitter grammar by vantreeseba](https://github.com/vantreeseba/tree-sitter-haxe),
MIT-licensed
- [haxe-language-server by the vshaxe team](https://github.com/vshaxe/haxe-language-server), MIT-licensed
