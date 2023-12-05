import py_type

lst = [1, 2, 3, 4]
py_type.py_list(lst)
print(lst, end='\n\n')  # print->[-1111, 999, 2, 3, 4, -1]

dit = {"a": 1, "b": 2, "c": 3, "d": 4}
py_type.py_dict(dit)
print(dit, end='\n\n')  # print->{'a': 999, 'b': 2, 'c': 3, 'd': 4, 'e': -1}

set0 = {"a", "b", "c", "d"}
py_type.py_set(set0)
print(set0)  # print->{'c', 'e', 'a', 'd', 'b'}
