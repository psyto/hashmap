struct Bucket<K, V> {
    items: Vec<(K, V)>,
}

pub struct HashMap<K, V> {
    bucket: Vec<Bucket<K, V>>,
}