pub mod period;

pub fn convert_to_vec<T, K>(collection: Vec<T>, f: impl Fn(T) -> K) -> Vec<K> {
    collection.into_iter().map(|elem| f(elem)).collect()
}
