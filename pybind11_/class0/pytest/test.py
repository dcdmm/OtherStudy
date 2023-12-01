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
