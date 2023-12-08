import class_custom_init
import class_custom_init_py

e = class_custom_init.Example(18, 1, 0, "daxue")
e.show()

e0 = class_custom_init.Example(3.4)
e0.show()

e1 = class_custom_init.Example(4, 5)
e1.show()

e2 = class_custom_init.Example("DCDMM")
e2.show()

print('*' * 100)

e_py = class_custom_init_py.Example(18, 1, 0, "daxue")
e_py.show()

e0_py = class_custom_init_py.Example.from_double(3.4)
e0_py.show()

e1_py = class_custom_init_py.Example.from_int(4, 5)
e1_py.show()

e2_py = class_custom_init_py.Example.from_str("DCDMM")
e2_py.show()
