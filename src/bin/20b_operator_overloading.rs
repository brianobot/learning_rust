use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    // you can make your own types support arithmetic and other operators 
    // by implementing a few built in traits, this process is called operator overloading
    
    // Operations    Operator
    // std::ops::Neg    -x
    // std::ops::Not    !x
    // std::ops::Add    x + y
    // std::ops::Sub    x - y
    // std::ops::Mul    x * y
    // std::ops::Div    x / y
    // std::ops::Rem    x % y
    // std::ops::BitAnd  x & y
    // std::ops::BitOr   x | y
    // std::ops::BitXor  x ^ y
    // std::ops::AddAssign x += y
    // std::ops::BitAddAssign  x &= y
    // std::ops::PartialEq   x == y, x != y
    // std::ops::Index    x[y], &x[y]
    // std::ops::IndexMut  x[y] = z, &mut x[y]
    
    
    // in rust the expression a + b is shorthand for a.add(b)
    // in other to call the trait methods you have to bring these trait into scope
    // that said, it's important to treat all arithmetic as function calls
    // 
    use std::ops::Add;
    
    assert_eq!(4.125f32.add(2f32), 6.125);
    assert_eq!(10.add(20), 10 + 20);
    
    // apart from the dereferencin operator
    // rust has 2 unary operators, -, and !
    // 
    #[allow(unused_imports)]
    use std  ::ops::*;
 
    #[derive(Debug, Clone, Copy)]
    struct Complex<T> {
        re: T,
        im: T
    }
    
    
    impl<  Neg for Complex<T> {
        type Output = Complex<T>;

        fn neg(self) -> Self::Output {
            Complex {
                re: -self.re,
                im: -self.im,
            }
        }
    }
    
    
    let complex_zero = Complex { re: 10i32, im: 0 };
    let neg_form_1 = -complex_zero.clone();
    let neg_form_2 = complex_zero.neg();
    
    println!("Neg Form 1: {:?}", neg_form_1);
    println!("Neg Form 2: {:?}", neg_form_2);
    
    
    
    Ok(())
}