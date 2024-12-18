use core::panic;
use std::{collections::HashSet, fmt::Display, vec};
extern crate nalgebra as na;
use itertools::Itertools;
use nalgebra::DMatrix;
use permutation::Permutation;
use rand:: Rng;
use crate::utilities::{binary_to_u32, cartesian_product, from_tag_to_vec, get_subset, n_to_binary_vec, ones_positions, representation_permutation_subset, representing_hypergroupoid, subset_as_u32, to_set};
#[derive(Debug, Clone,PartialEq)]
pub struct HyperGroupoidMat{
    pub h:HashSet<u32>,
    pub hyper_composition:DMatrix<u32>,
    pub n:u32
}
impl HyperGroupoidMat {
   pub fn new_random_from_cardinality(n:&u32)->Self{
    let h_vec=(0..*n as u32).into_iter().collect();
        let ht = get_random_hypercomposition_matrix(&n);
        HyperGroupoidMat { 
            h: h_vec, 
            hyper_composition: ht, 
            n: *n}
   }
   pub fn new_from_matrix(matrix:&DMatrix<u32>)->Self{
    if !matrix.is_square(){panic!("Matrix must be a square matrix!")}
    let a:Vec<&u32>= matrix.iter().filter(|a|**a==0).collect();
    if !a.is_empty(){panic!("In order to have a hypergroupoid, matrix can't contain zeroes!")}
    let h:HashSet<u32>= (0..matrix.ncols() as u32).into_iter().collect();
    let n=matrix.ncols();
    HyperGroupoidMat{
        h:h,
        hyper_composition:matrix.clone(),
        n:n as u32
   }
}
pub fn new_from_tag(mut tag:u128,cardinality:&u32)->Self{
    let vector_of_subsets_as_integers=from_tag_to_vec(&mut tag, &cardinality);
    let vector_of_subsets_as_integers: Vec<u32>=vector_of_subsets_as_integers.iter().map(|x|binary_to_u32(x)).collect();

    let hyper_composition_matrix = DMatrix::from_row_slice(*cardinality as usize, *cardinality as usize, &vector_of_subsets_as_integers);
        HyperGroupoidMat::new_from_matrix(&hyper_composition_matrix)
}
pub fn get_integer_tag(&self)->u32{

    binary_to_u32(&self.hyper_composition
        .transpose()//transpose is required because iteration in matrix is by column
        .iter()
        .map(|x|n_to_binary_vec(&(*x as u128), &self.n))
        .rev()
        .concat())

}
pub fn permutation_of_table(&self,sigma:&Permutation)->Self{
    let permutation_hypergroupoids = &self.hyper_composition;
    let alfa =DMatrix::from_iterator(self.n as usize, self.n as usize, 
        permutation_hypergroupoids.iter()
            .map(|x| representation_permutation_subset(&(*x as u128),&sigma)));
    
    HyperGroupoidMat { 
        h: self.h.clone(), 
        hyper_composition:alfa, 
        n: self.n}
}
pub fn is_hypergroup(&self)->bool{
    self.is_associative()&&self.is_reproductive()

}
pub fn is_commutative(&self)->bool{
    for a in self.get_singleton().iter(){
        for b in self.get_singleton().iter(){
            let ab=self.mul_by_representation(a, b);
            let ba=self.mul_by_representation(b, a);
            if ab!=ba {return false;}
        }
    }
    true
}
pub fn get_subset_from_k(&self,k:&u32)->HashSet<u32>{
    /*
    k is a number in 0..2^n-1. We use its binary representation to build a set
    whose elements are the non-zero bits of n*/
    let n = self.h.len() as u32;
    let mut subset: Vec<u32> = Vec::new();
    if k>=&2u32.pow(n){panic!("k can't be grater then 2^n");}
    for i in 0..n {
        if (k >> i)&1==1{
            subset.push(i);
            }
    }
    to_set(&subset)
}
   pub fn mul_by_representation(&self,int_k:&u32,int_l:&u32)->u32{
    let ones_k=ones_positions(*int_k, &self.h.len());
    let ones_l= ones_positions(*int_l, &self.h.len());
    let mut indexes:Vec<(u32,u32)>=Vec::new();
    for a in &ones_k{
        for b in &ones_l{
                indexes.push((*a,*b));
        }
    }
    indexes.iter().fold(0u32, |acc, x| acc|self.hyper_composition[(x.0 as usize,x.1 as usize)])
}

pub fn mul(&self,subset_k:&HashSet<u32>,subset_l:&HashSet<u32>)->u32{
    if !subset_k.is_subset(&self.h)||!subset_l.is_subset(&self.h) { panic!("K and L must be a subsets of H!")};
    let int_k=subset_as_u32(&subset_k);
    let int_l=subset_as_u32(&subset_l);
self.mul_by_representation(&int_k, &int_l)   
}
pub fn left_division(&self,a:&u32,b:&u32)->u32{
    /*This function compute the value b\a={x in H s.t. a in bx} */
    
    let sub_a=to_set(&get_subset(&2u32.pow(*a), &self.n));
    let sub_b=2u32.pow(*b);
    self.get_singleton().iter()
    .filter(
        |x| sub_a.is_subset(
            &to_set(&get_subset(
                        &self.mul_by_representation(&sub_b, x), &self.n)
                    )
                )
            ).fold(0, |acc,t|acc|t)

   
}
pub fn right_division(&self,a:&u32,b:&u32)->u32{
        /*This function compute the value a/b={x in H s.t. a in xb} */
        let sub_a=to_set(&get_subset(&2u32.pow(*a), &self.n));
    let sub_b=2u32.pow(*b);
    self.get_singleton().iter()
    .filter(
        |x| sub_a.is_subset(
            &to_set(&get_subset(
                        &self.mul_by_representation(x,&sub_b), &self.n)
                    )
                )
            ).fold(0, |acc,t|acc|t)

   

}
   pub fn is_reproductive(&self)->bool{
    let h:Vec<u32>=Vec::from_iter(0..self.n).iter().map(|_|2u32.pow(self.n)-1).collect();
    /*xH is row_sum */
    let row_sum:Vec<u32> = self.hyper_composition.row_iter().map(|x|x.iter().fold(0u32, |acc,element|acc|element)).collect();
    /*Hx is column sum */
    let col_sum:Vec<u32> = self.hyper_composition.column_iter().map(|x|x.iter().fold(0u32, |acc,element|acc|element)).collect();
    if h==row_sum&&h==col_sum {
        true
    }
    else {
        false
    }
   }
pub fn fix_reproductivity(&self)->Self{
    let h=2u32.pow(self.n)-1;
    if self.is_reproductive(){return self.clone();}
    let mut new_hypergroupoid_matrix = self.hyper_composition.clone();
    let mut missing_terms:u32;
    let row_sum:Vec<u32> = new_hypergroupoid_matrix.row_iter().map(|x|x.iter().fold(0u32, |acc,element|acc|element)).collect();
    
        for j in 0..self.n as usize /*here j is row index*/{
            if row_sum[j]<h {
                missing_terms=h-row_sum[j];
                let position_max = new_hypergroupoid_matrix.row(j).iter().position_max().unwrap();
                new_hypergroupoid_matrix[(j,position_max as usize)]|=missing_terms;
            }
            
        }            

    let new_hypergroupoid = HyperGroupoidMat{
        h:self.h.clone(),
        hyper_composition:new_hypergroupoid_matrix.clone(),
        n:self.n
    };
    if new_hypergroupoid.is_reproductive(){return new_hypergroupoid;}
    else {
        let col_sum:Vec<u32> = new_hypergroupoid_matrix.column_iter().map(|x|x.iter().fold(0u32, |acc,element|acc|element)).collect();
    
        for j in 0..self.n as usize{
            if col_sum[j]<h {
                missing_terms=h-col_sum[j];
                let position_max = new_hypergroupoid_matrix.column(j).iter().position_max().unwrap();
                new_hypergroupoid_matrix[(position_max as usize,j as usize)]|=missing_terms;
            }         
    }

    }
    HyperGroupoidMat{
        h:self.h.clone(),
        hyper_composition:new_hypergroupoid_matrix,
        n:self.n
    }
}
pub fn is_associative(&self)->bool{
    for a in &self.get_singleton(){
        for b in &self.get_singleton(){
            for c in &self.get_singleton(){
                let ab_c=self.mul_by_representation(
                    &self.mul_by_representation(&a, &b),&c);
                let a_bc = self.mul_by_representation(&a, &self.mul_by_representation(&b, &c));
                if a_bc==ab_c{continue;}else {
                        return false;
                    }
            }
        }
    }
true
}
pub fn get_singleton(&self)->DMatrix<u32>{
    DMatrix::from_row_iterator(1, self.n as usize, (0..self.n).into_iter().map(|i|2u32.pow(i)))
}
}
impl Display for HyperGroupoidMat{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let table:DMatrix<String>=DMatrix::from_iterator(self.n as usize, self.n as usize, 
            self.hyper_composition.iter().map(|x|format!("{:?}",to_set(&get_subset(x, &self.n)))));
        
        write!(f, "\nH: {:?},\nHypercomposition table:\n{} It is represented by: {}Size:{}\n", self.h, table, self.hyper_composition, self.n )
    }
}


pub fn get_random_hypercomposition_table(n:&u32)->Vec<Vec<u32>>{
    let vec: Vec<u32>=(0u32..*n as u32).into_iter().map(|x|x).collect();
    let index_cartesian=cartesian_product(&vec);
    let mut rng = rand::thread_rng();
    let mut hypercomposition_table=vec![vec![0u32;*n as usize];*n as usize];
    
    for item in index_cartesian {
        hypercomposition_table[item.0 as usize][item.1 as usize]=rng.gen_range(1..2u32.pow(*n as u32))
} 
hypercomposition_table
}
pub fn get_random_hypercomposition_matrix(n:&u32)->DMatrix<u32>{
    let mut rng = rand::thread_rng();
    let m  =DMatrix::from_iterator(*n as usize, *n as usize, (0..n.pow(2)).into_iter().map(|_|rng.gen_range(1..2u32.pow(*n as u32))));
    m
} 
pub fn collect_hypergroupoid(cardinality:&u32)->Vec<u128>{
    let size = cardinality.pow(3);
    let x = 2u128.pow(size-cardinality);
    let y = 2u128.pow(size);
    
        println!("size is {size}");
        println!("there are {} to be tested", y-x);

    (2u128.pow(size-cardinality)..2u128.pow(size)).into_iter().filter(|i|representing_hypergroupoid(&mut i.clone(),&cardinality)).collect()
}
pub fn collect_hypergroups(cardinality:&u32)->Vec<u128>{
    let size = cardinality.pow(3);
    (2u128.pow(size-cardinality)..2u128.pow(size)).into_iter().filter(|i|representing_hypergroupoid(&mut i.clone(),&cardinality)&&HyperGroupoidMat::new_from_tag(*i, cardinality).is_hypergroup()).collect()

}
