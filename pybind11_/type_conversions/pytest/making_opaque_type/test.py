import making_opaque_type

lst = making_opaque_type.StringList()
lst.push_back("Element 1")
lst.push_back("Element 2")

print(making_opaque_type.print_opaque_list(lst))