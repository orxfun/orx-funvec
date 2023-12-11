use orx_funvec::*;

#[test]
fn at() {
    let vec = vec![1, 2, 3];

    assert_eq!(vec.at(0), Some(1));
    assert_eq!(vec.ref_at(1), Some(&2));
    assert_eq!(vec.at(4), None);
    assert_eq!(vec.ref_at(4), None);
}

#[test]
fn iter() {
    let vec = vec![1, 2, 3];

    assert_eq!(vec.iter_over(2..4).collect::<Vec<_>>(), vec![Some(3), None]);
    assert_eq!(
        vec.ref_iter_over(2..4).collect::<Vec<_>>(),
        vec![Some(&3), None]
    );

    assert_eq!(vec.iter_over(0..2).flatten().sum::<i32>(), 3);
    assert_eq!(vec.ref_iter_over(0..2).flatten().sum::<i32>(), 3);
}
