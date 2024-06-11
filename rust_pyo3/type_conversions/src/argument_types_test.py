import type_conversions as tc

tc.int_test0(1)  # print->1
tc.int_test1(1)  # print->1
tc.int_test2(1)  # print->1
print(end='\n\n')

s0 = "hello world"
tc.str_test0(s0)
print(s0, end='\n\n')  # print->hello world
tc.str_test1(s0)
print(end='\n\n')

l0 = [0, 1, 2]
tc.list_test0(l0)
print(l0, end='\n\n')  # print->[0, 1, 2];l0不变
tc.list_test1(l0)
print(l0, end='\n\n')  # print->[0, 1, 2, 'one'];l0改变

d0 = {"a": 0, "b": 1, "c": 2}
tc.dict_test0(d0)
print(d0, end='\n\n')  # print->{'a': 0, 'b': 1, 'c': 2};d0不变
tc.dict_test1(d0)
print(d0, end='\n\n')  # print->{'a': 0, 'b': 1, 'c': 2, 'd': 4};d0改变

set0 = {0, 1, 2}
tc.set_test0(set0)
print(set0, end='\n\n')  # print->{0, 1, 2};set0不变
tc.set_test1(set0)
print(set0, end='\n\n')  # print->{0, 1, 2, 'three'};set0改变