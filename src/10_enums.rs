fn main() {
    // enums give a way of saying that a value is one of a possible set of values

    // this example shows a data that can be either a V4 or a V6
    // as can be seen from the lack of semi-colon, the enum is an expression
    enum IPAddrKind {
        V4, 
        V6,
    }

    let four = IPAddrKind::V4;
    let six = IPAddrKind::V6;

    // the variants of an enum are namespaced under it's identifier, hence we use the double colon
    // both four and six are of the same type and any function that takes the enum
    // IPAddrKind would also take four and six

    fn route(ip_kind: IPAddrKind) {}

    route(four);
    route(six);
}