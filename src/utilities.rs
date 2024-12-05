use std::{collections::HashSet};

use itertools::Itertools;
use nalgebra::{DMatrix, Matrix};
use permutation::Permutation;

pub fn to_set(v:&Vec<u32>)->HashSet<u32>{
    v.iter().map(|x|*x).collect()
}
pub fn power_set (n:&u32)->Vec<Vec<u32>>{
    (0..2u32.pow(*n)).into_iter().map(|i|  get_subset(&i, &n)).collect()
  }
  pub fn get_subset(k:&u32,n:&u32)->Vec<u32>{
    /*n is the cardinality of the set X, therefore there are 2^n subset.
    k is a number in 0..2^n. We use its binary representation to build a set
    whose elements are the non-zero bit of n*/
    let mut subset: Vec<u32> = Vec::new();
    if k>=&2u32.pow(*n){panic!("k can't be grater then 2^n");}
    for i in 0..*n {
        if (k >> i)&1==1{
            subset.push(i);
            }
    }
    subset
}
pub fn ones_positions(k:u32,n:&usize)->Vec<u32>{
    (0..*n as u32).into_iter().filter(|x|(k>>x)&1==1).collect()

}

pub fn cartesian_product(set: &Vec<u32>) -> Vec<(u32, u32)> {
    let mut product: Vec<(u32, u32)> = Vec::new();
    for &a in set {
        for &b in set {
            product.push((a, b));
        }
    }
    product
}
pub fn subset_as_u32(k:&HashSet<u32>)->u32{
    k.iter().map(|x|2u32.pow(*x)).sum()
}
pub fn permutaton_matrix_from_permutation(n:&u32,sigma:&Permutation)->DMatrix<u32>{
/*     let permutations_of_n:Vec<Vec<u32>> = (0..*n).permutations(*n as usize).collect();
    if permutations_of_n.binary_search(sigma).is_err() {panic!("Sigma = {:?} is not a permutation of n = {}.",sigma,n)}
     */
    let identity: DMatrix<u32>=DMatrix::identity(*n as usize,*n as usize);
    let rows:Vec<Vec<u32>> = identity.row_iter().map(|x|x.iter().map(|z|*z).collect()).collect();

    let x: Vec<u32> =sigma.apply_slice(rows).concat();
    DMatrix::from_row_slice(*n as usize, *n as usize, &x).transpose()
}
pub fn n_to_binary_vec(k: &u32, width: &u32) -> Vec<u32> {
	format!("{:0>width$}", format!("{:b}", k), width = *width as usize)
		.chars()
		.map(|x| if x == '1' { 1u32 } else { 0u32 })
		.collect()
}
pub fn binary_to_u32(binary_vec:&Vec<u32>)->u32 {
    let s= binary_vec.iter().map(|x|x.to_string()).collect::<String>();
    let intval = u32::from_str_radix(&s, 2).unwrap();
    intval
}
pub fn representation_permutation_subset (k:&u32,sigma:&Permutation)->u32 {
    /*The input value is k in (0..2^n), therefore it represent a subset of H with |H|=n.
    Any occurrence of 1 in the binary representation of k correspond to an element in the subset S corresponding to k 
    Example: k=5="101"-> S={2,0}. 
    The input value sigma is a permutation of S_n. We build the corresponding permutation matrix and we make it act on the binary representation of k. 
    The we convert the result into u32
    
    We prefer to inverse and normalize sigma
    */
    let normalized_sigma = sigma.clone().inverse().normalize(true);
    let binary_k=n_to_binary_vec(&k,&(normalized_sigma.len() as u32));

    binary_to_u32(&normalized_sigma.apply_slice(binary_k))
}