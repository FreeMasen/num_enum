

use ::num_enum::AsRefPrimitive;

// Guard against https://github.com/illicitonion/num_enum/issues/27
mod alloc {}
mod core {}
mod num_enum {}
mod std {}

#[derive(AsRefPrimitive)]
#[repr(u8)]
enum Enum {
    Zero,
    One,
    Two,
}

#[test]
fn simple() {
    let zero: &u8 = Enum::Zero.as_ref();
    assert_eq!(zero, &0u8);

    let one: &u8 = Enum::One.as_ref();
    assert_eq!(one, &1u8);

    let two: &u8 = Enum::Two.as_ref();
    assert_eq!(two, &2u8);
}

#[test]
fn hash_set() {
    use ::std::collections::HashSet;
    let hash_set = (0..3u8).filter(|v| v % 2 == 0).collect::<HashSet<_>>();
    let zero: &u8 = Enum::Zero.as_ref();
    assert!(hash_set.contains(zero));
    
    let one: &u8 = Enum::One.as_ref();
    assert!(!hash_set.contains(one));
    
    let two: &u8 = Enum::Two.as_ref();
    assert!(hash_set.contains(two));
}
