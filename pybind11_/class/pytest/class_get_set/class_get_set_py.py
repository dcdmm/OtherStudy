class People:
    def __init__(self, name):
        self._name = name  # 使用下划线前缀表示私有属性

    @property
    def name(self):  # 获取属性
        return self._name

    @name.setter
    def name(self, value):  # 修改属性
        self._name = value