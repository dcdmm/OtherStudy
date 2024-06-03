## 安装

1. 进入python base/虚拟环境
2. pip install maturin
3. apt install python3-dev

## 开发

假定该项目名为:rust_to_python

1. 进入python base/虚拟环境
2. mkdir rust_to_python
3. cd rust_to_python
4. maturin init
5. vscode中开发
   1. 进入python base/虚拟环境
   2. 使用命令code打开vscode
   3. vscode中打开rust_to_python项目

## 编译与安装
1. 进入python base/虚拟环境
2. cd rust_to_python
3. maturin build -r -o .
    * 参数-o/--out: The directory to store the built wheels in. Defaults to a new "wheels" directory in the project's target directory
    * 参数-r/--release: Build artifacts in release mode, with optimizations
4. 编译结果: rust_to_python-0.3.8-cp310-cp310-manylinux_2_34_x86_64.
   * rust_to_python: 包名(对应rust项目包名,见Cargo.toml [package] name)
   * 0.3.8: 版本号(对应rust项目版本号,见Cargo.toml [package] version)
   * cp310: 仅适用与cpython3.10(base/虚拟环境python版本为3.10.x)
5. 安装: pip install rust_to_python-0.3.8-cp310-cp310-manylinux_2_34_x86_64.