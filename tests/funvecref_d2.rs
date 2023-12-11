use orx_funvec::*;
use std::collections::{BTreeMap, HashMap};

const N1: usize = 2;
const N2: usize = 3;

fn assert_at<V: FunVecRefD2<usize>>(vec: &V) {
    for i in 0..N1 {
        for j in 0..N2 {
            let val = i + j;
            assert_eq!(Some(&val), vec.ref_at(i, j));
        }
        assert_eq!(None, vec.ref_at(i, N2));
    }
    assert_eq!(None, vec.ref_at(N1, 0));
}
fn assert_iter_over<V: FunVecRefD2<usize>>(vec: &V) {
    let iter = (0..N1).flat_map(|i| (0..N2).map(move |j| (i, j)));
    let sum: usize = vec.ref_iter_over(iter).flatten().sum();
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

    #[cfg(any(feature = "impl_all", feature = "impl_indexmap"))]
    {
        use indexmap::IndexMap;
        use orx_closure::Capture;

        struct Repo {
            storage: IndexMap<(usize, usize), usize>,
        }
        impl Repo {
            fn new() -> Self {
                Self {
                    storage: IndexMap::from_iter(
                        [
                            ((0, 0), 0),
                            ((0, 1), 1),
                            ((0, 2), 2),
                            ((1, 0), 1),
                            ((1, 1), 2),
                            ((1, 2), 3),
                        ]
                        .into_iter(),
                    ),
                }
            }
            fn get(&self, i: usize, j: usize) -> Option<&usize> {
                self.storage.get(&(i, j))
            }
        }
        let repo = Repo::new();
        let vec = Capture(repo).fun_option_ref(|r, (i, j)| r.get(i, j));
        assert_at(&vec);
        assert_iter_over(&vec);
    }
}
