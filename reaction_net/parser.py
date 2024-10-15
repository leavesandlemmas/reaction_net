import re
import itertools
from reaction_net.utils import unique_list
   
def parse_reaction(reaction):
    
    pattern = r"\s+(<->|->|=|<-)\s+"
    reactants, symbol, products = re.split(pattern, reaction)
    left, right = standardize(reactants), standardize(products)

    if symbol == "<-":    
        return right, '->', left
    
    if symbol == "=":
        return left, '<->', right
    
    return left, symbol, right
    
def standardize(cpl):
    pattern = r"\s+\+\s+"
    out = re.split(pattern , cpl)
    out = [s.strip() for s in out]
    
    # sort to ensure equivalent complexes have the same order
    # ex. E + S = S + E
    out.sort() 
    out = ' + '.join(out)
    return out

def get_vertices(parsed_reactions):
    if isinstance(parsed_reactions, dict):
       genexpr = (y for x in parsed_reactions 
                  for y in (parsed_reactions[x][0], parsed_reactions[x][2])) 
    else:
        genexpr = (y for x in parsed_reactions for y in (x[0], x[2]))
    return unique_list(genexpr)

def strip_coef(monomial):
    pattern = r"^\d*\s*[*]?\s*"
    if re.match(pattern, monomial):
        return re.split(pattern, monomial)[1]
    return monomial

def get_coef(monomial):
    pattern = r"^(\d*)\s*[*]?\s*"
    coef, species =  re.split(pattern, monomial)[1:]
    c = int(coef) if not (coef == '') else 1
    return species, c

def extract_species(vertex):
    pattern = r"\s+\+\s+"
    return map(strip_coef, re.split(pattern , vertex))

def get_species_coef(vertex):
    pattern = r"\s+\+\s+"
    return map(get_coef, re.split(pattern , vertex))

def get_species(vertices):
    species = (s for v in vertices for s in extract_species(v))
    filtered = filter(lambda x : x != "", set(species))
    return sorted(filtered)

def get_reaction_names(reactions):

    pattern = "\s+[:]\s+"
    for n, reaction in enumerate(reactions):
        split = re.split(pattern, reaction, maxsplit=1)
        if len(split) == 1:
            yield n, reaction
        else:  
            yield (split[0], split[1])

def process_vertices(vertices):
    for vertex in vertices:
        yield vertex, {  s : c  for s, c in get_species_coef(vertex) if vertex != "0"}

def parse_reaction_network(rxnet,named=True):
    rxnet = "".join(rxnet) if isinstance(rxnet, list) else rxnet   
    reactions = rxnet.splitlines()  
    named_reactions = get_reaction_names(reactions)
    if named:
        parsed_reactions = {name : parse_reaction(r) for name, r in named_reactions} 
    else:
        parsed_reactions = [parse_reaction(r) for name, r in named_reactions]
    vertices = get_vertices(parsed_reactions)
    species = get_species(vertices)
    
    return {   
        "species" : species,
        "vertices" : dict(process_vertices(vertices)),
        "reactions" : parsed_reactions
    }