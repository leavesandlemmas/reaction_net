import re

def parse_chemical_formula(formula):
    pattern = r"([A-Z][a-z]?)(\d*)"
    return {e : safe_coef_parse(c) for e, c in re.findall(pattern, formula)} 

def safe_coef_parse(string):
    if string == "":
        return 1
    return int(string)

def get_element_map(element, species, formulas):
    return [formulas[s].get(element, 0) for s in species]

def get_all_elements(formulas):
    out = set()
    for s in formulas:
        formula = formulas[s]
        out |= formula.keys()
    return sorted(out)

def get_all_element_map(elements, species, formulas):
    return [get_element_map(element, species, formulas) for element in elements]