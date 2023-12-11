use orx_closure::Capture;
use orx_funvec::*;
use std::collections::{BTreeMap, HashMap};

const N1: usize = 2;
const N2: usize = 3;

fn assert_at<V: FunVecD2<usize>>(vec: &V) {
    for i in 0..N1 {
        for j in 0..N2 {
            let val = i + j;
            assert_eq!(Some(val), vec.at(i, j));
        }
        assert_eq!(None, vec.at(i, N2));
    }
    assert_eq!(None, vec.at(N1, 0));
}
fn assert_iter_over<V: FunVecD2<usize>>(vec: &V) {
    let iter = (0..N1).flat_map(|i| (0..N2).map(move |j| (i, j)));
    let sum: usize = vec.iter_over(iter).flatten().sum();
    assert_eq!(9, sum);
}

#[test]
fn test_variants() {
    let vec = vec![vec![0, 1, 2], vec![1, 2, 3]];
    assert_at(&vec);

    let vec = vec![
        HashMap::from_iter([(0, 0), (1, 1), (2, 2)].into_iter()),
        HashMap::from_iter([(0, 1), (1, 2), (2, 3)].into_iter()),
    ];
    assert_at(&vec);
    assert_iter_over(&vec);

    let vec = BTreeMap::from_iter([(0, vec![0, 1, 2]), (1, vec![1, 2, 3])]);
    assert_at(&vec);
    assert_iter_over(&vec);

    let vec: Box<dyn Fn((usize, usize)) -> Option<usize>> =
        Box::new(|(i, j)| if i < N1 && j < N2 { Some(i + j) } else { None });
    assert_at(&vec);
    assert_iter_over(&vec);

    let vec = Capture(()).fun(|_, (i, j)| if i < N1 && j < N2 { Some(i + j) } else { None });
    assert_at(&vec);
    assert_iter_over(&vec);

    struct Limits {
        n1: usize,
        n2: usize,
    }
    impl Limits {
        fn get(&self, i: usize, j: usize) -> Option<usize> {
            if i < self.n1 && j < self.n2 {
                Some(i + j)
            } else {
                None
            }
        }
    }
    let limits = Limits { n1: N1, n2: N2 };
    let vec = Capture(limits).fun(|lim, (i, j)| lim.get(i, j));
    assert_at(&vec);
    assert_iter_over(&vec);
}
