class Point:
    count = 0

    def __init__(self, x, y):  # 初始化函数
        self.x = x
        self.y = y
        Point.count += 1

    def put_x(self):
        print("x:", self.x)
        return self.x

    def put_y(self, denominator):
        print("y", self.y)
        print("denominator", denominator)
        return self.y / denominator

    @staticmethod
    def put_count():
        print("count", Point.count)
        return Point.count
