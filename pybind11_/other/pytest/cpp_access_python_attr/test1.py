import cpp_access_python_attr

lst0 = [1, 2, 3]
lst1 = ["a", "b"]

cpp_access_python_c.process_list(lst0, lst1)
print("after", lst0)
print("after", lst1)