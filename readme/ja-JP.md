<div align="center">

# yummy_egui

[English](../README.md) | [简体中文](zh-CN.md) | 日本語 | [Français](fr-FR.md)

</div>

egui 0.33.3 向けの、描画バックエンドに依存しないウィジェット、パラメータフォーム、選択リスト、TOML テーマのライブラリです。

描画バックエンド、フォント、ウィンドウ、選択状態、永続化はアプリケーション側で管理します。`eframe` はサンプルでのみ使用します。

## クイックスタート

初期化時にテーマを一度適用し、既存の UI 内でウィジェットを描画します。

```rust
use yummy_egui::{ParameterList, ParameterSlider, Theme};

fn setup(ctx: &egui::Context) {
    Theme::default().apply(ctx);
}

fn settings_ui(ui: &mut egui::Ui, gain: &mut f32) {
    ParameterList::new("settings").show(ui, |rows| {
        rows.row("gain", "Gain", "Output level", |ui| {
            ui.add(ParameterSlider::new(gain, 0.0..=1.0).max_decimals(2));
        });
    });
}
```

フォントはホスト側で用意します。`Theme::apply` はフォントの追加や置き換えを行いません。スタイルエディターでの変更を維持するには、毎フレームのテーマ再適用を避けてください。

リポジトリのルートでサンプルを実行できます。

```sh
cargo run --example demo
cargo run --example style_editor
```

## 機能

- 整数、浮動小数点数、対数目盛、ステップ、接尾辞、ホバーアニメーションに対応するパラメータスライダー。
- ラベルの折り返し、ツールチップ、グループ見出し、明示的な行 ID に対応する 2 列のパラメータフォーム。
- 無効な行を表示できるスクロール可能な選択リスト。選択状態はアプリケーション側で管理。
- パネルボタン、アクション行、排他的な選択肢、セグメントコントロール、H1–H3 見出し。
- 組み込み TOML テーマ、外部テーマの明示的な読み込み、Context ごとのスタイル。
- カスタムウィジェットと主な egui 設定を調整するライブスタイルエディター。変更はメモリ内のみで、dock スタイルは対象外。
- `dock` feature による、オプションの `egui_dock` 0.18 連携。

## テーマ

[`src/theme/default/`](../src/theme/default/) のファイルを元に編集できます。

| ファイル | 設定対象 |
| --- | --- |
| `egui.toml` | egui の基本スタイル |
| `charming_egui.toml` | カスタムウィジェット、リスト、見出し |
| `egui_dock.toml` | Dock スタイル。`dock` feature 有効時のみ |

既存のディレクトリから読み込み、明示的に適用します。

```rust
fn load_theme(ctx: &egui::Context) -> Result<(), yummy_egui::ThemeError> {
    let theme = yummy_egui::Theme::load_dir("themes/my-theme")?;
    theme.apply(ctx);
    Ok(())
}
```

存在しないファイルには対応する組み込みの既定値を使用します。外部ファイルはフィールド単位ではなく文書全体を置き換えるため、同梱ファイルをコピーしてから編集してください。読み取り不能または無効なファイルはエラーになります。ファイルの監視や変更の書き戻しは行いません。TOML 文字列を直接渡す場合は `Theme::from_sources(ThemeSources { .. })` を使用します。

Dock のスタイルには、依存設定で `features = ["dock"]` を有効にし、`theme.dock_style().clone()` を `DockArea::style(...)` に渡してください。`Theme::apply` は dock 領域にスタイルを自動適用しません。

## ソースからビルド

Rust 1.88+ をインストールし、リポジトリのルートで実行してください。

```sh
cargo build --release
cargo check --examples --all-features
cargo test --all-features
cargo doc --no-deps --all-features --open
```

サンプルにはグラフィカル環境と、`eframe` の Glow バックエンドに必要なシステム依存ライブラリが必要です。

## ライセンス

プロジェクト全体のライセンスはまだ宣言されていません。改変した egui テーマには別途 [wgpu_pbr の MIT ライセンス表記](../src/theme/default/LICENSE-wgpu_pbr.txt)がありますが、crate 全体のライセンスを定めるものではありません。

## 開発

- [公開 API](../src/lib.rs)
- [ウィジェットギャラリー](../src/demo.rs)と[単独実行サンプル](../examples/demo.rs)
- [スタイルエディターのサンプル](../examples/style_editor.rs)
- [テーマの読み込み](../src/theme/mod.rs)と[カスタムスタイルの定義](../src/theme/style.rs)
