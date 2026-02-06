#![allow(dead_code)]

fn main() {
    // usually you would find them used above the line which they are referring to
    // funny how i started using comments since the first lesson, lol, Bye

    /*
    these are multiline comments.. now bye for real
     */

    // we also have doc comments, that are use in the auto generated docs for rust codes
    /// Line comments; document the next item
    /** Block comments; document the next item */
    struct SomeStruct;

    mod module {
        //! Line comments; document the enclosing item
        /*! Block comments; document the enclosing item !*/
    }
}
