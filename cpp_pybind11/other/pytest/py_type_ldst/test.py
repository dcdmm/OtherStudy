import py_type_ldst

lst = [1, 2, 3, 4]
py_type_ldst.py_list(lst)
print(lst, end='\n\n')  # print->[-1111, 999, 2, 3, 4, -1]

dit = {"a": 1, "b": 2, "c": 3, "d": 4}
py_type_ldst.py_dict(dit)
print(dit, end='\n\n')  # print->{'a': 999, 'b': 2, 'c': 3, 'd': 4, 'e': -1}

set0 = {"a", "b", "c", "d"}
py_type_ldst.py_set(set0)
print(set0, end='\n\n')  # print->{'c', 'e', 'a', 'd', 'b'}

tup = (1, 2, 3, 4)
py_type_ldst.py_tuple(tup)