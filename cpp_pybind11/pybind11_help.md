## 安装

* 安装pybind11
    * [具体见官方文档](https://pybind11.readthedocs.io/en/latest/installing.html)

* 其他安装(Ubuntu)
    * apt install cmake
    * apt install python3-dev

## 编译

* step 1
    ```markdown
    进入python base/虚拟环境(不同环境的cpython版本不同)
    
    如:
        * base环境python版本为3.11.12,base环境下编译的动态库文件为base.cpython-311-x86_64-linux-gnu.so
        * job虚拟环境python版本为3.10.12,job环境下编译的动态库文件为base.cpython-310-x86_64-linux-gnu.so
    ```
    
* step 2
    ```shell
    # /root/work/vcpkg:即vcpkg在Linux上下载路径
    cmake .. -DCMAKE_TOOLCHAIN_FILE=/root/work/vcpkg/scripts/buildsystems/vcpkg.cmake 
    ```