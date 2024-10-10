def make_index_map(iterable):
    return {x : n for n, x in enumerate(iterable)}

def unique_list(iterable):
    return sorted(set(iterable))

def enumerate_reactions(reactions):
    if isinstance(reactions, dict):
        for n, name in enumerate(reactions):
            yield n, reactions[name]
    else:
        for n, reaction in enumerate(reactions):
            yield n, reaction  