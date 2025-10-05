# httpie

一个用 Rust 编写的简单 HTTP 命令行客户端，类似于 Python 的 httpie 工具。

## 功能特性

- 🚀 支持 GET 和 POST HTTP 请求
- 🎨 彩色终端输出
- 📝 JSON 响应自动格式化
- ✅ URL 和参数验证
- ⚡ 基于 tokio 的异步处理

## 安装

确保您已安装 Rust 和 Cargo，然后克隆项目：

```bash
git clone <repository-url>
cd httpie
```

## 使用方法

### 发送 GET 请求

```bash
cargo run -- get https://httpbin.org/get
```

### 发送 POST 请求

```bash
# 发送键值对数据（将作为 JSON 发送）
cargo run -- post https://httpbin.org/post name=alice age=25 city=beijing
```

### 命令格式

```bash
# GET 请求
httpie get <URL>

# POST 请求
httpie post <URL> [key=value]...
```

## 示例

```bash
# 简单 GET 请求
httpie get https://api.github.com/users/octocat

# 带数据的 POST 请求
httpie post https://httpbin.org/post username=admin password=secret

# 多个键值对
httpie post https://httpbin.org/post name=John email=john@example.com age=30
```

## 依赖库

- `reqwest` - HTTP 客户端
- `tokio` - 异步运行时
- `clap` - 命令行参数解析
- `serde` - JSON 序列化/反序列化
- `colored` - 终端彩色输出
- `jsonxf` - JSON 格式化
- `mime` - MIME 类型处理
- `anyhow` - 错误处理

## 构建

```bash
# 开发模式运行
cargo run -- [参数]

# 构建发布版本
cargo build --release

# 运行测试
cargo test
```
