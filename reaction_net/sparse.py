import scipy.sparse 

def build_matrix(row_basis, col_basis, dok_genfunc):
    shape = len(row_basis), len(col_basis)
    out = scipy.sparse.dok_array(shape, dtype = 'int64')
    for (i,j,val) in dok_genfunc(row_basis, col_basis):
        out[i,j] = val
    return out
