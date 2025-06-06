use std::{collections::HashSet, ops::Mul};

use itertools::Itertools;

#[derive(Debug)]
pub struct Relation {
    pub a: HashSet<u64>,
    pub b: HashSet<u64>,
    pub rel: Vec<(u64,u64)>
}
impl <'a,'b>Mul<&'a Relation> for &'b Relation{
    type Output = Relation;
    fn mul(self, rhs: &'a Relation) -> Self::Output {
        assert_eq!(self.b,self.a,"product not define!");
        let pairs:Vec<(&u64, &u64)> =self.a.iter().cartesian_product(rhs.b.iter()).collect();
        let pairs=pairs.iter()
            .filter(|(x,z)|
                    self.b.iter()
                    .any(|y|self.rel.contains(&(**x,*y))&&rhs.rel.contains(&(*y,**z))))
                    .map(|p|(*p.0,*p.1)
                ).sorted().unique().collect();
    Relation{
        a:self.a.clone(),
        b:rhs.b.clone(),
        rel:pairs
    }
    }
}
impl Relation {
    pub fn is_reflexive(&self)->bool{
        assert_eq!(self.a,self.b,"Domain and codomain not coincede!");
        let pairs=self.a.iter().zip(self.b.clone())
            .into_iter()
            .map(|(x,y)|(2u64.pow(*x as u32),2u64.pow(y as u32)))
            .collect_vec();
        pairs.iter().all(|x|self.rel.contains(x))

    }
    pub fn is_symmetric(&self)->bool{
        assert_eq!(self.a,self.b,"Domain and codomain not coincede!");
        self.rel.iter().all(|(x,y)|self.rel.contains(&(*y,*x)))
  

    }
    pub fn is_transitive(&self)->bool{
        assert_eq!(self.a,self.b,"Domain and codomain not coincede!");
        let rr=(self*self).rel;
        
            rr.iter().all(|x|self.rel.contains(x))
    }
    pub fn is_equivalence(&self)->bool{
        self.is_reflexive()&&self.is_symmetric()&&self.is_transitive()
    }
   pub fn collect_classes(&self)->Vec<(u64,Vec<u64>)>{
    assert!(self.is_equivalence());
    let a:Vec<u64>= self.a.iter().sorted().map(|x|2u64.pow(*x as u32)).collect();
    let b:Vec<u64>= self.b.iter().sorted().map(|x|2u64.pow(*x as u32)).collect();

        let mut processed:Vec<u64>=Vec::new();
        let mut classes:Vec<(u64,Vec<u64>)>= Vec::new();
        
        for representant in &a {
            if processed.contains(representant){continue;}
            let mut class:Vec<u64>=Vec::new();
                for element in &b {
                    if self.rel.iter().contains(&(*representant,*element)){
                        class.push(*element);
                        processed.push(*element);
                    }
                
                }
                classes.push((*representant,class.clone()));
            }

        
        classes
    }
}