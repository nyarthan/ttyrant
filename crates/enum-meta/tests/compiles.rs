use enum_meta::EnumMeta;

#[derive(EnumMeta)]
#[meta_attrs(a, b)]
enum Foo {
    #[meta(a = "a", b = "b")]
    Bar,
}

fn main() {
    let _ = Foo::Bar;
}
