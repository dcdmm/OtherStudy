from collections.abc import Iterable, Iterator

import class_py 

it = class_py.SimpleIterator(5)
print(isinstance(it, Iterator))  # 是迭代器
print(isinstance(it, Iterable))  # 是可迭代对

for value in it:
    print(value)