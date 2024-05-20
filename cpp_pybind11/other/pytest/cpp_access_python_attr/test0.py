import cpp_access_python_attr
from cpp_access_python_f_py import Point

p1 = Point(1, 2)
p2 = Point(3, 4)

count = p1.count
x = p1.put_x()
y = p1.put_y(100)
count_s = p1.put_count()
print(count, '\t', x, '\t', y, '\t', count_s, end='\n\n')

cpp_access_python_c.process_custom_class(p1)
