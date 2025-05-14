fn values_variables_pointers() {
    let x = 42; // bind x
    let y = 43; // bind y
    let _var1 = &x; // shared (non-exclusive) ref to x
    let mut _var2 = &x; // shared (non-exclusive) ref to x
    _var2 = &y; // shared ref to y
}

fn illegal_flows() {
    let mut x;
    // illegal ununitialized access
    // assert_eq!(x, 42);
    x = 42; // bind x
    let _y = &x; // shared (non-exclusive) ref to x
    x = 43; // illegal, as x's borrow scope continues below
    //assert_eq!(*y, 42); // end of scope for y
}

fn copy_move() {
    let x1 = 42; // bind x1
    let y1 = Box::new(84); // bind y1
    {
        let _z = (x1, y1); // copy x1, move y1
    }
    let _x2 = x1; // bind x2 (i32 implements the Copy trait)
    // let y2 = y1; // illegal (Box doesn't implement copy, so y1 was moved into z and dropped)
}

fn cache(input: &i32, sum: &mut i32) {
    *sum = *input + *input; // shared refs (&T) are immutable copies
    assert_eq!(*sum, 2 * *input); // mutable refs (&mut T) are exclusive, only 1 can exist
}

#[rustfmt::skip]
fn noalias(input: &i32, output: &mut i32) {
    if *input == 1 { // shared ref (immutable copy)
        *output = 2; // mutable ref (exclusive access to target value) collapses to:
    }                // if input == 1 {
    if *input != 1 { //     *output = 2;
        *output = 3; // else {
    }                //     * output = 3;
}

#[rustfmt::skip]
fn reftype() {
    let x = 42;          // x is of type i32
    let mut y = &x;      // y is of type &i32
    let mut _z = &mut y; // z if f type &mut &i32
}

#[rustfmt::skip]
fn replace_with_84(s: &mut Box<i32>) {
    // this is not okay, as *s would be empty:
    // let was = *s;
    // but this is:
    let was = std::mem::take(s);
    // so is this:
    *s = was;
    // we can exchange values behind &mut:
    let mut r = Box::new(84);
    std::mem::swap(s, &mut r);
    assert_ne!(*r, 84);
}

fn replace_demo() {
    let mut s = Box::new(42);
    replace_with_84(&mut s);
}

fn lifetimes() {
    use rand::Rng;
    let mut rng = rand::rng();
    let mut x = Box::new(42);
    let r = &x; // 'a
    if rng.random::<f32>() > 0.5 {
        *x = 84;
    } else {
        println!("{}", r); // 'a
    }
}

fn lifetime_holes() {
    let mut x = Box::new(42);
    let mut z = &x; // 'a
    for i in 0..100 {
        println!("{}", z); // 'a
        x = Box::new(i);
        z = &x; // 'a
    }
    println!("{}", z); // 'a
}
