use crate::guess::Guess;

#[test]
fn guess_correct(){
    let g = Guess::new(50);
    assert!(g == Guess::new(50));
    assert_eq!(g, Guess::new(50));

    assert_eq!(g, g);
}

#[test]
fn guess_incorrect() {
    let g = Guess::new(50);
    assert!(g != Guess::new(25));
    assert_ne!(g, Guess::new(25));
}

#[test]
fn guess_lt() {
    let g1 = Guess::new(30);
    let g2 = Guess::new(60);
    assert!(g1<g2);
    assert!(g1.value()<g2.value());
    assert!(g2>g1);
    assert!(g2.value()>g1.value());

    assert!(g1 != g2);
}

#[test]
#[should_panic]
fn wrong_data() {
    Guess::new(3040);
}

#[test]
#[should_panic]
fn too_small() {
    Guess::new(-1);
}
