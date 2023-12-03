import class1_inherit
import class1_inherit_py

c = class1_inherit.Dog("Molly")
print(c.name)
print(c.bark())

c1 = class1_inherit.pet_store()
print(type(c1))  # print-><class 'class1_inherit.Pet'>
# no pointer downcasting for regular non-polymorphic types
# print(c1.bark())  # 报错:AttributeError: 'class1_inherit.Pet' object has no attribute 'bark'

c2 = class1_inherit.pet_store2()
print(type(c2)) # print-><class 'class1_inherit.PolymorphicDog'>
print(c2.bark())

print('*' * 100)

c_py = class1_inherit_py.Dog("Molly_py")
print(c_py.name)
print(c_py.bark())

# 动态语言(python)调用实例方法,不检查类型,只要方法存在,参数正确,就可以调用
# 鸭子类型(如果走起路来像鸭子,叫起来也像鸭子,那么它就是鸭子(If it walks like a duck and quacks like a duck, it must be a duck)
print(class1_inherit_py.pet_store(c_py))
