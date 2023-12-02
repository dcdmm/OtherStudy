class Pet:
    def __init__(self, name):
        self.name = name


class Dog(Pet):
    def __init__(self, name):
        super().__init__(name)

    def bark(self):
        return "woof_py!"