import re 
from utils import enumerate_reactions, make_index_map


def make_matrix_builder(dok_generator):
    def build_matrix(row_basis, col_basis):
        shape = (len(row_basis), len(col_basis))

        return {
            "shape" : shape,
            "dok" : list(dok_generator(row_basis, col_basis))
        }
    return build_matrix

def target_matrix_dok(vertices, reactions):

    vertex_index = make_index_map(vertices)

    for n, reaction in enumerate_reactions(reactions):
        left, symbol, right = reaction
        y = vertex_index[right]
        yield (y, n,  1)

def source_matrix_dok(vertices, reactions):

    vertex_index = make_index_map(vertices)

    for n, reaction in enumerate_reactions(reactions):
        left, _, _ = reaction
        x = vertex_index[left]
        yield (x, n, 1)

def incidence_matrix_dok(vertices, reactions):

    vertex_index = make_index_map(vertices)

    for n, reaction in enumerate_reactions(reactions):
        
        left, _, right = reaction
        x, y = vertex_index[left], vertex_index[right]
        yield (x, n, -1)
        yield (y, n,  1)

def parse_vertex(vertex):       
    coef_pattern = r"^(\d*)\s*[\s*]\s*"
    plus_symbol = r"\s+\+\s+"
    for monomial in re.split(plus_symbol , vertex):
        match = re.split(coef_pattern, monomial)
        if len(match) == 3:
            _ , coef, sp = match
            coef = int(coef)
        else:
            coef, sp = 1, match[0]
        yield (coef, sp)

def vertex_label_matrix_dok(species, vertices):
    species_index = make_index_map(species)

    for n, vertex in enumerate(vertices):
        v = vertices[vertex]
        for sp in v:
            s = species_index[sp]
            yield (s, n, v[sp]) 

def stoichiometric_matrix_dok(species, reactions):
    species_index = make_index_map(species)
    for n, reaction in enumerate_reactions(reactions):
        left, _, right = reaction

        for (coef, sp) in parse_vertex(left):
            s = species_index[sp]
            yield (s, n, -coef) 

        for (coef, sp) in parse_vertex(right):
            s = species_index[sp]
            yield (s, n, coef) 


def reactant_matrix_dok(species, reactions):
    species_index = make_index_map(species)
    for n, reaction in enumerate_reactions(reactions):
        left, _, _ = reaction
        for (coef, sp) in parse_vertex(left):
            s = species_index[sp]
            yield (s, n, coef) 

def species_reaction_adjacency_dok(species, reactions):
    species_index = make_index_map(species)
    species_num = len(species)

    for n, reaction in enumerate_reactions(reactions):
        left, _, right = reaction
        for (coef, sp) in parse_vertex(left):
            s = species_index[sp]
            yield (s, species_num + n, coef) 

        for (coef, sp) in parse_vertex(right):
            s = species_index[sp]
            yield ( n + species_num,  s , coef)   