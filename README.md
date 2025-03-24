# tauri-plugin-locale

> This plugin only works on tauri v2, if you need the v1 plugin, feel free to submit a PR!

Why not use the `locale` method provided by tauri?

For example:

- Tauri: `zh-Hans-CN` → This plugin: `zh-CN`
- Tauri: `zh-Hant-TW` → This plugin: `zh-TW`

This simplified format is widely used in web development and i18n libraries, omitting script tags (Hans/Hant) for better compatibility.

## Platform Support

| Platform | Supported |
| -------- | --------- |
| Windows  | ✅        |
| macOS    | ✅        |
| Linux    | ✅        |
| Android  | ❌        |
| iOS      | ❌        |

## Install

```shell
cargo add tauri-plugin-locale
```

You can install the JavaScript Guest bindings using your preferred JavaScript package manager:

```shell
pnpm add tauri-plugin-locale-api
```

## Usage

`src-tauri/src/lib.rs`

```diff
pub fn run() {
    tauri::Builder::default()
+       .plugin(tauri_plugin_locale::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

`src-tauri/capabilities/default.json`

```diff
{
    ...
    "permissions": [
        ...
+       "locale:default"
    ]
}
```

Afterwards all the plugin's APIs are available through the JavaScript guest bindings:

```ts
import { getLocale } from "tauri-plugin-locale-api";

const locale = await getLocale();
console.log(locale); // en-US
```

## Methods

| Method      | Description                   |
| ----------- | ----------------------------- |
| `getLocale` | Get the locale of the system. |

## Thanks

- Use [current_locale](https://github.com/i509VCB/current_locale) to get the locale of the system.

## Who's Use It

- [EcoPaste](https://github.com/EcoPasteHub/EcoPaste) - Open source cross-platform clipboard management tool.
