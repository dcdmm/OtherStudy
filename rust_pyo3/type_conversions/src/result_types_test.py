import type_conversions as tc

s0 = tc.str_t0()
print(type(s0))  # print-><class 'str'>
s1 = tc.str_t1()
print(type(s1))  # print-><class 'str'>

o0 = tc.Option_t0()
print(o0)  # print->5

l0 = tc.list_t0()
print(l0)  # print->[1, 2, 3, 4]

T0 = tc.T_t0()
print(T0, type(T0))  # print->{} <class 'dict'>