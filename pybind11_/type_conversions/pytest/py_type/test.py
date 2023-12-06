import py_type

lst = [1, 2, 3, 4]
new_lst = py_type.py_list(lst)
print(new_lst, type(new_lst)) # print->[1, 2, 3, 4, 100] <class 'list'>
print(lst, end='\n\n')  # print->[-1111, 999, 2, 3, 4, -1]

dit = {"a": 1, "b": 2, "c": 3, "d": 4}
new_dit = py_type.py_dict(dit)
print(new_dit, type(new_dit))
print(dit, end='\n\n')  # print->{'a': 999, 'b': 2, 'c': 3, 'd': 4, 'e': -1}

set0 = {"a", "b", "c", "d"}
new_set = py_type.py_set(set0)
print(new_set, type(new_set))
print(set0, end='\n\n')  # print->{'c', 'e', 'a', 'd', 'b'}

tup = (1, 2, 3, 4)
py_type.py_tuple(tup)