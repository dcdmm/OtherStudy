import func

print(func.double(100))  # print->200

print(func.no_args())  # print->42

print(func.default_para(1, 1))  # print->2
print(func.default_para(1))  # print->0

func.args_para(0)
func.args_para(0, 1, 2, 3, 4)

func.kwargs_prara(0)
func.kwargs_prara(0, a=1, b=2)