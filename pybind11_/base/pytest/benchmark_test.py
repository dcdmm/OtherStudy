import time
import benchmark


def sum_py(n):
    result = 0
    for i in range(n):
        for j in range(n):
            result += 1
    return result


start_time = time.time()
r = sum_py(50000)
end_time = time.time()

elapsed_time = end_time - start_time
# n=10000 ===>1.831888198852539
# n=20000 ===>7.099264621734619
# n=50000 ===>48.87408089637756
print(f"python函数执行耗时：{elapsed_time}秒")


start_time_cpp = time.time()
r_cpp = benchmark.sum_cpp(50000)
end_time_cpp = time.time()

# n=10000 ===>0.05034065246582031
# n=20000 ===>0.1952359676361084
# n=50000 ===>1.2279067039489746
elapsed_time_cpp = end_time_cpp - start_time_cpp
print(f"cpp函数执行耗时：{elapsed_time_cpp}秒")