fn main() {
    // an example of a macro is the assert_eq! macro
    // this functionality could have been written as a generic function
    // but macros do more than functions can do, for example, when an assert_eq! macro fails
    // it prints the filename and the line number where the failure happened, fucntions have no way of getting that information

    // macros are expanded during the compilation phase before types are checked and well before
    // any machine code is generated, they are expanded recursively into rust code, since macros expansion
    // can themselves contain macros
    //
    // macros are differentiated from functions with the bang symbol before their parenthesis
    //
    // macro_rules! macro is the main way to define macros in Rust
    // when defining macros, the ! is not used, it is only used when calling them
    // a macro defined by macro_rules works entirely by pattern matching
    /*
     * (pattern 1) => (template 1)
     * (pattern 2) => (template 2)
     *
     */

    // you can use square brackets or curly braces for macros, they are still work
    println!("Hello");
    println!["Hello"]; // this is why vec! is possible
    println! {"Hello"};

    // by convention, () are used for assert_*! macros, [] for vec! and {} for macro_rules!
    // assert_eq! expands to
    // assert_eq![2, 1 + 1];

    /*
    * match (&2, &(1 + 1)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    */

    macro_rules! answer {
        () => {
            42
        };
    }

    macro_rules! double {
        ($x:expr) => {
            $x * 2
        };
    }

    let result = double!(answer!());
    println!("Result: {}", result);

    // Some interestig macros
    // file!() return a string path of the current file
    // line!() return the line which the first macro that call it was called from
    // column!() same as above, but for column
    // include!("filename.rs"), includes the content of the file in the current line, the content must be valid Rust code
    // include_str!("file.txt"), include the text content of the specified file
    // include_bytes!("file.dat") same as above, but the file is treated a binary data
    // todo!(), unimplemented!()
    // matches!(value, pattern)

    // Building the json! Macro
    use std::collections::HashMap;

    #[allow(dead_code)]
    #[derive(Clone, PartialEq, Debug)]
    enum Json {
        Null,
        Boolean(bool),
        Number(f64),
        String(String),
        Array(Vec<Json>),
        #[allow(clippy::box_collection)]
        Object(Box<HashMap<String, Json>>),
    }

    macro_rules! json {
          // the null literal must be matched directly
          (null) => { Json::Null };
          ([ $( $element:tt ),* ]) => { Json::Array(vec![ $( json!($element) ),* ]) };
          ({ $( $key:tt : $value:tt ),* }) => { Json::Object(Box::new(vec![
              $( ($key.to_string(), json!($value)) ),*
          ])) };
          ( $other:tt ) => {
              Json::from($other)
          }
      }

    impl From<bool> for Json {
        fn from(b: bool) -> Json {
            Json::Boolean(b)
        }
    }

    impl From<String> for Json {
        fn from(value: String) -> Json {
            Json::String(value)
        }
    }

    impl<'a> From<&'a str> for Json {
        fn from(value: &'a str) -> Json {
            Json::String(value.to_string())
        }
    }

    macro_rules! impl_from_num_for_json {
          ( $( $t:ident ),* ) => {
              $(
                impl From<$t> for Json {
                    fn from(n: $t) -> Json {
                        Json::Number(n as f64)
                    }
                }
              )*
          };
      }

    impl_from_num_for_json!(
        u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64
    );

    assert_eq!(json!(null), Json::Null);

    let macro_generated_array = json!([1, 2, 3, 4]);
    let hand_coded_array = Json::Array(vec![
        Json::Number(1.),
        Json::Number(2.),
        Json::Number(3.),
        Json::Number(4.),
    ]);

    assert_eq!(macro_generated_array, hand_coded_array);

    // Macros marked with #[macro_export] are automatically public to other modules
}
