class Example:
    """python间接实现多构造函数(python不支持函数重载)"""

    def __init__(self, age=28, rank=1, salary=99999, name="dmm"):
        self.name = name
        self.salary = salary
        self.rank = rank
        self.age = age

    @classmethod
    def from_int(cls, i1, i2):
        instance = cls(age=i1, rank=i2)
        return instance

    @classmethod
    def from_double(cls, s):
        instance = cls(salary=s)
        return instance

    @classmethod
    def from_str(cls, n):
        instance = cls(name=n)
        return instance

    def show(self):
        print(self.age, end='\t')
        print(self.rank, end='\t')
        print(self.salary, end='\t')
        print(self.name)
