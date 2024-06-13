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
# 类方法
print(class_py.Nonzero.cls_method())  # print->10;#[classmethod] fn实现 cls_method