# Forever Key Generator

README Language：
- [English](readme.en.md)
- [繁体中文](readme.zh-HK.md)
- [简体中文](readme.zh-CN.md)

`forever-keygen` 是一個用於從6位代碼生成加密金鑰的工具。它支援互動式和命令列兩種使用模式，並提供了靈活的配置選項，如自定義鹽值、PBKDF2迭代次數和輸出金鑰長度。

## 功能特性
- **互動式和命令列模式**：使用者可以選擇互動式輸入或使用命令列參數來生成加密金鑰。
- **自定義鹽值**：支援使用隨機鹽值或使用者提供的鹽值進行金鑰生成。
- **PBKDF2迭代次數和金鑰長度配置**：允許使用者自定義PBKDF2迭代次數和輸出金鑰的長度。

## 安裝
確保你已經安裝了Rust環境，然後可以透過以下步驟進行安裝：
1. 克隆倉庫：
```bash
git clone https://github.com/your-repo/Forever-Keygen.git
cd Forever-Keygen
```
2. 構建項目：
```bash
cargo build --release
```
3. 安裝可執行文件：
```bash
cargo install --path .
```

## 使用方法

1. 正確安裝forever-keygen（或者不安裝也行，直接下載使用release版本也可以，隨你喜好）

2. 如果用於網盤傳文件，對於文件安全性有較高要求則可以先和需要接收文件的人協商好鹽值，然後再使用這個工具

3. 工具有兩種使用方式：互動式模式、命令列模式

### 互動式模式
在不提供命令列參數時，工具將進入互動式模式，您可以輸入6位代碼和鹽值選項：
```bash
forever-keygen
```
按照提示輸入6位代碼，選擇是否使用隨機鹽值，並根據需要輸入自定義鹽值。工具將顯示生成的鹽值和加密金鑰。

### 命令列模式
使用命令列參數可以直接指定6位代碼、鹽值、PBKDF2迭代次數和輸出金鑰長度：
```bash
forever-keygen -c 123456 -s mysalt -i 200000 -k 256
```
- `-c, --code`：6位代碼，用於金鑰生成。
- `-s, --salt`：鹽字串，用於金鑰生成。
- `-i, --iterations`：PBKDF2迭代次數，預設為100000。
- `-k, --key-length`：輸出金鑰長度（位元組），預設為128。

> *注意：在互動式模式下，將預設指定迭代次數100000和輸出金鑰長度128*

## 範例

輸入：

```bash
forever-keygen -c 123456 -s this_is_a_test_salt_yeahyeahyeah
```

輸出示例：

```
Salt: Sz9DjTMNqOLiqA0zjUM/Sw==
Encryption key: l1FLUUUJWkr4ner3wuzbdEi4xXGHowosJNqyr1cyEmOTZLIaw+f+GcAKyCu+cpU/LyHrefKGZs4MmW5YHf5JYOtmvJlOa6o5NxoU315x8s/859VxMjciBWxDWf9PipwCLwzi1N7/g532raAHaDkWA0Ap4e1cHnDFditr2Sh1+Ww=
```

- 文件收發雙方（或者需要解壓的相關人）只需要記住 <u>最開始設定的</u> **鹽值** 即可（例如範例中的 `this_is_a_test_salt_yeahyeahyeah`）

- *輸出結果中的 `Salt` 只作為參考*

- 然後你就可以把`Encryption key`作為壓縮文件的加密密碼啦，推薦使用帶有AES加密的7z、rar、lz4等，或者zip with AES，而不要使用原生zip的加密（不安全）

- 金鑰6位數字可以寫在 **文件名** 上，所以請不要洩露鹽值啦~


還有

## *永遠不要使用弱密碼*