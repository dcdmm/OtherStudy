import time

import base
from base import child  # 导入子模块

print(base.__doc__)  # print->Rust函数文档注释

print(base.triple(100))  # print->300

print(child.double(100))  # print->200


def sum_py(n):
    result = 0
    for _ in range(n):
        for _ in range(n):
            result += 1
    return result


start_time = time.time()
r = sum_py(50000)
end_time = time.time()

elapsed_time = end_time - start_time
# n=10000 ===>2.9454572200775146
# n=20000 ===>11.730472803115845
# n=50000 ===>76.07530212402344
print(f"python函数执行耗时：{elapsed_time}秒")

start_time_rust = time.time()
r_rust = base.sum_rust(50000)
print(r_rust)
end_time_rust = time.time()

# n=10000 ===>0.6951656341552734
# n=20000 ===>2.696211338043213
# n=50000 ===>17.062873601913452
elapsed_time_rust = end_time_rust - start_time_rust
print(f"rust函数执行耗时：{elapsed_time_rust}秒")
