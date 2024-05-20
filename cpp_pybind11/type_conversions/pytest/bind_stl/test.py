from bind_stl import VectorInt, MapStringDouble

'''
模仿python list的方法:
append
clear
extend
insert
pop
count
remove
__len__
__iter_
__getitem__
__setitem__
__contains_
'''

v_int = VectorInt([1, 2])
print(type(v_int))
# 模仿python list __len__方法
print(len(v_int))  # print->2
# 模仿python list append方法
v_int.append(3)
print(v_int)  # print->VectorInt[1, 2, 3]
# 模仿python list clear方法
v_int.clear()
print(v_int)  # print->VectorInt[]
# 模仿python list extend方法
v_int.extend([4, 5])
print(v_int)  # print->VectorInt[4, 5]
# 模仿python list insert(self, *args, **kwargs)方法
v_int.insert(1, 999)
print(v_int)  # print->VectorInt[4, 999, 5]
# 模仿python list pop(self, *args, **kwargs方法
v_int.pop(1)
print(v_int)  # print->VectorInt[4, 5]
v_int.pop()
print(v_int)  # print->VectorInt[4]
# 模仿python list __getitem__方法
print(v_int[0])  # print->4
# 模仿python list __setitem__方法
v_int[0] = -1
print(v_int)  # print->VectorInt[-1]
v_int.extend([-2, -3, -1])
# 模仿python list __iter__方法
for i in v_int:
    print(i)
# 模仿python list count方法
print(v_int.count(-1))  # print->2
# 模仿python list remove方法
v_int.remove(-1)
print(v_int)  # print->VectorInt[-2, -3, -1]
# 模仿python list __contains_方法
print(-1 in v_int) # print->True

# 自定义方绑定的法
v_int.pop_back()
print(v_int)
v_int.multiply_elements(100)
print(v_int)

print('*' * 100)

'''
模仿python dict的方法:
keys
values
items
__len__
__iter_
__getitem__
__setitem__
__contains_
'''
mm = MapStringDouble()
mm["a"] = 1
mm["b"] = 2.5
print(mm)
# 模仿python dict __len__方法
print(len(mm))
# 模仿python dict __setitem__方法
mm["a"] = 10
# 模仿python dict __iter__方法
for i in mm:
    # 模仿python dict __getitem__方法
    print(mm[i])
# 模仿python dict __contains_方法
print('a' in mm)
# 模仿python dict keys方法
print(mm.keys())
print(list(mm.keys()))  # print->['a', 'b']
# 模仿python dict values方法
print(list(mm.values()), type(list(mm.values())))  # print->[10.0, 2.5] <class 'list'>
# 模仿python dict items方法
print(mm.items())
print(list(mm.items()))  # print->('a', 10.0), ('b', 2.5)]
