import type_conversions as tc

vec_wrapper = tc.VecWrapper()
vec_wrapper.append(10)
vec_wrapper.append(20)
vec_wrapper.append(30)

print(vec_wrapper.get(0))
print(vec_wrapper.get(1))
print(vec_wrapper.get(2))

vec_wrapper.pop()
# print(vec_wrapper.get(2))  # IndexError: Index out of bounds

print(vec_wrapper.vec)  # print->[10, 20]
vec_wrapper.vec = [-1, -2, -3]
print(vec_wrapper.get(0))  # print->-1
