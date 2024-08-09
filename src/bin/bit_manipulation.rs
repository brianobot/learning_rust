fn main() {
    // while learning rust, i have discovered that i have plenty of knowledge debt around
    // bit manipulation, i have not had to manipulate bit on a any of my major projects
    // mostly because i worked predominately with higher level dynamic languages like python
    // this section is not entirely rust, but is approaced with rust as the tool for the job
    // i hope it helps someone out there

    // guess what, it is actually less used and it is mostly important
    // for people that intent on actually manipulating data on the bit level, 
    // video compression and encryption algorithm use these heavily... 

    // NOTE; they are not the most intuitive of things/
    // but here is why we need them

    // the smallest amount of space a variable can take up is one byte
    // because it is the smallest unit of addressable space a cpu can reference, even boolean variables 
    // which should only take up bit takes up 8 bit (1 byte) with the rest of the bit padding off with zeros
    // this can be a serious issues when memory space is a limitation 

    // these can be addressed by storing different values on different bit of a byte
    // when printing binary bit in rust and you need to pad the leading bit with zeros
    // use the {:#10b} this adds 10 leading zero with the first 2 being 0b so effecitively
    // this would hold 8 bits of binary data

    // there are 6 bitwise operators that you need to know
    /*
    AND Operator (&)
        matches when the two bit in comparison match as one
        1 & 1 = 1
        1 & 0 = 0
        0 & 0 = 0
    
    OR Operator (|)
        matches when atleast one of the bit in comparison is one
        1 | 1 = 1
        1 | 0 = 1
        0 | 0 = 0

        uses: can be used to 

    XOR Operator (^)
        matches when each of the bit are an opposite of each other
        1 ^ 1 = 0
        1 ^ 0 = 1
        0 ^ 0 = 0 

        uses: can be used to remove some bits from a data
        101 ^ 000 = 101 (removes 000 from 101)
        110 ^ 101 = 011 (removes 101 from 110)

    NOT Operator (~)
        this operates on a single bit and converts it to the opposite
        ~1 = 0
        ~0 = 1
    
    LEFT SHIFT (<<)
        just like the NOT operator the shift operators only work on a single bit
        and the second number denotes how many bits you want to shift by, the leftmost
        bits are discarded in this case
        0b0001 << 1 = 0b0010

    RIGHT SHIFT (>>)
        same as left shift
        0b0010 >> 1 = 0b0001
        

    
     */
}