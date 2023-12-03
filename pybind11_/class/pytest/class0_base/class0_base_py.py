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

    @staticmethod
    def howAge():
        return Pet.age
