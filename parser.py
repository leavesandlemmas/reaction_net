def is_digit(x):
    return 48 <= ord(x) <= 57

def is_whitespace(x):
    return ord(x) in [9, 32]

def is_newline(x):
    return ord(x) == 10

