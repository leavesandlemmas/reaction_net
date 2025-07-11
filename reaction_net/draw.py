from reaction_net.matrix_builder import species_reaction_adjacency_dok

def write_dot(draw_generator):
    indent = " " * 4
    newline = "\n" + indent
    graph = newline.join(draw_generator)
    return "\n".join(["digraph {", indent + graph ,  "}"])

def graph_from_adjacency(adjacency_dok):
    for row, col, val in adjacency_dok:
        if val > 1:
            yield '{} -> {} [label = "{}";]'.format(col, row, val) 
        else:
            yield '{} -> {}'.format(col, row) 

def draw_species_reaction_graph(species, reactions):

    yield 'node [style = none, shape = circle]'
    for n, s in enumerate(species):
        yield '{} [label = "{}";]'.format(n, s)
    
    yield 'node [style = filled, shape = box]'
    for n, r in enumerate(reactions, len(species)):
        yield str(n)

    adj_dok = species_reaction_adjacency_dok(species, reactions)    
    for string in  graph_from_adjacency(adj_dok):
        yield string