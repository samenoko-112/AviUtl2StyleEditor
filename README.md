# Tauri + Vue + TypeScript

This template should help get you started developing with Vue 3 and TypeScript in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Type Support For `.vue` Imports in TS

Since TypeScript cannot handle type information for `.vue` imports, they are shimmed to be a generic Vue component type by default. In most cases this is fine if you don't really care about component prop types outside of templates. However, if you wish to get actual prop types in `.vue` imports (for example to get props validation when using manual `h(...)` calls), you can enable Volar's Take Over mode by following these steps:

1. Run `Extensions: Show Built-in Extensions` from VS Code's command palette, look for `TypeScript and JavaScript Language Features`, then right click and select `Disable (Workspace)`. By default, Take Over mode will enable itself if the default TypeScript extension is disabled.
2. Reload the VS Code window by running `Developer: Reload Window` from the command palette.

You can learn more about Take Over mode [here](https://github.com/johnsoncodehk/volar/discussions/471).

# AviUtl2 Style Editor

AviUtl2のstyle.confファイルを視覚的に編集するためのデスクトップアプリケーションです。

## 機能

- **フォント設定**: 各要素のフォントファミリーとサイズを設定
- **色設定**: すべての色項目をカラーピッカーで編集
- **レイアウト設定**: 各要素のサイズを数値入力とスライダーで調整
- **フォーマット設定**: フッター表示のカスタマイズ

## 技術スタック

- **フロントエンド**: Vue.js 3 + TypeScript
- **バックエンド**: Rust (Tauri)
- **UI**: カスタムCSS

## 開発

### 必要な環境

- Node.js 18+
- Rust 1.70+
- Tauri CLI

### セットアップ

```bash
# 依存関係のインストール
npm install

# 開発サーバーの起動
npm run tauri dev
```

### ビルド

```bash
# 本番ビルド
npm run tauri build
```

## 使用方法

1. アプリケーションを起動
2. 自動的にAviUtl2のstyle.confファイルを読み込み
3. 各タブで設定を編集
4. 「保存」ボタンで変更を適用
5. 「ユーザー設定として保存」でProgramDataに保存

## ファイルパス

- **デフォルト設定**: `C:\Program Files\AviUtl2\style.conf`
- **ユーザー設定**: `%PROGRAMDATA%\aviutl2\style.conf`

## ライセンス

MIT License
