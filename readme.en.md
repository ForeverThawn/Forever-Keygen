# Forever Key Generator

README Language：
- [English](README.en.md)
- [简体中文](README.zh-CN.md)
- [繁体中文](readme.zh-HK.md)

`forever-keygen` is a tool designed to generate encryption keys from 6-digit codes. It supports two usage modes: interactive and command-line, and offers flexible configuration options such as custom salt values, PBKDF2 iteration counts, and output key lengths.

## Features
- **Interactive and Command-Line Modes**: Users can choose either interactive input or command-line arguments to generate encryption keys.
- **Customizable Salt**: Supports the use of either random salts or user-provided salts for key generation.
- **Configurable PBKDF2 Iterations and Key Length**: Allows users to customize the number of PBKDF2 iterations and the length of the output key.

## Installation
Make sure you have the Rust environment installed. Then, you can install the tool through the following steps:
1. Clone the repository:
```bash
git clone https://github.com/your-repo/Forever-Keygen.git
cd Forever-Keygen
```
2. Build the project:
```bash
cargo build --release
```
3. Install the executable:
```bash
cargo install --path .
```

## Usage

1. Install `forever-keygen` correctly (alternatively, you can directly download and use the release version; it's up to you).

2. If you're using this tool for file transfer on a cloud storage service and have high requirements for file security, you should first discuss and agree on a salt value with the person who will receive the file, and then use this tool.

3. The tool has two usage methods: interactive mode and command-line mode.

### Interactive Mode
When no command-line arguments are provided, the tool enters interactive mode. You can input the 6-digit code and salt options:
```bash
forever-keygen
```
Follow the prompts to enter the 6-digit code, choose whether to use a random salt, and input a custom salt if necessary. The tool will display the generated salt and encryption key.

### Command-Line Mode
You can directly specify the 6-digit code, salt, PBKDF2 iteration count, and output key length using command-line arguments:
```bash
forever-keygen -c 123456 -s mysalt -i 200000 -k 256
```
- `-c, --code`: The 6-digit code used for key generation.
- `-s, --salt`: The salt string used for key generation.
- `-i, --iterations`: The number of PBKDF2 iterations, with a default value of 100,000.
- `-k, --key-length`: The length of the output key in bytes, with a default value of 128.

> *Note: In interactive mode, the number of iterations is set to 100,000 and the output key length is set to 128 bytes by default.*

## Examples

Input:

```bash
forever-keygen -c 123456 -s this_is_a_test_salt_yeahyeahyeah
```

Sample output:

```
Salt: Sz9DjTMNqOLiqA0zjUM/Sw==
Encryption key: l1FLUUUJWkr4ner3wuzbdEi4xXGHowosJNqyr1cyEmOTZLIaw+f+GcAKyCu+cpU/LyHrefKGZs4MmW5YHf5JYOtmvJlOa6o5NxoU315x8s/859VxMjciBWxDWf9PipwCLwzi1N7/g532raAHaDkWA0Ap4e1cHnDFditr2Sh1+Ww=
```

- Both the sender and the receiver of the file (or relevant people who need to unzip the file) only need to remember the **salt value** that was initially set (e.g., `this_is_a_test_salt_yeahyeahyeah` in the example).

- *The `Salt` in the output is for reference only.*

- Then you can use the `Encryption key` as the encryption password for the compressed file. It is recommended to use compression formats with AES encryption such as 7z, rar, lz4, or zip with AES, rather than the native encryption of zip (which is not secure).

- You can write the 6-digit key on the **file name**. So, please don't disclose the salt value!


Also

## *NEVER USE WEAK PASSWORDS.*