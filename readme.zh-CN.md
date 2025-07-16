# Forever Key Generator

你可以选择阅读不同语言版本的 README：
- [简体中文](README.zh-CN.md)
- [繁体中文](readme.zh-HK.md)
- [English](README.en.md)

`forever-keygen` 是一个用于从6位代码生成加密密钥的工具。它支持交互式和命令行两种使用模式，并提供了灵活的配置选项，如自定义盐值、PBKDF2迭代次数和输出密钥长度。

## 功能特性
- **交互式和命令行模式**：用户可以选择交互式输入或使用命令行参数来生成加密密钥。
- **自定义盐值**：支持使用随机盐值或用户提供的盐值进行密钥生成。
- **PBKDF2迭代次数和密钥长度配置**：允许用户自定义PBKDF2迭代次数和输出密钥的长度。

## 安装
确保你已经安装了Rust环境，然后可以通过以下步骤进行安装：
1. 克隆仓库：
```bash
git clone https://github.com/your-repo/Forever-Keygen.git
cd Forever-Keygen
```
2. 构建项目：
```bash
cargo build --release
```
3. 安装可执行文件：
```bash
cargo install --path .
```

## 使用方法

1. 正确安装forever-keygen（或者不安装也行，直接下载使用release也可以，随你的便）

2. 如果用于网盘传文件，对于文件安全性有较高要求则可以先和需要接收文件的人协商好盐值，然后再使用这个工具

3. 工具有两种使用方式：交互式模式、命令行模式

### 交互式模式
在不提供命令行参数时，工具将进入交互式模式，您可以输入6位代码和盐值选项：
```bash
forever-keygen
```
按照提示输入6位代码，选择是否使用随机盐值，并根据需要输入自定义盐值。工具将显示生成的盐值和加密密钥。

### 命令行模式
使用命令行参数可以直接指定6位代码、盐值、PBKDF2迭代次数和输出密钥长度：
```bash
forever-keygen -c 123456 -s mysalt -i 200000 -k 256
```
- `-c, --code`：6位代码，用于密钥生成。
- `-s, --salt`：盐字符串，用于密钥生成。
- `-i, --iterations`：PBKDF2迭代次数，默认为100000。
- `-k, --key-length`：输出密钥长度（字节），默认为128。

> *注意：在交互式下，将默认指定迭代次数100000和输出密钥长度128*

## 示例

输入：

```bash
forever-keygen -c 123456 -s this_is_a_test_salt_yeahyeahyeah
```

输出示例：

```
Salt: Sz9DjTMNqOLiqA0zjUM/Sw==
Encryption key: l1FLUUUJWkr4ner3wuzbdEi4xXGHowosJNqyr1cyEmOTZLIaw+f+GcAKyCu+cpU/LyHrefKGZs4MmW5YHf5JYOtmvJlOa6o5NxoU315x8s/859VxMjciBWxDWf9PipwCLwzi1N7/g532raAHaDkWA0Ap4e1cHnDFditr2Sh1+Ww=
```

- 文件收发双方（或者需要解压的相关人）只需要记住 <u>最开始设定的</u> **盐值** 即可（例如示例中的 `this_is_a_test_salt_yeahyeahyeah`）

- *输出结果中的 `Salt` 只作为参考*

- 然后你就可以把`Encryption key`作为压缩文件的加密密码啦，推荐使用带有AES加密的7z、rar、lz4等，或者zip with AES，而不要使用原生zip的加密（不安全）

- 密钥6位数字可以写在 **文件名** 上，所以请不要泄露盐值啦~


还有

## *永远不要使用弱密码*