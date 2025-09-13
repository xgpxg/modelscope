# modelscope

[中文](README_zh.md) | [English](README.md)

![Release](https://github.com/xgpxg/modelscope/actions/workflows/publish.yml/badge.svg)
![Crates.io](https://img.shields.io/crates/d/modelscope)

用于从Modelscope下载模型的CLI工具。

功能：

- ⬇️ 从Modelscope下载模型
- 🫏 显示进度条
- ⚡ 多线程下载
- 🔗 断点续传

支持的操作系统：

- Windows
- macOS
- Linux

## 安装

可以通过以下方式之一安装：

- 使用Cargo安装

```shell
cargo install modelscope
```

- 使用预编译的包
  从 [发布页面](https://github.com/xgpxg/modelscope/releases) 下载适合你的操作系统的包，然后解压。

## 使用方式：

```shell
modelscope -m <MODEL_ID> -s <SAVE_DIR>
```

![img.png](screenshot.png)

## 命令：

```shell
Usage: modelscope [OPTIONS] --model-id <MODEL_ID>

Options:
  -m, --model-id <MODEL_ID>  model id
  -s, --save-dir <SAVE_DIR>  save dir, if not set, use current dir, will create if not exists [default: ]
  -h, --help                 Print help
  -V, --version              Print version
```

## 在lib中使用

添加依赖：

```shell
cargo add modelscope
```

示例：

```rust
use modelscope::ModelScope;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let model_id = "Qwen/Qwen3-0.6B";
    let save_dir = "./data";
    ModelScope::download(model_id, save_dir).await?;

    Ok(())
}
```