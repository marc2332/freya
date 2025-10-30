use pathmap::PathMap;

fn main() {
    let mut map = PathMap::new();

    map.insert(&[0], 1);
    map.insert(&[1], 2);
    map.insert(&[2], 3);

    map.insert(&[0, 0], 4);
    map.insert(&[0, 1], 5);
    map.insert(&[0, 1, 0], 6);
    map.insert(&[0, 1, 0], 7);
    map.remove(&[0, 1, 0]);
    map.insert(&[0, 1, 0], 7);

    println!("{map:#?}");
}
