from class_has_a_combination import People, Pet

dog = Pet("wangcai", 3)
dog.show()

p0 = People("xiaoming", dog)
print(p0.getPetName())
print(p0.getPetAge())

p1 = People("xiaohei", Pet("xiaohei", 5))
p1.p.show()
