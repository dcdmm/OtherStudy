import func_template_overload

print(func_template_overload.add_long(1, 34))
print(func_template_overload.add_double(1, 2.1))

# 原生python不支持函数重载
print(func_template_overload.sub(1, 3))
print(func_template_overload.sub(1, -3.1))