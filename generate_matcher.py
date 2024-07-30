import itertools

def generate_case(params, return_ty):
    param_match = ", ".join(["t%d @ (ParamType::I32 | ParamType::F32 | ParamType::Pointer)" % (idx,) if param == "32" else "t%d @ (ParamType::I64 | ParamType::F64)" % (idx,) for (idx, param) in enumerate(params)])
    param_ty = ", ".join(["i" + x for x in params])
    if return_ty == None:
        return_ty = ""
        return_match = "None"
    else:
        if return_ty == "i32":
            return_match = "Some(ParamType::I32 | ParamType::Pointer)"
        else:
            return_match = "Some(ParamType::" + return_ty.upper() + ")"
        return_ty = " -> " + return_ty
    moves = "\n".join(["let %s = *%s;" % ("t" + str(i), "t" + str(i)) for i in range(len(params))])
    param_list = ", ".join(["arg_i%s(&_args[%d], &mut _cal, t%d)" % (params[i], i, i) for i in range(len(params))])
    return """([%s], %s) => {
    let func: Symbol<unsafe extern "C" fn(%s)%s> =
        lib.get(name.as_bytes()).unwrap();
    %s
    linker.func_new_unchecked(module, name, ty, move |mut _cal, _args| {
        func(%s);
        Ok(())
    }).unwrap();
}\n""" % (param_match, return_match, param_ty, return_ty, moves, param_list)


types = ["32", "64"]
return_types = []
for ty in types:
    return_types.append("i" + ty)
    return_types.append("f" + ty)
return_types.append(None)

body = ""
for arg_count in range(8):
    possible_args = itertools.product(types, repeat = arg_count)
    for args in possible_args:
        for return_ty in return_types:
            body += generate_case(args, return_ty)
print(body)
