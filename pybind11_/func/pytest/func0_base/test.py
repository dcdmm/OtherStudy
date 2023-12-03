import func0_base
import func0_base_py

print(func0_base.add.__doc__)
print(func0_base_py.add.__doc__)
print('*' * 100)

print(func0_base.sub.__doc__)
print(func0_base_py.sub.__doc__)
print('*' * 100)

print(func0_base.multi.__doc__)
print(func0_base_py.multi.__doc__)
print('*' * 100)

print(func0_base.div.__doc__)
print(func0_base_py.div.__doc__)
print('*' * 100)

print(func0_base.add(1, 2))
print(func0_base_py.add(1, 2))
print('*' * 100)

print(func0_base.sub(i=4, j=10))  # 使用关键字参数
print(func0_base_py.sub(i=4, j=10))
print('*' * 100)

print(func0_base.multi())  # 使用默认参数
print(func0_base_py.multi())
print(func0_base.multi(i=4))
print(func0_base_py.multi(i=4))
print('*' * 100)

print(func0_base.div(1, 3))
print(func0_base_py.div(1, 3))
