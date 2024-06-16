import class_py 

mc = class_py.MC(0)
print(mc.my_field)  # print->0

class_py.rust_struct(mc) #// print->rust_struct: 1
print(mc.my_field)  # print->1

class_py.pyref_(mc) #// print->pyref_: 1

class_py.pyrefmut_(mc)  #// print->pyrefmut_: 2
print(mc.my_field)  # print->2

class_py.print_refcnt(mc)  #// print->print_refcnt: 3


mc1 = class_py.MC1(0)
class_py.dissamble_clone(mc1)  #// print->dissamble_clone: 1