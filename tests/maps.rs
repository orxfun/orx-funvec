use indexmap::IndexMap;
use orx_funvec::*;
use std::collections::{BTreeMap, HashMap};
use test_case::test_case;

#[test_case(HashMap::from_iter([(0, 'a'), (10, 'b'), (20, 'c')].into_iter()) ; "HashMap")]
#[test_case(BTreeMap::from_iter([(0, 'a'), (10, 'b'), (20, 'c')].into_iter()) ; "BTreeMap")]
#[test_case(IndexMap::from_iter([(0, 'a'), (10, 'b'), (20, 'c')].into_iter()) ; "IndexMap")]
fn at<F: FunVecD1Ref<char> + FunVecD1<char>>(map: F) {
    assert_eq!(Some(&'a'), map.ref_at(0));
    assert_eq!(Some(&'b'), map.ref_at(10));
    assert_eq!(Some(&'c'), map.ref_at(20));
    assert_eq!(None, map.ref_at(1));

    assert_eq!(Some('a'), map.val_at(0));
    assert_eq!(Some('b'), map.val_at(10));
    assert_eq!(Some('c'), map.val_at(20));
    assert_eq!(None, map.val_at(1));
}

#[test_case(HashMap::from_iter([(0, 'a'), (2, 'b'), (4, 'c')].into_iter()) ; "HashMap")]
#[test_case(BTreeMap::from_iter([(0, 'a'), (2, 'b'), (4, 'c')].into_iter()) ; "BTreeMap")]
#[test_case(IndexMap::from_iter([(0, 'a'), (2, 'b'), (4, 'c')].into_iter()) ; "IndexMap")]
fn iter<F: FunVecD1Ref<char> + FunVecD1<char>>(map: F) {
    let slice: Vec<_> = map.ref_iter_over(0..4).collect();
    assert_eq!(&slice, &[Some(&'a'), None, Some(&'b'), None]);
    let slice: Vec<_> = map.val_iter_over(0..4).collect();
    assert_eq!(&slice, &[Some('a'), None, Some('b'), None]);

    let joined = String::from_iter(map.ref_iter_over(0..100).flatten());
    assert_eq!("abc", &joined);
    let joined = String::from_iter(map.val_iter_over(0..100).flatten());
    assert_eq!("abc", &joined);
}
