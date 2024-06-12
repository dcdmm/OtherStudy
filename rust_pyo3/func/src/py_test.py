import func

print(func.double(100))  # print->200

print(func.no_args())  # print->42

print(func.default_para(1, 1))  # print->2
print(func.default_para(1))  # print->0

func.args_para(0)
func.args_para(0, 1, 2, 3, 4)

func.kwargs_prara(0)
func.kwargs_prara(0, a=1, b=2)

obj = func.MyStruct(0)
print(obj.add.__text_signature__)  # print->($self, x, z, y=...)
print(obj.add(1, "hello world"))  # print->4
print(obj.sub.__text_signature__)  # print->(self, x, sentence, y=[1, 2])
print(obj.sub(1, "hello world"))  # print->-4
print(obj.mul.__text_signature__)  # print->None
print(obj.mul(1, "hello world"))  # print->0