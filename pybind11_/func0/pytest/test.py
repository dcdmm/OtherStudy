import func0
import func0_py

print(func0.__doc__)
print(func0_py.__doc__)
print('*' * 100)

print(func0.add.__doc__)
print(func0_py.add.__doc__)
print('*' * 100)

print(func0.sub.__doc__)
print(func0_py.sub.__doc__)
print('*' * 100)

print(func0.multi.__doc__)
print(func0_py.multi.__doc__)
print('*' * 100)

print(func0.add(1, 2))
print(func0_py.add(1, 2))
print('*' * 100)

print(func0.sub(i=4, j=10))  # 使用关键字参数
print(func0_py.sub(i=4, j=10))
print('*' * 100)

print(func0.multi())  # 使用默认参数
print(func0_py.multi())
print(func0.multi(i=4))
print(func0_py.multi(i=4))
