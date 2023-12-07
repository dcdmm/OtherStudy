from making_opaque_type import StringList, print_opaque_list, ClassWithSTLVecProperty

lst = StringList()
print(type(lst))
print(lst.__len__())
lst.push_back("Element 1")
lst.push_back("Element 2")
print(print_opaque_list(lst))
lst.pop_back()
print(print_opaque_list(lst))

cvp = ClassWithSTLVecProperty()
print(print_opaque_list(cvp.stringList))
cvp.stringList = lst
print(print_opaque_list(cvp.stringList))
cvp.stringList.push_back("Element 3")
print(print_opaque_list(cvp.stringList))
print(type(cvp.stringList))
print(cvp.stringList)