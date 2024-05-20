import cpp_access_python_f

def sum_list_per(lst, denominator):
    return sum(lst) / denominator

lst0 = [1, 2, 3, 4]
print(sum_list_per(lst0, 100))

slp = cpp_access_python_f.process_function(sum_list_per, lst0)