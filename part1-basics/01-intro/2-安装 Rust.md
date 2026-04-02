## 1.2 安装 Rust

### macOS / Linux 安装

打开终端，执行以下命令：

```bash
# 下载并运行安装脚本
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**这个命令做了什么？**

1. `curl`：下载工具
2. `--proto '=https'`：只允许 HTTPS 协议（安全）
3. `--tlsv1.2`：最低 TLS 版本 1.2
4. `-sSf`：安静模式 + 显示错误 + 失败时不输出
5. `| sh`：将下载的内容传给 shell 执行

安装完成后，你会看到：

```
Rust is installed now. Great!

To get started you may need to restart your current shell.
This would reload your PATH environment variable to include
Cargo's bin directory ($HOME/.cargo/bin).

To configure your current shell, run:
source "$HOME/.cargo/env"
```

执行：

```bash
source "$HOME/.cargo/env"
```

这会加载 Rust 的环境变量。

### Windows 安装

1. 访问 [https://rustup.rs](https://rustup.rs)
2. 点击 `rustup-init.exe` 下载
3. 双击运行，一路按回车即可

### 验证安装

```bash
# 检查 Rust 编译器版本
rustc --version

# 检查 Cargo（包管理器）版本
cargo --version

# 检查 rustup（工具链管理器）版本
rustup --version
```

你应该看到类似输出：

```
rustc 1.75.0 (82e1608df 2023-12-21)
cargo 1.75.0 (1d8b05cdd 2023-11-20)
rustup 1.26.0 (5af9b9484 2023-04-05)
```

> 💡 **提示**：版本号可能不同，这没关系。建议保持最新。

---

---

## 下一步

[Rust 工具链详解](../3-Rust 工具链详解.md)