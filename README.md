# react-doc-ja

React の日本語のドキュメント (https://ja.react.dev/) を表示するアプリです。ドキュメントは**日本語のサイトにあるので、翻訳しません** (翻訳のサービスも、API キーも要りません)。PC (Linux, Windows, macOS) と Android で動きます。

- **Tauri 2** (Rust) + **React** (TypeScript)。
- 英語の `react.dev` ではなく、日本語の `ja.react.dev` を開きます。日本語に訳されていないページは、サイトの側で、英語のまま出ます。

## 仕組み

```
リンクを開く ─> Rust: ドキュメントのページを取得 (ja.react.dev と react.dev だけ)
            ─> React: 本文を取り出し (リンクと画像は絶対 URL に)
            ─> React: そのまま表示 (リンクは、アプリの中で開く)
```

- 表示する HTML は、必ず `DOMPurify` を通します (スクリプトなどを除きます)。
- アプリの中で開くのは `https://ja.react.dev/` と `https://react.dev/` のページだけです。ほかのリンクはブラウザで開きます。最初のページは `/learn` です。

## 使い方

1. アプリを起動します。トップ (`/learn`) が開きます。リンクを押すと、そのページを取得して表示します。
2. 「英語のサイトへ」で、今のページを、英語のサイト (`react.dev`) の同じページに切り替えます (ページごと。英語のページは、翻訳しません。「日本語のサイトへ」で戻ります)。
3. 左上の ☰ で**メニュー (ナビゲーションドロワー)** を開きます。メニュー (目次) はページの左のメニューから読みます。項目を押すと、そのページを開いて、閉じます。
4. **読んだ場所**: 右端にスクロールバーを出します。つまみを押して動かすと、つまみの横に今のページの、読み進めた場所 / ページ全体 (「3 / 12 画面 (25%)」) が出ます。最後に開いたページと、ページごとの読んだ位置を端末に覚えて、次の起動で戻します。
5. **トップとしおり**: ツールバーの ⌂ でトップのページに戻ります。☆ で今のページにしおりを付けて、メニューの「しおり」から開きます (外すのは ✕)。

## 共通のコード (サブモジュール)

画面、HTML の取り出し、翻訳、取得、キャッシュ、ML Kit のプラグインは**共通のリポジトリ** [doc-translator-core](https://github.com/sluchin/doc-translator-core) にあります (サブモジュール `shared/`)。このリポジトリには React だけの設定 (`src/site.ts` と `src-tauri/src/lib.rs` の `SITE`) と、アイコンと、アプリの設定だけがあります。姉妹アプリは [zephyr-doc-trans](https://github.com/sluchin/zephyr-doc-trans) (Zephyr)、[kotlin-doc-trans](https://github.com/sluchin/kotlin-doc-trans) (Kotlin)、[typescript-doc-trans](https://github.com/sluchin/typescript-doc-trans) (TypeScript) です。このアプリは、共通のコードを使いますが、翻訳しません (`SiteProfile` の `japanese`)。

```bash
git clone --recurse-submodules git@github.com:sluchin/react-doc-ja.git   # 取り込み済みなら: git submodule update --init
```

## 開発 (Docker)

ビルドと検査は全て Docker の中で行います (ホストに Node.js や Rust は要りません)。`UID` と `GID` をシェルでエクスポートしておきます。

```bash
export UID GID=$(id -g)
docker compose run --rm install          # 依存関係 (npm ci)
docker compose run --rm test             # サイトの設定のテストと cargo test (共通のコードのテストは shared で)
docker compose run --rm format           # 整形の確認 (format-fix で整形)
docker compose run --rm analyze          # tsc, ESLint, Clippy
docker compose run --rm build-desktop    # PC (Linux) のデバッグ版
docker compose run --rm build-android    # Android のデバッグ版 APK
```

- Android の APK: `src-tauri/gen/android/app/build/outputs/apk/universal/debug/app-universal-debug.apk` (arm64)。
- Windows と macOS はそれぞれの OS で `npm ci && npx tauri build` を実行します (Rust, Node.js, 各 OS の Tauri の前提が要ります。https://tauri.app/start/prerequisites/)。
- 画面の確認 (`npm run tauri:dev`) はホストで行います。

## リリース

`v0.1.0` のようなタグを push すると、GitHub Actions の `release.yml` が PC (Windows、macOS、Linux のインストーラ。Windows は exe 単体の zip も) と Android の APK を作って、GitHub Releases の**ドラフト**に載せます (中身を確かめて、手で公開します)。APK はデバッグ鍵で署名したデバッグ版です。タグを打たずに試すときは Actions の Release を手動で実行します (成果物は Artifacts に保存されます)。

## ライセンス

GPL-3.0-or-later。React のドキュメントは Meta のものです (利用条件はサイトで確認してください)。このリポジトリにはドキュメントの文章を含めません (実行のときに、取得します)。
