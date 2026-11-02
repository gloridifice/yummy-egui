<div align="center">

# yummy_egui

[English](../README.md) | 简体中文 | [日本語](ja-JP.md) | [Français](fr-FR.md)

</div>

基于 egui 0.34.3 的控件、参数表单、选择列表和 TOML 主题库，不绑定渲染后端。

渲染后端、字体、窗口、选择状态和持久化由应用管理。`eframe` 仅用于示例程序。

## 快速开始

在应用初始化时应用一次主题，然后在已有 UI 中绘制控件：

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

字体由宿主应用提供，`Theme::apply` 不会安装或替换字体。若要保留样式编辑器的实时修改，不要每帧重新应用主题。

在仓库根目录运行独立示例：

```sh
cargo run --example demo
cargo run --example style_editor
```

## 功能

- 参数滑块支持整数、浮点数、对数刻度、步长、单位后缀和悬停动画。
- 双列参数表单支持标签换行、提示文本、分组标题和显式行 ID。
- 可滚动的选择列表支持禁用行，选择状态由应用管理。
- 提供面板按钮、操作按钮行、互斥选项、分段控件和 H1–H3 标题。
- 内置 TOML 主题，支持显式加载外部主题；样式按 Context 隔离。
- 实时样式编辑器可调整自定义控件和常用 egui 设置；修改仅保存在内存中，不包含 dock 样式。
- 通过可选的 `dock` feature 集成 `egui_dock` 0.19。

## 主题

以 [`src/theme/default/`](../src/theme/default/) 中的文件为起点：

| 文件 | 配置内容 |
| --- | --- |
| `egui.toml` | egui 基础样式 |
| `yummy_egui.toml` | 自定义控件、列表和标题 |
| `egui_dock.toml` | Dock 样式，仅在启用 `dock` feature 时生效 |

从已有目录加载主题，再显式应用：

```rust
fn load_theme(ctx: &egui::Context) -> Result<(), yummy_egui::ThemeError> {
    let theme = yummy_egui::Theme::load_dir("themes/my-theme")?;
    theme.apply(ctx);
    Ok(())
}
```

缺失的文件使用对应的内置默认值。外部文件替换整份文档，而非逐字段合并，建议先复制内置文件再修改。无法读取或格式无效的文件会返回错误。加载操作不会监听文件变化，也不会回写修改。若要直接传入 TOML 字符串，可使用 `Theme::from_sources(ThemeSources { .. })`。

请将已有外部主题文件 `charming_egui.toml` 重命名为 `yummy_egui.toml`，旧文件名不再加载。TOML 结构和公开 Rust API 名称保持不变。

从 egui 0.33 迁移外部 `egui.toml` 时，若文件包含 `[debug]` 段，请在其中补上 `warn_if_rect_changes_id = false` 和 `show_focused_widget = false`。

使用 dock 样式时，在依赖中启用 `features = ["dock"]`，再将 `theme.dock_style().clone()` 传给 `DockArea::style(...)`。`Theme::apply` 不会自动设置 dock 区域的样式。

## 从源码构建

安装 Rust 1.92+ 后，在仓库根目录执行：

```sh
cargo build --release
cargo check --examples --all-features
cargo test --all-features
cargo doc --no-deps --all-features --open
```

示例程序需要图形环境，以及 `eframe` 的 Glow 后端所需的系统依赖。

## 许可证

本项目采用 [MIT 许可证](../LICENSE)。改编的 egui 主题保留其 [wgpu_pbr MIT 许可声明](../src/theme/default/LICENSE-wgpu_pbr.txt)。

## 开发

- [文档索引（英文）](../doco/README.md)
- [架构（英文）](../doco/architecture.md)与[开发指南（英文）](../doco/development.md)
- [发布到 crates.io（英文）](../doco/publishing.md)
- [公开 API](../src/lib.rs)
- [控件展示](../src/demo.rs)与[独立示例](../examples/demo.rs)
- [样式编辑器示例](../examples/style_editor.rs)
- [主题加载](../src/theme/mod.rs)与[自定义样式定义](../src/theme/style.rs)
