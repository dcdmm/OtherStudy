import type_conversions as tc

s0 = "hello world"
tc.str_test0(s0)
print(s0, end='\n\n')  # print->hello world

tc.str_test1(s0)
print(end='\n\n')

l0 = [0, 1, 2]
tc.list_test0(l0)
print(l0, end='\n\n')  # print->[0, 1, 2]

tc.list_test1(l0)
print(l0, end='\n\n')  # print->[0, 1, 2, 'one']

d0 = {"a": 0, "b": 1, "c": 2}
tc.dict_test0(d0)
print(d0, end='\n\n')  # print->{'a': 0, 'b': 1, 'c': 2}

tc.dict_test1(d0)
print(d0, end='\n\n')  # print->{'a': 0, 'b': 1, 'c': 2, 'd': 4}