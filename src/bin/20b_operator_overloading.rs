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
    // std::cmp::PartialEq   x == y, x != y
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
    use std::ops::*;

    #[derive(Debug, Clone, Copy)]
    struct Complex<T> {
        re: T,
        im: T,
    }

    impl<T: Neg<Output = T>> Neg for Complex<T> {
        type Output = Complex<T>;

        fn neg(self) -> Self::Output {
            Complex {
                re: -self.re,
                im: -self.im,
            }
        }
    }

    let complex_zero = Complex { re: 10i32, im: 0 };
    let neg_form_1 = -complex_zero;
    let neg_form_2 = complex_zero.neg();

    println!("Neg Form 1: {:?}", neg_form_1);
    println!("Neg Form 2: {:?}", neg_form_2);

    // to suport addassign for our complex type
    impl<T: AddAssign> AddAssign for Complex<T> {
        fn add_assign(&mut self, rhs: Self) {
            self.re += rhs.re;
            self.im += rhs.im;
        }
    }

    let mut cmplx = Complex { re: 10, im: 10 };
    let plx = cmplx;

    cmplx += plx;
    println!("Complex: {cmplx:?}");

    // unlike the arithemtic and bitwise operators
    // comparision operation takes the operand by references

    let is_greater = 3f32 >= 2f32;
    println!("Is Greater: {is_greater}");

    // index and IndexMut
    // a[i] == *a.index(i)
    let a = [1, 2, 3, 4];
    let _b = a[0];

    println!("A = {a:?}");
    // not all operators can be overloaded in python
    // & is always a reference
    // && || are limited to boolean values
    // .., ..= always create a range struct
    // = is the assignment operator, which copies or moves a value

    Ok(())
}
