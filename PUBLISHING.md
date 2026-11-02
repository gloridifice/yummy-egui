# 发布到 crates.io

## 首次发布前

本项目采用 [MIT 许可证](LICENSE)，`Cargo.toml` 已声明 `license = "MIT"`。发布包包含根目录许可证，以及 `src/theme/default/LICENSE-wgpu_pbr.txt` 中保留的主题许可声明。

本地使用现有 Rust 工具链即可，无需额外安装旧版本。本项目声明 Rust 1.92 为最低支持版本；CI 使用 Windows，分别在 Rust 1.92.0 和 stable 上检查编译、测试、文档和打包，它不代表 Linux/macOS 的完整测试。

## 本地预检

在仓库根目录运行：

```sh
cargo fmt --all -- --check
cargo check --locked --all-targets --all-features
cargo test --locked
cargo test --locked --all-features
cargo doc --locked --no-deps --all-features
cargo package-list
cargo publish-check
cargo publish-check --all-features
```

`publish-check` 是 `.cargo/config.toml` 中的别名，执行 `cargo publish --dry-run --locked --registry crates-io`，不会上传。它验证打包和编译，不会代替单元测试，也不保证注册表最终接受发布。

默认要求工作区中的待打包文件已提交。开发期间可运行 `cargo publish-check --allow-dirty` 检查未提交修改；正式发布不要加这个参数。检查文件清单时，应看到源码、内嵌 TOML 主题、主题许可、示例、README 和项目 MIT 许可证，不应有凭证或构建产物。

## 本地发布

1. 登录 [crates.io](https://crates.io/)，在账号设置中验证邮箱。
2. 在 [API Tokens](https://crates.io/settings/tokens) 创建有发布权限的令牌，限制到 `yummy_egui`，并设置有效期。首次创建 crate 时，需要令牌具备创建新 crate 的权限。
3. 运行以下命令，在 Cargo 的提示中输入令牌，不要把令牌写进命令、仓库或聊天：

   ```sh
   cargo login --registry crates-io
   ```

   Cargo 通常将令牌保存在用户级 `$CARGO_HOME/credentials.toml`（默认 `~/.cargo/credentials.toml`，Windows 为 `%USERPROFILE%\.cargo\credentials.toml`）。该文件按秘密处理，不要提交。

4. 确认 `Cargo.toml` 版本、许可证和预检结果，提交准备发布的修改，然后运行：

   ```sh
   cargo publish --locked --registry crates-io
   ```

crates.io 的包名首次发布时才会确定归属；`yummy-egui` 与 `yummy_egui` 不能作为两个独立名称使用。同一版本不能覆盖上传。后续发布必须递增 `Cargo.toml` 的版本，并刷新、提交 `Cargo.lock`，重新完成预检。

## 使用 cargo-release

也可以用 `cargo-release` 代替上面的手动版本管理和发布命令。根目录的 `release.toml` 限定只能从 `master` 发布，标签格式为 `v{{version}}`，并在发布前运行 `cargo test --locked --all-features`。

先完成登录和预检，提交当前修改，再运行：

```sh
# 发布 Cargo.toml 中的当前版本
cargo release
cargo release --execute

# 后续递增补丁版本
cargo release patch
cargo release patch --execute
```

不加 `--execute` 时只预演；正式执行会发布到 crates.io、创建标签并推送到 `origin`，版本变更会自动提交。测试 hook 不代替上面的完整预检。不要再用普通 `cargo publish` 或 Actions 重复发布同一版本。

## 从 GitHub Actions 手动发布

工作流文件：`.github/workflows/publish.yml`。普通 push、PR 不会触发上传。

1. 将配置推送到 GitHub 默认分支。
2. 在仓库 **Settings → Environments** 中创建 `crates-io`，配置允许发布的分支；可用时添加 required reviewers。
3. 在该环境中添加名为 `CARGO_REGISTRY_TOKEN` 的 secret，值为上述 crates.io 发布令牌。不要使用 `GITHUB_TOKEN` 代替。
4. 打开 **Actions → Publish to crates.io → Run workflow**，选择待发布的分支或标签，输入与 `Cargo.toml` 完全相同的版本号，例如 `0.1.1`。

工作流先运行 CI，再校验版本和许可证，最后使用环境 secret 发布。没有配置令牌、版本不符或检查失败时不会上传。不要对同一版本同时执行本地和 Actions 发布。
