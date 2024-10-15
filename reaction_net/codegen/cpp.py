from itertools import chain

def make_linear_function(shape, dok, input_symbol="input", output_symbol="out"):
    indent = " " * 4
    newline = "\n" + indent
    type_template ="template<typename T>" 
    signature = "array<T, {0}> linear_map(array<T, {1}> {2})".format(
        shape[0], shape[1], input_symbol) + "{"
    
    eqns = convert_from_dok(shape, dok, 
                            input_symbol=input_symbol, output_symbol=output_symbol)
    iterable = (
        ["array<T, {}> {};".format(shape[0], output_symbol)] + 
        eqns + ["return {};".format(output_symbol)] )
    expressions = indent + newline.join(iterable) 

    return "\n".join([type_template, signature,  expressions, "}\n"]) 


def convert_from_dok(shape, dok, input_symbol = "x", output_symbol = "out"):
    out = {n : [] for n in range(shape[0])}
    for row, col, val in dok:
        if val == -1:
            string = "-{}[{}]".format(input_symbol, col)
        if val == 1:
            string = "{}[{}]".format(input_symbol, col)
        if val == 0:
            string = ""
        if val > 1 or val < -1:
            string = "{2}*{0}[{1}]".format(input_symbol, col, val)

        out[row].append(string)
    return ["{}[{}] = ".format(output_symbol, row) + " + ".join(out[row]) + ";"  for row in out]
