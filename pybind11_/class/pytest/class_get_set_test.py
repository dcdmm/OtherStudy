import class_get_set
import class_get_set_py

p = class_get_set.People("dcdmm")
print(p.name)  # 装饰器@property
p.name = "dcgo"  # 装饰器@name.setter
print(p.name)

p_py = class_get_set_py.People("dcdmm_py")
print(p_py.name)
p_py.name = "dcgo_py"
print(p_py.name)