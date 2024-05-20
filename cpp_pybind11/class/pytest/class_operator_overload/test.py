import class_operator_overload

v0 = class_operator_overload.Vector2(1, 2)
v1 = class_operator_overload.Vector2(10, 100)

v2 = v0 + v1  # 运算符重载
print(v2)

v3 = 2.0 * v0
print(v3)