pub fn original_foo(){
    let p : &mut &i32 = &mut &0;
    {
        let x = 1;
        *p = &x;
    }
}

pub fn new_foo(){
    let p : &mut &i32 = &mut &0;
    {
        let x = 1;
        bar_extracted(p, &x);
        println!("{}", **p);
    }
}

fn bar_extracted<'lt0, 'lt1, 'lt2, 'lt3, 'lt4, 'lt5, 'lt6, 'lt7, 'lt8, 'lt9, 'lt10, 'lt11, 'lt12, 'lt13, 'lt14, 'lt15, 'lt16, 'lt17, 'lt18, 'lt19, 'lt20, 'lt21, 'lt22, 'lt23, 'lt24, 'lt25, 'lt26, 'lt27, 'lt28, 'lt29, 'lt30, 'lt31, 'lt32, 'lt33, 'lt34, 'lt35, 'lt36, 'lt37, 'lt38, 'lt39, 'lt40, 'lt41, 'lt42, 'lt43, 'lt44, 'lt45, 'lt46, 'lt47, 'lt48, 'lt49, 'lt50>(p: &'lt50 mut &'lt50  i32, x: &'lt0  i32)  {
    *p = &x;
}

fn main() {}






































































































































































































