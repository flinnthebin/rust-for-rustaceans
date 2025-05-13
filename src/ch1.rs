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
    let y = &x; // shared (non-exclusive) ref to x
    x = 43; // illegal, as x's borrow scope continues below
    //assert_eq!(*y, 42); // end of scope for y
}

fn copy_move() {
    let x1 = 42; // bind x1
    let y1 = Box::new(84); // bind y1
    {
        let z = (x1, y1); // copy x1, move y1
    }
    let x2 = x1; // bind x2 (i32 implements the Copy trait)
    // let y2 = y1; // illegal (Box doesn't implement copy, so y1 was moved into z and dropped
}
