import func_args_kwargs


args = 1, "dcdmmcomeon", 3.9
af = func_args_kwargs.args_function(*args)  # 解包
print(af, end='\n\n')

af1 = func_args_kwargs.args_function(2, "xiaoming", 10.0)
print(af1, end='\n\n')

dit = {"a": 1, "b": 2}
kf = func_args_kwargs.kwargs_function(**dit)  # 解包
print(kf, end='\n\n')

kf1 = func_args_kwargs.kwargs_function(a=1, b=2, c=3)
print(kf1, end='\n\n')

func_args_kwargs.mixed_plus_args(1, 3.14)

func_args_kwargs.mixed_plus_args(1, 3.14, None, "hello java", 1000)

func_args_kwargs.mixed_plus_kwargs(1, 2.4)

func_args_kwargs.mixed_plus_kwargs(1, 2.4, a="hello c++", b=10)

func_args_kwargs.mixed_plus_args_kwargs(1, 2.3)

func_args_kwargs.mixed_plus_args_kwargs(1, 2.3, [1, 2], "hello rust")

func_args_kwargs.mixed_plus_args_kwargs(1, 2.3, [1, 2], "hello rust", name="dcdmm", age="28")