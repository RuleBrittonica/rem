pub fn original_foo() {
    let p: &mut &i32 = &mut &0;
    let x = 1;
    *p = &x;
}
pub fn new_foo() {
    let p: &mut &i32 = &mut &0;
    let x = 1;
    bar_extracted(p, &x);
    println!("{}", **p);
}
fn bar_extracted<'lt0, 'lt1>(p: &mut &'lt0 i32, x: &'lt1 i32)
where
    'lt1: 'lt0,
{
    *p = &x;
}
fn main() {}
