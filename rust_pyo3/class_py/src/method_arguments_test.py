import class_py

mc = class_py.MyClass()
print(mc.num)  # print->-1
# print->num=100 (was previously=-1), py_args=(1, 2, 3), name=hello rust, py_kwargs=Some({'a': -1, 'b': -2, 'c': -3}) 
print(mc.method(100, "hello rust", 1, 2, 3, a=-1, b=-2, c=-3))
print(mc.my_class_method.__text_signature__)  # print->(cls, e_int, f_int)
print(class_py.MyClass.my_static_method.__text_signature__)  # print->(e_int, f_int)