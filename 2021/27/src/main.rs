mod foo;
mod bar;
mod mod1;

pub fn main() {
    foo::func1();
    bar::func2();
    mod1::mod2::mod3::baz::func3();
}
