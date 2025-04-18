use core::panic;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::{collections::HashSet, fmt::Display, ops::Index, usize::MAX, vec};
extern crate nalgebra as na;
use itertools::Itertools;
use nalgebra::DMatrix;
use permutation::Permutation;
use rand::{Error,  Rng};
use crate::{relations::Relation, utilities::{binary_to_n, binary_to_u1024, cartesian_product, from_tag_to_vec, from_tag_u1024_to_vec, get_subset, n_to_binary_vec, ones_positions, permutaton_matrix_from_permutation, representation_permutation_subset, representing_hypergroupoid, representing_hypergroupoid_u1024, subset_as_u64, vec_to_set, U1024}};
#[derive(Debug, Clone,PartialEq)]
pub struct HyperGroupoidMat{
    pub h:HashSet<u64>,
    pub hyper_composition:DMatrix<u64>,
    pub n:u64
}
impl HyperGroupoidMat {
/// Generate a new random hyperstructure with cardinality n.
/// 
/// # Example
/// ```
/// use hyperstruc::hs::HyperGroupoidMat;
/// 
/// let n  =4u64;
/// let hyperstructure = HyperGroupoidMat::new_random_from_cardinality(&n);
/// println!("{hyperstructure}");
///  
   pub fn new_random_from_cardinality(n:&u64)->Self{
    let h_vec=(0..*n as u64).into_iter().collect();
        let ht = get_random_hypercomposition_matrix(&n);
        HyperGroupoidMat { 
            h: h_vec, 
            hyper_composition: ht, 
            n: *n}
   }
/// Generate a new hyperstructure given a square matrix. Every entry in the matrix are u64 and represent a subset of H={0,1,2,...,n},
/// where n is the size of the matrix, i.e., the cardinality of the new hyperstructure.
/// In particular, if x,y are elements of H, then x*y is the entries in position (x,y). 
/// For more detail about representation, see pub fn get_subset() in utilities.rs.
/// # Example
/// ```
/// use hyperstruc::hs::HyperGroupoidMat;
/// use nalgebra::DMatrix;
/// let matrix=DMatrix::from_row_slice(3usize,3usize,&[1,2,7,2,7,7,7,7,5]);
/// let hyperstructure=HyperGroupoidMat::new_from_matrix(&matrix);
/// println!("{hyperstructure}");
///
   pub fn new_from_matrix(matrix:&DMatrix<u64>)->Self{
    if !matrix.is_square(){panic!("Matrix must be a square matrix!")}
    let a:Vec<&u64>= matrix.iter().filter(|a|**a==0).collect();
    if !a.is_empty(){panic!("In order to have a hypergroupoid, matrix can't contain zeroes!")}
    let h:HashSet<u64>= (0..matrix.ncols() as u64).into_iter().collect();
    let n=matrix.ncols();
    HyperGroupoidMat{
        h:h,
        hyper_composition:matrix.clone(),
        n:n as u64
   }
}

/// Generate a new hyperstructure given a tag and the cardinality of the set H. If n is the cardinality, then tag is a u128 less than or equal to n^3. 
/// Its binary representation, divided in groups of n-bits, provide the table of hyperoperation; each group of n-bits corresponds to a subset of H. 
/// For example, if n=2, then a tag must be less or equal to 2^8. The tag t=185 has binary representation 10111000, divided in groups of 2-bits it is
/// 10-11-10-01. The bits 10 represent the subset {1}, the bits 11 represents {0,1}, the bits 01 represents {0}.
/// With this example it follows that 0*0={1}, 0*1={0,1}, 1*0 = {1} and 1*1 = emptyset.
/// 
/// # Example
/// 
/// ```
/// use hyperstruc::hs::HyperGroupoidMat;
/// use nalgebra::DMatrix;
/// let cardinality = 2;
/// let t=185;
/// let new_hyperstructure_from_tag = HyperGroupoidMat::new_from_tag(&t,&cardinality);
/// let new_hyperstructure_from_matrix = HyperGroupoidMat::new_from_matrix(&DMatrix::from_row_slice(2usize,2usize,&[2,3,2,1]));
/// assert_eq!(new_hyperstructure_from_tag, new_hyperstructure_from_matrix)
/// 
/// 
pub fn new_from_tag(tag:&u128,cardinality:&u64)->Self{
    let vector_of_subsets_as_integers=from_tag_to_vec(&tag, &cardinality);
    let vector_of_subsets_as_integers: Vec<u64>=vector_of_subsets_as_integers.iter().map(|x|binary_to_n(x)).collect();

    let hyper_composition_matrix = DMatrix::from_row_slice(*cardinality as usize, *cardinality as usize, &vector_of_subsets_as_integers);
        HyperGroupoidMat::new_from_matrix(&hyper_composition_matrix)
}
/// Generate a new hyperstructure given a tag and the cardinality of the set H. If n is the cardinality, then tag is a u128 less than or equal to n^3. 
/// Its binary representation, divided in groups of n-bits, provide the table of hyperoperation; each group of n-bits corresponds to a subset of H. 
/// For example, if n=2, then a tag must be less or equal to 2^8. The tag t=185 has binary representation 10111000, divided in groups of 2-bits it is
/// 10-11-10-01. The bits 10 represent the subset {1}, the bits 11 represents {0,1}, the bits 01 represents {0}.
/// With this example it follows that 0*0={1}, 0*1={0,1}, 1*0 = {1} and 1*1 = emptyset.
/// 
/// # Example
/// 
/// ```
/// use hyperstruc::hs::HyperGroupoidMat;
/// use hyperstruc::utilities::U1024;
/// 
/// let cardinality = 6u64;
/// let hs = HyperGroupoidMat::new_random_from_cardinality(&cardinality);
/// let tag_hs = hs.get_integer_tag_u1024();
/// let check_hs = HyperGroupoidMat::new_from_tag_u1024(&tag_hs, &cardinality);
/// assert_eq!(check_hs,hs)
/// 
/// 
pub fn new_from_tag_u1024(mut tag:&U1024,cardinality:&u64)->Self{
    let vector_of_subsets_as_integers=from_tag_u1024_to_vec(&mut tag, &cardinality);
    let vector_of_subsets_as_integers: Vec<u64>=vector_of_subsets_as_integers.iter().map(|x|binary_to_n(x)).collect();

    let hyper_composition_matrix = DMatrix::from_row_slice(*cardinality as usize, *cardinality as usize, &vector_of_subsets_as_integers);
        HyperGroupoidMat::new_from_matrix(&hyper_composition_matrix)
}
 /// # Example
/// 
/// ```
/// use hyperstruc::hs::HyperGroupoidMat;
/// use nalgebra::DMatrix;
/// let cardinality = 2;
/// let t=185;
/// let new_hyperstructure_from_tag = HyperGroupoidMat::new_from_tag(&t,&cardinality);
/// let new_hyperstructure_from_matrix = HyperGroupoidMat::new_from_matrix(&DMatrix::from_row_slice(2usize,2usize,&[2,3,2,1]));
/// assert_eq!(new_hyperstructure_from_tag.get_integer_tag(), new_hyperstructure_from_matrix.get_integer_tag())
/// 
/// 
pub fn get_integer_tag(&self)->u128{

    binary_to_n(&self.hyper_composition
        .transpose()//transpose is required because iteration in matrix is by column
        .iter()
        .map(|x|n_to_binary_vec(&(*x as u128), &(self.n as u64)))
        .concat())

}
/// # Example
/// 
/// ```
/// use hyperstruc::hs::HyperGroupoidMat;
/// use nalgebra::DMatrix;
/// use hyperstruc::utilities::U1024;
/// 
/// let cardinality = 2;
/// let t=185;
/// let new_hyperstructure_from_tag = HyperGroupoidMat::new_from_tag(&t,&cardinality);
/// let new_hyperstructure_from_matrix = HyperGroupoidMat::new_from_matrix(&DMatrix::from_row_slice(2usize,2usize,&[2,3,2,1]));
/// let tag1 = new_hyperstructure_from_tag.get_integer_tag_u1024();
/// let tag2 = new_hyperstructure_from_matrix.get_integer_tag_u1024();
/// 
/// assert_eq!(tag1,tag2);
/// assert_eq!(tag1,U1024::from(t));
///
/// 
pub fn get_integer_tag_u1024(&self)->U1024 {
    let binary_vector:Vec<u64> = self.hyper_composition
        .transpose()//transpose is required because iteration in matrix is by column
        .iter()
        .map(|x|n_to_binary_vec(&(*x as u128), &(self.n as u64)))
        .concat()
        .into_iter()
        .collect();
    U1024::from_binary_vec(&binary_vector)    //binary_vector.iter().fold(U1024::from(0u64), |acc,x|acc+U1024::from(2).pow(U1024::from(*x)))
}
pub fn permutation_of_table(&self,sigma:&Permutation)->Self{
    let permutation_hypergroupoids = &self.hyper_composition;
    let alfa =DMatrix::from_iterator(self.n as usize, self.n as usize, 
        permutation_hypergroupoids.iter()
            .map(|x| representation_permutation_subset(&(*x as u128),&sigma).try_into().unwrap()));
    
    HyperGroupoidMat { 
        h: self.h.clone(), 
        hyper_composition:alfa, 
        n: self.n}
}
pub fn isomorphic_hypergroup_from_permutation(&self, sigma:&Permutation)->Self{
    let perm_mat = permutaton_matrix_from_permutation(&(self.n as u64), &sigma.clone());
    let isomorphic_matrix=perm_mat.clone()*self.permutation_of_table(sigma).hyper_composition*perm_mat.transpose();
    HyperGroupoidMat::new_from_matrix(&isomorphic_matrix)
}
///
/// Collect isomorphism class of an hypergroup. Elements in the class are obtained by permutation of the set defining the hyperstructure.
/// The representant of the class is chosen to be the smaller among the tags in the class.
/// It returns a tuple (representant, class), where class is a vector of tags.
/// 
pub fn collect_isomorphism_class(&self)->(U1024,Vec<U1024>){
    let cardinality = self.n as usize;
    let permut_vec:Vec<Vec<usize>> = (0..cardinality).permutations(cardinality ).collect();
    let permutation:Vec<Permutation> = permut_vec.iter().map(|sigma| Permutation::oneline(sigma.clone())).collect();
    let mut isomorphism_classes:Vec<U1024>=Vec::new();

    for sigma in &permutation {        
        let isomorphic_image_tag = self.isomorphic_hypergroup_from_permutation(&sigma).get_integer_tag_u1024();
        isomorphism_classes.push(isomorphic_image_tag.try_into().unwrap());

    }
    isomorphism_classes=isomorphism_classes.iter().sorted().dedup().map(|x|*x).collect();
    let representant_of_class=isomorphism_classes.iter().min().unwrap();
        
       (*representant_of_class,isomorphism_classes)

}
///
/// 
/// # Example
/// 
/// ```
/// use hyperstruc::hs::HyperGroupoidMat;
/// 
/// let cardinality = 3u64;
/// let tag = 22150143u128;
/// let hs = HyperGroupoidMat::new_from_tag(&tag,&cardinality);
/// 
/// assert!(hs.is_hypergroup())
/// 
pub fn is_hypergroup(&self)->bool{
    self.is_associative()&self.is_reproductive()

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

pub fn is_left_identity(&self,e:&u64)->bool{
    if !e.is_power_of_two() {panic!("Not an element in hypergroupoid!")}
    let e=e.trailing_zeros();//the number of trailing_zeros in a power of two integer is equal to the exponent of that power.
    let row_e=self.hyper_composition.row(e as usize);
    (0..self.n).into_par_iter().all(|x| (row_e.index(x as usize)>>x)&1==1)

}
pub fn is_right_identity(&self,e:&u64)->bool{
    if !e.is_power_of_two() {panic!("Not an element in hypergroupoid!")}
    let e=e.trailing_zeros();//the number of trailing_zeros in a power of two integer is equal to the exponent of that power.
    let col_e=self.hyper_composition.column(e as usize);
    (0..self.n).into_par_iter().all(|x| (col_e.index(x as usize)>>x)&1==1)
    }
pub fn is_identity(&self,e:&u64)->bool{
    self.is_left_identity(&e)&&self.is_right_identity(&e)
}
pub fn collect_left_identity(&self)->Vec<u64>{
    self.get_singleton().par_iter()
        .filter(|e|self.is_left_identity(e))
        .map(|e|*e)
        .collect()

}
pub fn collect_right_identity(&self)->Vec<u64>{
    self.get_singleton().par_iter()
        .filter(|e|self.is_right_identity(e))
        .map(|e|*e)
        .collect()

}
pub fn collect_identities(&self)->Vec<u64>{
    self.get_singleton().iter()
        .filter(|e|self.is_right_identity(&e)&&self.is_left_identity(&e))
        .map(|e|*e)
        .collect_vec()

}
pub fn is_left_scalar(&self,s:&u64)->bool{
    if !s.is_power_of_two() {panic!("Not an element in hypergroupoid!")}
    let s=s.trailing_zeros();
    self.hyper_composition.column(s as usize).iter().all(|x|x.is_power_of_two())
}
pub fn is_right_scalar(&self,s:&u64)->bool{
    if !s.is_power_of_two()  {panic!("Not an element in hypergroupoid!")}
    let s=s.trailing_zeros();
    self.hyper_composition.row(s as usize).iter().all(|x|x.is_power_of_two())
}
pub fn collect_scalars(&self)->Vec<u64>{
    self.get_singleton().iter()
        .filter(|s|self.is_left_scalar(&s)&self.is_right_scalar(&s))
        .map(|x|*x)
        .collect::<Vec<u64>>()
}
pub fn collect_scalar_identity(&self)->Vec<u64>{
    self.collect_scalars()
        .par_iter()
        .filter(|s|self.is_identity(s))
        .map(|x|*x)
        .collect::<Vec<u64>>()
}
pub fn collect_ph(&self)->Vec<u64>{
    let  mut a: Vec<u64>  =self.get_singleton();
    loop {
        let ph=a;
        a=ph.iter().cartesian_product(ph.iter()).map(|(x,y)|self.mul_by_representation(x, y)).unique().collect();
        for x in self.get_singleton(){
            a.push(x);
        }
        a=a.iter().unique().map(|x|*x).sorted().collect();
        if a==ph {
            break ph;
        }


    }
}
pub fn beta_relation(&self)->Relation{
    let ph=self.collect_ph();
    let ph :Vec<Vec<u64>>= ph.iter().map(|x|ones_positions(x, &self.n)).collect();
    let ph: Vec<(u64, u64)> =ph.iter().zip(ph.iter())
        .map(|x|
            x.0.iter().cartesian_product(x.1.iter())
            .map(|(x,y)|(*x,*y))
            .collect_vec())
            .concat().iter()
            .unique()
            .map(|(x,y)|(2u64.pow(*x as u32),2u64.pow(*y as u32))).sorted().collect_vec();
        Relation{
            a:self.h.clone(),
            b:self.h.clone(),
            rel:ph
        }

}
/// Represents the integer $k$ as a subset of the set H={0,1,..,n-1}.
/// There are 2^n different integer representing subsets of H. It will panic if $k is greater then 2^n.
/// The subset S represented by k is given by the binary representation of k: 
/// i is in S if and only if the i-th digit of binary representation of k is a one.
/// Output is Vec<u64>. Use fn vec_to_set to convert it into a HashSet<u64>.
/// Reverse function is subset_as_u64() in utilities.rs
/// # Example
/// ```
/// use hyperstruc::hs::HyperGroupoidMat;
/// use std::collections::HashSet;
/// 
/// let cardinality = 4u64;
/// let hyperstructure = HyperGroupoidMat::new_random_from_cardinality(&cardinality);
/// let k=6;
/// let subset=hyperstructure.get_subset_from_k(&k);
/// println!("{:?}",subset);
/// let test_subset:HashSet<u64>= (1..=2).into_iter().collect();
/// assert_eq!(subset,test_subset);
/// 
/// let k=8;
/// let subset=hyperstructure.get_subset_from_k(&k);
/// println!("{:?}",subset);
/// let test_subset:HashSet<u64>= vec![3].into_iter().collect();
/// assert_eq!(subset,test_subset);
///
pub fn get_subset_from_k(&self,k:&u64)->HashSet<u64>{
    let cardinality = self.h.len() as u64;
    let subset: Vec<u64> = get_subset(&k, &cardinality);
    vec_to_set(&subset)
}
   pub fn mul_by_representation(&self,int_k:&u64,int_l:&u64)->u64{
    let ones_k=ones_positions(int_k, &(self.h.len() as u64));
    let ones_l= ones_positions(int_l, &(self.h.len() as u64));
    let mut indexes:Vec<(u64,u64)>=Vec::new();
    for a in &ones_k{
        for b in &ones_l{
                indexes.push((*a,*b));
        }
    }
    indexes.iter().fold(0u64, |acc, x| acc|self.hyper_composition[(x.0 as usize,x.1 as usize)])
}
/// # Example
/// ```
/// use hyperstruc::hs::HyperGroupoidMat;
/// use nalgebra::DMatrix;
/// use std::collections::HashSet;
/// let matrix=DMatrix::from_row_slice(3usize,3usize,&[1,2,7,2,7,7,7,7,5]);
/// let hyperstructure=HyperGroupoidMat::new_from_matrix(&matrix);
/// let a:HashSet<u64>=[1].into();
/// let b:HashSet<u64>=[0].into();
/// let ab=2u64;
/// let mul=hyperstructure.mul(&a,&b);
/// assert_eq!(ab,mul);
/// 
pub fn mul(&self,subset_k:&HashSet<u64>,subset_l:&HashSet<u64>)->u64{
    if !subset_k.is_subset(&self.h)||!subset_l.is_subset(&self.h) { panic!("K and L must be a subsets of H!")};
    let int_k=subset_as_u64(&subset_k);
    let int_l=subset_as_u64(&subset_l);
self.mul_by_representation(&int_k, &int_l)   
}
/// Compute b\a={x in H : a in bx}.
/// Input a and b must be type u64, representing non empty subset of H. Therefore, singleton are powers of 2.
/// 
/// /// # Example
/// ```
/// use hyperstruc::hs::HyperGroupoidMat;
/// use nalgebra::DMatrix;
/// use std::collections::HashSet;
/// let matrix=DMatrix::from_row_slice(3usize,3usize,&[1,2,7,2,7,7,7,7,5]);
/// let hyperstructure=HyperGroupoidMat::new_from_matrix(&matrix);
/// let a=2u64;
/// let b=4u64;
/// let ab=3u64;
/// let mul=hyperstructure.left_division(&a,&b);
/// assert_eq!(ab,mul);
pub fn left_division(&self,a:&u64,b:&u64)->u64{    
    self.get_singleton().iter()
    .filter(|x| 
        a&(&self.mul_by_representation(&b,&x))==*a
        )
        .fold(0, |acc, x| acc|x)
}
/// Compute a/b={x in H : a in xb}.
/// Input a and b must be type u64, representing non empty subset of H. Therefore, singleton are powers of 2.
/// 
/// /// # Example
/// ```
/// use hyperstruc::hs::HyperGroupoidMat;
/// use nalgebra::DMatrix;
/// use std::collections::HashSet;
/// let matrix=DMatrix::from_row_slice(3usize,3usize,&[1,2,7,2,7,7,7,7,5]);
/// let hyperstructure=HyperGroupoidMat::new_from_matrix(&matrix);
/// let a=1u64;
/// let b=2u64;
/// let ab=6u64;
/// let mul=hyperstructure.right_division(&a,&b);
/// assert_eq!(ab,mul);
/// 
pub fn right_division(&self,a:&u64,b:&u64)->u64{    
    self.get_singleton().iter()
        .filter(|x| 
            a&(&self.mul_by_representation(&x,&b))==*a
            )
            .fold(0, |acc, x| acc|x)
}

/// Return true if hyperstructure is reproductive, i.e., xH = H = Hx holds for all x in H.
/// # Example
/// ```
/// use hyperstruc::hs::HyperGroupoidMat;
/// use nalgebra::DMatrix;
/// let matrix=DMatrix::from_row_slice(3usize,3usize,&[1,2,7,2,7,7,7,7,5]);
/// let hyperstructure=HyperGroupoidMat::new_from_matrix(&matrix);
/// println!("{hyperstructure}");
/// assert!(hyperstructure.is_reproductive())
///
   pub fn is_reproductive(&self)->bool{
    let power_set = 2u64.pow(self.n as u32)-1;
    self.hyper_composition.row_iter().all(|x|x.iter().fold(0u64, |acc,element|acc|element)==power_set)
    &&
    self.hyper_composition.column_iter().all(|x|x.iter().fold(0u64, |acc,element|acc|element)==power_set)
   }
/// Return true if hyperstructure is associative, i.e., (xy)z = x(zy) holds for all x in H.
/// # Example
/// ```
/// use hyperstruc::hs::HyperGroupoidMat;
/// use nalgebra::DMatrix;
/// let matrix=DMatrix::from_row_slice(3usize,3usize,&[1,2,7,2,7,7,7,7,5]);
/// let hyperstructure=HyperGroupoidMat::new_from_matrix(&matrix);
/// println!("{hyperstructure}");
/// assert!(hyperstructure.assert_associativity())
///
pub fn assert_associativity(&self)->bool{
    for a in &self.get_singleton(){
        for b in &self.get_singleton(){
            for c in &self.get_singleton(){
                let ab_c=self.mul_by_representation(
                    &self.mul_by_representation(&a, &b),&c);
                let a_bc = self.mul_by_representation(&a, &self.mul_by_representation(&b, &c));
                assert_eq!(ab_c,a_bc,"{a}{b}_{c},{a}_{b}{c}")
            }
        }
    } 
true
}
/// Return true if hyperstructure is associative, i.e., (xy)z = x(zy) holds for all x in H.
/// # Example
/// ```
/// use hyperstruc::hs::HyperGroupoidMat;
/// use nalgebra::DMatrix;
/// let matrix=DMatrix::from_row_slice(3usize,3usize,&[1,2,7,2,7,7,7,7,5]);
/// let hyperstructure=HyperGroupoidMat::new_from_matrix(&matrix);
/// println!("{hyperstructure}");
/// assert!(hyperstructure.is_associative())
///
pub fn is_associative(&self)->bool{
 
for a in &self.get_singleton(){
        for b in &self.get_singleton(){
            for c in &self.get_singleton(){
                let ab_c=self.mul_by_representation(
                    &self.mul_by_representation(&a, &b),&c);
                let a_bc = self.mul_by_representation(&a, &self.mul_by_representation(&b, &c));
                if a_bc==ab_c {continue;} else {
                        return false;
                    }
            }
        }
    }
true
}
pub fn get_singleton(&self)->Vec<u64>{
    //DMatrix::from_row_iterator(1, self.n as usize, (0..self.n).into_iter().map(|i|2u64.pow(i)))
    (0..self.n).into_iter().map(|i|2u64.pow(i as u32)).collect()
}
/// Calculate the distance between two hyperstructures. The distance is defined as the the 
/// Hamming distance between binary representations of hyperstructure's tags, i.e., 
/// the number of positions at which the corresponding binary tags are different.
/// 
///
pub fn hamming_distance(&self,other:&HyperGroupoidMat)->usize {
    assert_eq!(self.n,other.n);
    (self.get_integer_tag()^other.get_integer_tag()).count_ones() as usize
}
pub fn hamming_distance_u1024(&self, other:&HyperGroupoidMat)->usize{
    assert_eq!(self.n, other.n);
    let dist:u32 = (self.get_integer_tag_u1024()^other.get_integer_tag_u1024()).to_little_endian().iter().map(|x|x.count_ones()).sum();
    dist as usize
}
}
impl Display for HyperGroupoidMat{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let table:DMatrix<String>=DMatrix::from_iterator(self.n as usize, self.n as usize, 
            self.hyper_composition.iter().map(|x|format!("{:?}",vec_to_set(&get_subset(x, &self.n)))));
        
        write!(f, "\nH: {:?},\nHypercomposition table:\n{} It is represented by: {}Size:{}\n", self.h, table, self.hyper_composition, self.n )
    }
}
pub fn get_random_hypercomposition_table(n:&u64)->Vec<Vec<u64>>{
    let vec: Vec<u64>=(0u64..*n as u64).into_iter().map(|x|x).collect();
    let index_cartesian=cartesian_product(&vec);
    let mut rng = rand::thread_rng();
    let mut hypercomposition_table=vec![vec![0u64;*n as usize];*n as usize];
    
    for item in index_cartesian {
        hypercomposition_table[item.0 as usize][item.1 as usize]=rng.gen_range(1..2u64.pow(*n as u32))
} 
hypercomposition_table
}
pub fn get_random_hypercomposition_matrix(n:&u64)->DMatrix<u64>{
    let mut rng = rand::thread_rng();
    let m  =DMatrix::from_iterator(*n as usize, *n as usize, (0..n.pow(2)).into_iter().map(|_|rng.gen_range(1..2u64.pow(*n as u32))));
    m
} 
pub fn distance_tags(tag1:&u128,tag2:&u128,cardinality:&u64)->u64 {
    let width = cardinality.pow(3);
    let binary_tag1 = n_to_binary_vec(tag1, &width);
    let binary_tag2 = n_to_binary_vec(tag2, &width);

    binary_tag1.iter().zip(binary_tag2).into_iter().filter(|(x,y)|*x!=y).count() as u64
}
pub fn distance_tags_u1024(tag1:&U1024,tag2:&U1024,cardinality:&u64)->usize{
    let width = cardinality.pow(3);
    let binary_tag1 = from_tag_u1024_to_vec(tag1, &width).concat();
    let binary_tag2 =from_tag_u1024_to_vec(tag2, &width).concat();

    binary_tag1.iter().zip(binary_tag2).into_iter().filter(|(x,y)|*x!=y).count()
}
///
/// Collects all binary strings that differ by d bits from the tag binary string.
/// The distance is intended to be the Hamming's distance. 
/// 
/// 
pub fn circumference_radius_d(tag:&U1024,d:&usize,cardinality:&u64)->Vec<U1024>{
    let width = cardinality.pow(3u32);
    let circunference:Vec<_> = (0..width).into_iter().combinations(*d)
    .into_iter()
    .map(|pos|
        pos
            .iter()
            .fold(*tag,
                |acc,x| acc^(U1024::from(U1024::one()<<*x))
            )).collect();

    circunference
}
///
/// Collects all binary strings that differ by d bits from the tag binary string and 
/// filter them to take only those representing hypergroups.
/// The distance is intended to be the Hamming's distance. 
/// 
/// 
pub fn circumference_radius_d_filtered(tag:&U1024,d:&usize,cardinality:&u64)->Vec<U1024>{
    circumference_radius_d(tag, d, cardinality).par_iter().filter(|x|representing_hypergroupoid_u1024(&x, cardinality)&&HyperGroupoidMat::new_from_tag_u1024(x, cardinality).is_hypergroup())
    .map(|x|*x).collect::<Vec<U1024>>()
}
///
/// Collects all binary strings that differ by 1 bits from the tag binary string and 
/// filter them to take only those representing hypergroups.
/// The distance is intended to be the Hamming's distance. 
/// 
/// 
pub fn hg_in_circumference_radius_one(tag:&U1024,cardinality:&u64)->Vec<U1024>{
    circumference_radius_d_filtered(tag, &1usize, cardinality)
}


