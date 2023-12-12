use orx_funvec::*;

fn at<F: FunVecRefD1<char> + FunVecD1<char>>(map: F) {
    assert_eq!(Some(&'a'), map.ref_at(0));
    assert_eq!(Some(&'b'), map.ref_at(10));
    assert_eq!(Some(&'c'), map.ref_at(20));
    assert_eq!(None, map.ref_at(1));

    assert_eq!(Some('a'), map.at(0));
    assert_eq!(Some('b'), map.at(10));
    assert_eq!(Some('c'), map.at(20));
    assert_eq!(None, map.at(1));
}

fn iter<F: FunVecRefD1<char> + FunVecD1<char>>(map: F) {
    let slice: Vec<_> = map.ref_iter_over(0..4).collect();
    assert_eq!(&slice, &[Some(&'a'), None, Some(&'b'), None]);
    let slice: Vec<_> = map.iter_over(0..4).collect();
    assert_eq!(&slice, &[Some('a'), None, Some('b'), None]);

    let joined = String::from_iter(map.ref_iter_over(0..100).flatten());
    assert_eq!("abc", &joined);
    let joined = String::from_iter(map.iter_over(0..100).flatten());
    assert_eq!("abc", &joined);
}

#[test]
fn hashmap_at_iter() {
    use std::collections::HashMap;

    at(HashMap::from_iter(
        [(0, 'a'), (10, 'b'), (20, 'c')].into_iter(),
    ));

    iter(HashMap::from_iter(
        [(0, 'a'), (2, 'b'), (4, 'c')].into_iter(),
    ));
}

#[test]
fn btreemap_at_iter() {
    use std::collections::BTreeMap;

    at(BTreeMap::from_iter(
        [(0, 'a'), (10, 'b'), (20, 'c')].into_iter(),
    ));

    iter(BTreeMap::from_iter(
        [(0, 'a'), (2, 'b'), (4, 'c')].into_iter(),
    ));
}

#[cfg(any(feature = "impl_all", feature = "impl_indexmap"))]
#[test]
fn indexmap_at_iter() {
    use indexmap::IndexMap;

    at(IndexMap::from_iter(
        [(0, 'a'), (10, 'b'), (20, 'c')].into_iter(),
    ));

    iter(IndexMap::from_iter(
        [(0, 'a'), (2, 'b'), (4, 'c')].into_iter(),
    ));
}
