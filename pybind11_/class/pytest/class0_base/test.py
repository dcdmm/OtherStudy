import class0_base
import class0_base_py

c = class0_base.Pet("hello c++")
c_py = class0_base_py.Pet("hello python")

print(c)  # __repr__方法
print(c_py)

print("*" * 100)

print(c.getName())
print(c_py.getName())

c.setName("hello c++!")
c_py.setName("hello python!")

print(c.getName())
print(c_py.getName())

print("*" * 100)

print(c.age)  # 类属性
print(class0_base.Pet.age)
print(c_py.age)
print(class0_base_py.Pet.age)

print("*" * 100)

print(c.howAge())  # 静态方法
print(class0_base.Pet.howAge())
print(c.howAge())
print(class0_base_py.Pet.howAge())

print("*" * 100)

c.name = "hello c++!!!"
print(c.name)  # 实例属性
c_py.name = "hello python!!!"
print(c_py.name)

print('*' * 100)

# c.new_name = "333333"  # 报错:AttributeError: 'class0_base.Pet' object has no attribute 'new_name'
c_py.new_name = "333333"  # 动态绑定属性(参考:MLNote\A_PythonBasis\面向对象编程\类-类对象-实例对象.ipynb)
print(c_py.new_name)

c_dyn = class0_base.Pet_dyn("111111")
c_dyn.new_name = "333333"
print(c_dyn.new_name)