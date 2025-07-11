import json 
import csv
from reaction_net.chemical_formula import parse_chemical_formula

def load_network_txt(filename):
    with open(filename, 'r') as txtfile:
        rxn_net = txtfile.read()
    return rxn_net

def load_network_json(filename):
    with open(filename, 'r') as jsonfile:
        out = json.load(filename)
    return out

def save_network_json(network, filename):
    with open(filename, 'r') as jsonfile:
        out = json.dump(network, filename, indent=4)
    return out

def load_chemical_formula(filename):
    with open(filename, 'r' ,encoding  = "utf-8-sig") as csvfile:
        reader = csv.reader(csvfile)
        header = next(reader)
        print("CHEMICAL FORMULA FILE HEADER:\n " + ", ".join(header))
        idx = header.index('chemical_formula')

        out = {row[0].strip() : parse_chemical_formula(row[idx]) for row in reader}
    return out
        