import other

result = other.Python_test1(10000)
print(result)  # print->100.0

try:
    other.throws_test0()
except TypeError as e:
    print(e)  # print->Error message

try:
    other.throws_test1([0])
except IndexError as e:
    print(e)  # print->Index out of bounds