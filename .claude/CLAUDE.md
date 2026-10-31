# プロジェクト設定

React の日本語のドキュメント (https://ja.react.dev/) を表示するアプリ。日本語のサイトなので、**翻訳しない** (`SiteProfile` の `japanese: true`。翻訳と原文のボタンと、設定のボタンも出ない)。PC (Linux, Windows, macOS) と Android。Tauri 2 (Rust) + React (TypeScript)。

```
リンクを開く ─> Rust (docs.rs): ページを取得 (ja.react.dev だけ)
            ─> React (lib/extract.ts, lib/segments.ts): 本文を取り出し、段落に分ける
            ─> React: そのまま表示 (`japanese: true` なので、翻訳しない) ─> DOMPurify ─> 表示
```

- **共通のコードはサブモジュール `shared/`** (リポジトリ [doc-translator-core](https://github.com/sluchin/doc-translator-core)) にある。画面 (`shared/ts/ui/`)、HTML の取り出しと翻訳の組み立て (`shared/ts/lib/`)、Rust の取得と翻訳とキャッシュ (`shared/rust/`)、ML Kit のプラグイン (`shared/plugins/tauri-plugin-mlkit/`)、開発用のイメージ (`shared/docker/`)。**共通のコードの変更は共通のリポジトリで行う** (このリポジトリの `shared/` の中で、直接直さない)。変えたら、共通のリポジトリをコミットして push してから、ここで、サブモジュールの参照を更新してコミットする。
- ナビゲーションドロワー (メニュー): `SiteProfile.menu` でメニューの取り方を決める。このアプリは `{ type: "html", selector: "aside nav" }` (ページの左のメニュー。サーバーが HTML に出す。セクションの見出し (`h3`) は項目ではないので、出ない。子の一覧は、`div` で包まれている)。
- このリポジトリにあるもの: サイトの設定 (`src/site.ts` の `SiteProfile`。`src-tauri/src/lib.rs` の `Site`。ホストは両方で同じにする)、`src/main.tsx`、サイトの設定のテスト (`src/site.test.ts`)、`src-tauri/` の設定とアイコン、`assets/` (アイコンの素材)。
- 翻訳のバックエンド (Google Cloud Translation、Gemini API、ML Kit) と、API キーの扱いは共通のコードの `.claude/CLAUDE.md` と README.md を見る。

**Android Studio は使わない。** ビルド、テスト、整形、静的解析は全てDocker の中で実行する。

## セキュリティの決まり (守ること)

- 取得するのは `https://ja.react.dev/` と `https://react.dev/` だけ (`docs.rs` の `is_doc_url`、`lib/docUrl.ts` の `isDocUrl`)。リダイレクトも、そのホストの中だけ。ほかのリンクはブラウザで開く。
- 表示する HTML は取得したものも、翻訳されたものも、必ず `sanitize()` (DOMPurify) を通す。`dangerouslySetInnerHTML` に渡すのはその結果だけ。
- CSP (`tauri.conf.json`) は画像を `ja.react.dev` と `data:` だけに限る。広げない。
- ドキュメントの文章をリポジトリに置かない (テストの HTML は作り物)。ページの取得は利用者がリンクを開いたときだけ (事前の取得、巡回はしない)。名前を名乗る User-Agent を付ける。

## コマンド

- `docker compose run` には必ず `--rm` を付ける。`UID` と `GID` をシェルでエクスポートしておく。
- 依存関係: `install` / テスト: `test` (サイトの設定のテストと cargo test。共通のコードのテストと 100% のカバレッジは共通のリポジトリで) / 整形の確認: `format` / 整形: `format-fix` / 静的解析: `analyze` (tsc, ESLint, Clippy) / PC のビルド: `build-desktop` / Android の APK: `build-android` / 生成物の削除: `clean`。
- Windows と macOS のビルドはそれぞれの OS で行う。画面の確認はホストの `npm run tauri:dev` か、実機。

## コーディング規約

- Prettier (TypeScript, CSS, JSON, Markdown) と rustfmt に従う。手で整形しない。ESLint と Clippy の警告をゼロにする (`-D warnings`)。
- **ファイルの先頭に著作権とライセンスを書く** (`Copyright (c) 2026 Tetsuya Higashi` と `SPDX-License-Identifier: GPL-3.0-or-later`)。
- **コメントは日本語。** 句読点は「, 」(カンマと空白) と「.」(ピリオド) にする (「、」と「。」は使わない)。読点は必要なところだけに打つ。ログとエラーのメッセージ、識別子は英語。画面に出す文字は日本語 (句読点は「、」「。」でよい)。
- 関数には TSDoc / rustdoc の説明を付ける (`@param` `@returns`)。
- `!` (非 null の表明) は使わず、`as` は直前に型が保証されているときだけ使う。`unwrap()` はテスト以外で使わない。

## 開発フロー

1. Push 時に GitHub Actions (`.github/workflows/build.yml`) が `test` `format` `analyze` `build-desktop` `build-android` (チェックアウトはサブモジュールも取る) を実行する。
2. コードを修正したら、`format-fix` → `format` → `analyze` → `test` を全て成功させる。共通のコードを変えたら、共通のリポジトリでも、`coverage` まで、成功させる。`tauri.conf.json`、`Cargo.toml`、Android の設定を変えたら、`build-desktop` と `build-android` も実行する。
3. ビルドやテストを実行できなかったときはそのことを報告する。ユーザーに依頼されない限り、`git commit` / `git push` は行わない。

## React のドキュメントの作り (サイト固有)

- 英語の `react.dev` ではなく、日本語の `ja.react.dev` を開く。日本語に訳されていないページは、サイトの側で英語のまま出る (翻訳は、しない)。
- 本文は `main article`。ページの `button` (ページのコピーなど) は取り除く。コードの例 (Sandpack) は、サーバーが HTML に出す分だけが出る (編集と実行はできない)。
- 開くのは `https://ja.react.dev/` と、英語のサイト `https://react.dev/` の全てのページ (`pathPrefix` は `/`)。最初のページは `/learn`。ツールバーのボタンで、今のページを、もう一方のホストの同じパスに切り替える (`SiteProfile` の `alt`、Rust の `Site` の `alt_host`。ページごと。英語のページも翻訳しない)。
- アイコンはオリジナル (青緑の R)。**React のロゴを使わない。**
- 画像の CSP は `ja.react.dev` と `react.dev` だけ。ほかのホストの画像が出ないときは `tauri.conf.json` の CSP に足す (理由をコメントに書く)。

## 実機で未確認

画面の見た目、React のページの取り出しと表示 (コードの例、画像、図)、Android の APK の動作 (インストール、リンク、設定、戻るボタン)、Windows と macOS のビルド。

## アイコン

`assets/` の SVG (`icon.svg` デスクトップ、`icon-bg.svg` と `icon-fg.svg` Android の背景と前景、`icon-mono.svg` テーマアイコン) と、PNG (SVG から作った 1024 px) と、`icon.json` (マニフェスト)。作り直すときは SVG から PNG を作り (`npx tauri icon assets/<名前>.svg -p 1024 -o <一時のディレクトリ>`)、`npx tauri icon assets/icon.json` を実行する (`src-tauri/icons/` と、Android の `mipmap-*` が更新される)。`tauri android init` のあとにも、必ず実行する (初期化が Tauri の既定のアイコンに戻すため)。

## コミットガイドライン

- コミットメッセージは英語。件名は Conventional Commits の `type(scope): description` (type: `feat` `fix` `docs` `style` `refactor` `test` `build` `ci` `chore`。scope: `app` `rust` `android` `docker`)。description は英語で、命令形。
- 件名のあとに空行を 1 行入れて、本文を書く。本文はハイフンで始まる箇条書き。
- コミットメッセージに `Co-Authored-By` または Claude 帰属行を含めない。
- rebase のあとの push は `git push --force-with-lease --force-if-includes origin <ブランチ名>`。

## 開発ルール

- Markdown (`*.md`) の日本語の文章は、句読点に「、」「。」を使う (「,」「.」は使わない)。インラインコード・コードブロック・URL・ファイル名・数値 (`1.2.0`、`1,000`) は対象外。見出しや番号付きリストの「1.」の形は変えない。`docs/` の生成物と、この `CLAUDE.md` は対象外。
