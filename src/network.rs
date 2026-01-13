mod registry;
use registry::Registry;


mod matrix;
use matrix::CsrMatrix;
use crate::ast;

pub struct Network {
    species_registry: Registry<String>,
    reaction_registry: Registry<String>,
    complex_matrix : CsrMatrix<i64>,
    source_matrix : CsrMatrix<i64>,
    target_matrix : CsrMatrix<i64>,
}

impl Network {

    // pub fn build(reactions : Vec<ast::Reaction>) -> Self {
    //     let mut species_registry: Registry<String> = Registry::new();
    //     let mut reaction_names: Vec<Option<String>> = Vec::with_capacity(reactions.len());
    //     let mut complex_matrix: CooMatrix<i64> = CooMatrix::new();
    //     let mut source_matrix: CooMatrix<i64> = CooMatrix::new();
    //     let mut target_matrix: CooMatrix<i64> = CooMatrix::new();

    //     let mut complex_counter : usize = 0;
    //     for n in 0..reactions.len() {
    //         let reaction = reactions[n];
    //         reaction_names.push(reaction.name);
    //         for s in 0..reaction.left.terms.len(){
    //             complex_matrix.insert(s, complex_counter, reaction.left.terms[s]);
    //         }
    //         for s in 0..reaction.right.terms.len(){
    //             complex_matrix.insert(s, complex_counter, reaction.left.terms[s]);
    //         }
            
    //         match reaction.arrow  {
    //             ast::Arrow::Right => {
    //                 source_matrix.insert(complex_counter, n, 1);
    //                 target_matrix.insert(complex_counter + 1, n, 1);
    //             },
    //             ast::Arrow::Left  => {
    //                 source_matrix.insert(complex_counter + 1, n, 1);
    //                 target_matrix.insert(complex_counter, n, 1);
    //             },
    //             ast::Arrow::Reversible => {
    //                 source_matrix.insert(complex_counter, n, 1);
    //                 source_matrix.insert(complex_counter + 1, n, 1);
    //                 target_matrix.insert(complex_counter, n, 1);
    //                 target_matrix.insert(complex_counter+1, n, 1);
    //             },
    //         };

             
    //     }
    // }
}
