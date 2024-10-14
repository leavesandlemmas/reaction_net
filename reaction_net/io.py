import json 

def load_network_json(filename):
    with open(filename, 'r') as jsonfile:
        out = json.load(filename)

    return out


def save_network_json(ne, filename):
    with open(filename, 'r') as jsonfile:
        out = json.load(filename)

    return out
