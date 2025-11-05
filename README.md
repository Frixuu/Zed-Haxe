# Haxe language support for Zed

![Screenshot of Zed editor](media/example-gruvbox.webp)

## Usage

The syntax highlighting should appear immediately.

The backing language server should be downloaded automatically in the background.

This extension detects all `.hxml` files in the workspace root,
and then tries to choose the best one as configuration.
This decision might be wrong!
To explicitly use a specific `.hxml` file,
create a `.zed/settings.json` file in your project root:

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

  [(reference)](https://github.com/vshaxe/haxe-language-server/blob/65ba91ce13e413fe721d371cdf9e39024a53f2ec/src/haxeLanguageServer/Configuration.hx#L136)

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

## Usage (Lime/OpenFL/HaxeFlixel)

The Haxe language server does not natively understand Lime projects,
supporting `.hxml` files only.
However, Lime lets you easily generate them.

Assuming that

- you have a `Project.xml` file in your workspace root, and
- you're targetting `html5`,

you can run in your favorite shell:

```sh
lime build html5
lime display html5 > html5.hxml
```

This creates a `html5.hxml` file telling Haxe which libraries to include
and what platform to target.
See [the instructions above](#usage) on how to tell Zed to use this file.

## Usage (other)

Many other Haxe toolchains can create `.hxml` files, including:

```sh
nme prepare html5
```

```sh
ceramic clay hxml web > web.hxml
```

## Install nightly

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

## Credits

- [tree-sitter grammar by tong](https://github.com/tong/tree-sitter-haxe)
- [haxe-language-server by the vshaxe team](https://github.com/vshaxe/haxe-language-server), MIT-licensed
