import class_py

n = class_py.Number(0)
print(n.inner)  # print->0;#[pyo3(get)]属性实现
n.inner = 1
print(n.inner)  # print->1;#[pyo3(set)]属性实现

# ValueError: cannot be zero
# nz0 = class_py.Nonzero(0)

nz1 = class_py.Nonzero(1)
print(nz1.inner)  # print->1;#[getter]属性实现
nz1.inner = 2  
print(nz1.inner)  # print->0;#[setter]属性实现