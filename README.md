# Code Trio

一个使用 Rust、Python 和 Go 三种编程语言实现的链表数据结构项目。这个项目展示了如何在不同的编程语言中实现相同的数据结构和算法，并提供了一个统一的接口来运行和测试这些实现。

## 项目结构

```
.
├── packages/
│   ├── rust/          # Rust 实现
│   ├── python/        # Python 实现
│   └── go/            # Go 实现
├── scripts/           # 运行脚本
├── Makefile          # 构建和测试命令
└── LICENSE           # 许可证文件
```

## 功能特点

- 在三种不同的编程语言中实现链表数据结构
- 统一的测试框架
- 跨语言性能比较
- 简单的构建和运行系统

## 安装要求

- Rust (最新稳定版)
- Python 3.x
- Go 1.x
- Make

## 快速开始

1. 克隆仓库：

```bash
git clone https://github.com/yourusername/code-trio.git
cd code-trio
```

2. 构建项目：

```bash
make build
```

3. 运行测试：

```bash
make test
```

4. 运行所有实现：

```bash
make run
```

## 项目说明

### Rust 实现

位于 `packages/rust` 目录，使用 Rust 的所有权系统和生命周期特性实现链表。

### Python 实现

位于 `packages/python` 目录，使用 Python 的面向对象特性实现链表。

### Go 实现

位于 `packages/go` 目录，使用 Go 的接口和指针特性实现链表。

## 贡献指南

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件

## 联系方式

如有任何问题或建议，请通过以下方式联系：

- 提交 Issue
- 发送 Pull Request
