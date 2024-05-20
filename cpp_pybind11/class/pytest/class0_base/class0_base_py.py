class Pet:
    age = 100  # 类属性

    def __init__(self, name):
        self.name = name

    def setName(self, name_):
        self.name = name_

    def getName(self):
        return self.name

    def __repr__(self):
        return "<example.Pet named '" + self.name + "'>"

    def is_cpp(self, is_cpp): # 第一个参数为self,self表示实例本身
        if self.name == is_cpp:
            return True
        else:
            return False

    @staticmethod
    def howAge():
        return Pet.age
