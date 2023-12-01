import class0
import class0_py

c = class0.Pet("hello c++")
c_py = class0_py.Pet("hello python")

print(c)  # print-><example.Pet named 'hello c++'>
print(c_py)  # print-><example.Pet named 'hello python'>

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

print(c.howAge())
print(class0.Pet.howAge())
print(c.howAge())
print(class0_py.Pet.howAge())

print(c.name)  # 实例属性
print(c_py.name)

print('*' * 100)
p = class0.People("dcdmm")
print(p.name)
p.name = "dcgo"
print(p.name)

p_py = class0_py.People("dcdmm_py")
print(p_py.name)
p_py.name = "dcgo_py"
print(p_py.name)