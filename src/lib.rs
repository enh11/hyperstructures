//! This library contains utilities for Hyperstructures.
//!
#[macro_use]
pub mod hs;
pub mod hypergroups;
pub mod utilities;
pub mod tags;
pub mod enumeration;
pub mod unital_magma;
pub mod relations;
pub mod fuzzy;
pub mod generating_functions;

#[cfg(test)]
mod tests {
    use nalgebra::DMatrix;
    use crate::{hs::HyperGroupoid, hypergroups::HyperGroup};
    #[test]
    fn example_corsini_112_1() {
        let cardinality = 4u64;
        let matrix = 
            DMatrix::from_row_slice(cardinality as usize, cardinality as usize, 
                &[1,1,7,11,1,1,7,11,7,7,7,12,11,11,12,11]);
        let hg = HyperGroup::new_from_matrix(&matrix);
        let subset_a = 1u64;
        let subset_b = 7u64;
        let subset_c = 11u64;

        assert!(!hg.subhypergroup_is_closed(&subset_a));
        assert!(!hg.subhypergroup_is_closed(&subset_b));
        assert!(!hg.subhypergroup_is_closed(&subset_c));
    }
    #[test]
    fn example_corsini_112_3() {
        let cardinality = 4u64;
        let matrix = 
        DMatrix::from_row_slice(cardinality as usize, cardinality as usize, 
            &[1,4,4,14,4,2,4,13,4,4,4,15,14,13,15,15]);
        let hg = HyperGroup::new_from_matrix(&matrix);
        let subset_a = 1u64;
        let subset_b = 2u64;

        assert!(hg.subhypergroup_is_closed(&subset_a));
        assert!(hg.subhypergroup_is_closed(&subset_b));

        assert!(!hg.subhypergroup_is_invertible(&subset_a));
        assert!(!hg.subhypergroup_is_invertible(&subset_b)); 
    }
        #[test]
    fn example_corsini_112_4() {
        let cardinality = 5u64;
        let matrix = DMatrix::from_row_slice(cardinality as usize, cardinality as usize, &[1,2,4,8,24,2,3,24,28,28,4,24,5,26,26,8,28,26,31,31,24,28,26,31,31]);
        let hg = HyperGroup::new_from_matrix(&matrix);
        let subset_ab = 3u64;
        let subset_ac = 5u64;

        assert!(hg.subhypergroup_is_invertible(&subset_ab));
        assert!(hg.subhypergroup_is_invertible(&subset_ac));

        let subset_a=1u64;
        assert!(hg.subhypergroup_is_closed(&subset_a));
        assert!(!hg.subhypergroup_is_invertible(&subset_a));

    }
    #[test]
    ///
    /// Generate the transposition hypergroup with hyperoperation defined as 
    /// `x+y = {max(x,y)} if x != y`
    /// `x+y = {z ∈ H | z≤x} if x = y`.
    /// 
    fn example_transposition_hg() {
        let cardinality = 5;
        let function = {
            |a:u64,b:u64| 
                if a!=b {return 1<<a.max(b)}
                else {return (0..=a).into_iter().fold(0, |acc,x|acc|1<<x)}
            };
        let hg = HyperGroup::new_from_function(function, &cardinality).unwrap();
        assert!(hg.is_transposition());

    }
    #[test]
    ///
    /// Tests here come from `Some Remarks on Hyperstructures their Connections with Fuzzy Sets and Extensions to Weak Structures` by `Piergiulio Corsini`.
    /// These Hv-hypergroupoids of order 2, which are not associative.
    /// 
    fn example_fuzzy_degree() {
    //H12
    let cardinality =2u64;
    let input_values  = vec![vec![0,1],   vec![0,1],
                                            vec![1],vec![0]];
    let hs = HyperGroupoid::new_from_elements(&input_values, cardinality);
    let degree = hs.get_fuzzy_degree();
    assert_eq!(degree,1usize);
    //H13
    let cardinality =2u64;
    let input_values  = vec![vec![0,1],   vec![1],
                                            vec![0,1],vec![0]];
    let hs = HyperGroupoid::new_from_elements(&input_values, cardinality);
    let degree = hs.get_fuzzy_degree();
    assert_eq!(degree,1usize);
    //H15
    let cardinality =2u64;
    let input_values  = vec![vec![0,1],   vec![0],
                                            vec![1],vec![0,1]];
    let hs = HyperGroupoid::new_from_elements(&input_values, cardinality);
    let degree = hs.get_fuzzy_degree();
    assert_eq!(degree,1usize);
    //H9
    let cardinality =2u64;
    let input_values  = vec![vec![0,1],   vec![1],
                                            vec![1],vec![0]];
    let hs = HyperGroupoid::new_from_elements(&input_values, cardinality);
    let degree = hs.get_fuzzy_degree();
    assert_eq!(degree,2usize);
            
    //H10
    let cardinality =2u64;
    let input_values  = vec![vec![0],   vec![0,1],
                                            vec![1],vec![0]];
    let hs = HyperGroupoid::new_from_elements(&input_values, cardinality);
    let degree = hs.get_fuzzy_degree();
    assert_eq!(degree,2usize);

    //H11
    let cardinality =2u64;
    let input_values  = vec![vec![1],   vec![0,1],
                                            vec![0],vec![1]];
    let hs = HyperGroupoid::new_from_elements(&input_values, cardinality);
    let degree = hs.get_fuzzy_degree();
    assert_eq!(degree,2usize);
    //H14
    let cardinality =2u64;
    let input_values  = vec![vec![0,1],   vec![0],
                                            vec![0],vec![0,1]];
    let hs = HyperGroupoid::new_from_elements(&input_values, cardinality);
    let degree = hs.get_fuzzy_degree();
    assert_eq!(degree,2usize);
    //H16
    let cardinality =2u64;
    let input_values  = vec![vec![0],   vec![0,1],
                                            vec![0,1],vec![0]];
    let hs = HyperGroupoid::new_from_elements(&input_values, cardinality);
    let degree = hs.get_fuzzy_degree();
    assert_eq!(degree,2usize);
   //H17
    let cardinality =2u64;
    let input_values  = vec![vec![0,1],   vec![0,1],
                                            vec![0],vec![0,1]];
    let hs = HyperGroupoid::new_from_elements(&input_values, cardinality);
    let degree = hs.get_fuzzy_degree();
    assert_eq!(degree,2usize);
   //H18
    let cardinality =2u64;
    let input_values  = vec![vec![0,1],   vec![0,1],
                                            vec![1],vec![0,1]];
    let hs = HyperGroupoid::new_from_elements(&input_values, cardinality);
    let degree = hs.get_fuzzy_degree();
    assert_eq!(degree,2usize);

    }
}



