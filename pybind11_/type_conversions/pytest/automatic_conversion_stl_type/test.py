import automatic_conversion_stl_type

lst = [1, 2, 3, 4, 5]

automatic_conversion_stl_type.print_vector(lst)
print(lst, end='\n\n')  # print->[1, 2, 3, 4, 5];lst不变
automatic_conversion_stl_type.print_deque(lst)
print(lst, end='\n\n')  # lst不变
automatic_conversion_stl_type.print_list(lst)
print(lst, end='\n\n')  # lst不变
automatic_conversion_stl_type.print_array(lst)
print(lst, end='\n\n')  # lst不变
automatic_conversion_stl_type.print_valarray(lst)
print(lst, end='\n\n')  # lst不变

dit = {"a": 1, "b": 2, "c": 3}
automatic_conversion_stl_type.print_map(dit)
print(dit, end='\n\n')  # dit不变

set0 = {1, 2, 3, 4}
automatic_conversion_stl_type.print_set(set0)
print(set0)  # set0不变