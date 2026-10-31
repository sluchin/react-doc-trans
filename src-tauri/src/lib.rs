// Copyright (c) 2026 Tetsuya Higashi
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! React の日本語のドキュメント (ja.react.dev) を表示するアプリ (日本語なので, 翻訳しない). 中身は, 共通のコード (shared).

use doc_translator_core::Site;

/// 取得するサイト (TypeScript 側の `src/site.ts` と, ホストを同じにする).
const SITE: Site = Site {
    host: "ja.react.dev",
    alt_host: Some("react.dev"),
    user_agent: "react-doc-ja/0.1 (+https://github.com/sluchin/react-doc-ja)",
};

/// アプリを起動する.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    doc_translator_core::builder(SITE)
        .plugin(tauri_plugin_mlkit::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
