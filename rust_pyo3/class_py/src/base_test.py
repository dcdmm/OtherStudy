import class_py

n = class_py.Number(0)
print(n.inner)  # print->0;#[pyo3(get)] inner实现
n.inner = 1
print(n.inner)  # print->1;#[pyo3(set)] inner实现

# ValueError: cannot be zero
# nz0 = class_py.Nonzero(0)

nz1 = class_py.Nonzero(1)
print(nz1.inner)  # print->1;#[getter] fn get_inner实现
nz1.inner = 2  
print(nz1.inner)  # print->0;#[setter] fn set_inner实现
# 实例方法
print(nz1.inner_add(100))  # print->102;fn inner_add实现
# 类方法(类对象调用)
print(class_py.Nonzero.cls_method())  # print->10;#[classmethod] fn cls_method实现
# 类方法(实例对象调用)
print(nz1.cls_method())  # pirnt->10
# 静态方法(类对象调用)
print(class_py.Nonzero.static_method(1, "rust"))  # print->100;#[staticmethod] fn static_method实现
# 静态方法(实例对象调用)
print(nz1.static_method(1, "c++"))  # print->100
# 类属性(类对象调用)
print(class_py.Nonzero.my_attribute)  # print->hello
# 类属性(实例方法调用)
print(nz1.my_attribute)  # print->hello 
class_py.Nonzero.my_attribute = "hello world"
print(class_py.Nonzero.my_attribute)  # print->hello world
print(nz1.my_attribute)  # print->hello world
print(nz1.MY_CONST_ATTRIBUTE) # print->foobar
class_py.Nonzero.MY_CONST_ATTRIBUTE = "Fa" 
print(nz1.MY_CONST_ATTRIBUTE)  # print->Fa