import class_overload

# print(help(class_overload.Pet))

c = class_overload.Pet("dc", 28)
print(c.name)
print(c.age)
# 原生python不支持函数重载
c.set("dmm") # call void set(const std::string &name_)
c.set(1) # call void set(int age_)
print(c.name)
print(c.age)
