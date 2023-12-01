import class0
import class0_py

c = class0.Pet("hello c++")
c_py = class0_py.Pet("hello python")

print(c)  # __repr__方法
print(c_py)

print(c.getName())
print(c_py.getName())

c.setName("hello c++!")
c_py.setName("hello python!")

print(c.getName())
print(c_py.getName())

print(c.age)  # 类属性
print(class0.Pet.age)
print(c_py.age)
print(class0_py.Pet.age)

print(c.howAge())  # 静态方法
print(class0.Pet.howAge())
print(c.howAge())
print(class0_py.Pet.howAge())

c.name = "hello c++"
print(c.name)  # 实例属性
c_py.name = "hello python!!!"
print(c_py.name)

print('*' * 100)

# c.new_name = "333333"  # 报错:AttributeError: 'class0.Pet' object has no attribute 'new_name'
c_py.new_name = "333333"  # 动态绑定属性(参考:MLNote\A_PythonBasis\面向对象编程\类-类对象-实例对象.ipynb)
print(c_py.new_name)

c_dyn = class0.Pet_dyn("111111")
c_dyn.name = "222222"
print(c_dyn.name)
c_dyn.new_name = "333333"
print(c_dyn.new_name)

print('*' * 100)

p = class0.People("dcdmm")
print(p.name)  # 装饰器@property
p.name = "dcgo"  # 装饰器@name.setter
print(p.name)

p_py = class0_py.People("dcdmm_py")
print(p_py.name)
p_py.name = "dcgo_py"
print(p_py.name)
