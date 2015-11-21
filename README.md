rs-auto-sync
===

### Feature
- 监控本地文件，自动同步修改到远程目录
- 使用 `rsync` 执行同步且只有修改的文件将进行同步
- 不同步隐藏文件 (比如 .git)

### Usage

```bash
$ brew install rsync

$ cargo build
$ cargo run local_path remote_path
```
