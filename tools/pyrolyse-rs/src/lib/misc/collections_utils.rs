use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use itertools::Itertools;

pub fn flatten_vec_vec<T: Clone>(v_v: &[Vec<T>]) -> Vec<T> {
    let mut v = vec![];
    for v_to_append in v_v.iter() {
        v.extend_from_slice(v_to_append)
    }
    v
}

pub fn group_iter_to_hm<
    I: IntoIterator<Item = T>,
    T: Hash + Clone + Eq + Debug,
    K: Hash + Ord + Debug + Clone,
    V,
>(
    i: I,
    f: &dyn Fn(&T) -> K,
    g: &dyn Fn(&T) -> V,
) -> HashMap<K, Vec<V>> {
    let mut t_v: Vec<T> = i.into_iter().collect::<Vec<T>>();
    t_v.sort_by_key(f);

    let key_t_hm = t_v
        .iter()
        //.group_by(|t| f(*t))
        .chunk_by(|t| f(*t))
        .into_iter()
        .map(|(key, group)| {
            let v = group.cloned().collect();
            (key, v)
        })
        .collect::<HashMap<K, Vec<T>>>();

    let length = key_t_hm.iter().fold(0, |l, tuple| l + tuple.1.len());
    assert!(t_v.len() == length);

    key_t_hm
        .iter()
        .map(|(k, v)| {
            let v = v.iter().map(g).collect::<Vec<V>>();
            ((*k).clone(), v)
        })
        .collect::<HashMap<K, Vec<V>>>()
}
