use enum_meta::EnumMeta;

#[derive(EnumMeta)]
#[meta_attrs(a, b)]
enum Foo {
    #[meta(a = "a", b = "b")]
    Bar,
}

fn main() {
    let foo = Foo::Bar;
    let a = foo.a();
    let b = foo.b();

    assert_eq!(a, "a");
    assert_eq!(b, "b");
}
