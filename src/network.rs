

pub struct NetworkBuilder {
    species_registry : Registry<String>,
    complex_matrix : CooMatrix<i64>,
    source_matrix : CooMatrix<i64>,
    target_matrix : CooMatrix<i64>,
}

impl NetworkBuilder {

     pub fn new() -> Self {
        let species_registry = Registry::new();
         let complex_matrix: CooMatrix<i64> = CooMatrix::new();
         let source_matrix: CooMatrix<i64> = CooMatrix::new();
         let target_matrix: CooMatrix<i64> = CooMatrix::new();
        Self {species_registry, complex_matrix, source_matrix, target_matrix}
       }
}


