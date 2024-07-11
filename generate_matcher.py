import itertools

def generate_case(params, return_ty):
    param_match = ", ".join(["ValType::%s" % x.upper() for x in params])
    param_ty = ", ".join(params)
    if return_ty == None:
        return_ty = ""
        return_match = "None"
    else:
        return_match = "Some(ValType::" + return_ty.upper() + ")"
        return_ty = " -> " + return_ty
    param_list = ", ".join(["p" + str(x) for x in range(len(params))])
    return """([%s], %s) => {
    let func: Symbol<unsafe extern "C" fn(%s)%s> =
        lib.get(import.name.as_bytes()).unwrap();
    linker
        .func_wrap(import.module, import.name, move |%s| func(%s))
        .unwrap();
}\n""" % (param_match, return_match, param_ty, return_ty, param_list, param_list)


types = ["i32", "f32", "i64", "f64"]
return_types = [ty for ty in types]
return_types.append(None)

body = ""
for arg_count in range(5):
    possible_args = itertools.product(types, repeat = arg_count)
    for args in possible_args:
        for return_ty in return_types:
            body += generate_case(args, return_ty)
for int_only_args in range(5, 17):
    body += generate_case(["i32"] * int_only_args, None)
    body += generate_case(["i32"] * int_only_args, "i32")
print(body)
