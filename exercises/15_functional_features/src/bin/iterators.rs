/* Al iterators implement a trait like this
pub trait Iterator {
    // Here item refers to an associated type, this is the return type of next
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
With this, the implementators only need to define the next method
 */

fn main() {
    // We can create iterators like
    let v1 = vec![1, 2, 3];
    // Iterators are lazy, and do nothing until it calls a method
    // iter() produce an iterator with inmutable references
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
    // to create an iterator which takes ownership and return owned values
    // we use into_iter

    // To use an iterator with mutable references, we use iter_mut

    // Beside next(), in iterator documentation we can check other method that consume
    // the iterator. Thus methods, which consume iterators are name *consuming adapters.
    // For example,
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);

    // Then we have the *iterator adapters* which don't consume the iterator, instead
    // produce a new one. For example, map with a clousure
    let v1_iter_adap = v1.iter().map(|x| x +1 );
    // Then we can consume the iterator and collect the values in a collection data type
    let v2: Vec<_> = v1_iter_adap.collect();
    assert_eq!(v2, vec![2, 3, 4]);

    /*Many iterator adapters take closures as arguments, and commonly the closures 
    we’ll specify as arguments to iterator adapters will be closures that capture 
    their environment. */

    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style:String,
    }

    fn shoe_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        // Take ownership from vector shoes, then the iterator take ownership
        // to finally the filter adapt iterator where the condition satisfy
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoe_in_size(shoes, 10);
    println!("{in_my_size:#?}");

}
