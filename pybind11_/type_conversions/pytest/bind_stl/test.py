from bind_stl import VectorInt

v_int = VectorInt([1, 2])
print(type(v_int))
# 默认python list __len__方法
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
for i in v_int:
    print(i)
# 模仿python list count方法
print(v_int.count(-1))  # print->2
# 模仿python list remove方法
v_int.remove(-1)
print(v_int)  # print->VectorInt[-2, -3, -1]


v_int.modify_and_pop_back(10)
print(v_int)