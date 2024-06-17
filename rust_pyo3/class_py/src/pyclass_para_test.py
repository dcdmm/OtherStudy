from class_py import People, Study

print(issubclass(Study, People))  # print->True

p = People(20, False)
s = Study(21, True, "dcdmm")

p.print_age()  #// print->People age: 20
p.print_is_man()  #// print->People is_man: false
print(p.age, p.is_man)  # print->20 False

# 父类中的print_age方法
s.print_age()  #// print->People age: 21
# 子类的print_is_man方法(重写父类)
s.print_is_man()  #// print->Study is_man: true
# 子类的print_name方法(新的方法)
s.print_name()  #// print->Study age: dcdmm
print(s.age, s.is_man, s.name)  # print->21 True dcdmm