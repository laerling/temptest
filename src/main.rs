struct Foo {
    a: i32,
    b: *const i32,
}

impl Foo {
    fn new(val: i32) -> Self {
	let mut foo = Foo {
	    a: val, b: std::ptr::null(),
	};
	println!("{:8X}", (&foo as *const i32) as usize);
	let self_ref: *const i32 = &foo.a;
	foo.b = self_ref;
	return foo
    }
}

fn id<T: Unpin>(n: T) -> T { n }

fn main() {
    let a = Foo::new(13);
    unsafe { println!("{}, {}", a.a, *a.b); }
    let b = Foo::new(24);
    unsafe { println!("{}, {}", b.a, *b.b); }
}
