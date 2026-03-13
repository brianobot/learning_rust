use std::io::{self, Read, Write};

fn main() {
    // types that implement the Read traits are Readers and have method for byte oriented input
    // types that implement the BufRead traits are buffered readers and have methods for reading lines of texts and so forth
    // types that implement the Write traits are Writers and have methods for byte oriented output
    //
    // Readers are values that your program can read bytes values from
    // std::fs::File
    // std::net::TcpStreams
    // std::io::StdIn
    //
    // Writers are values that your progam can write byte values to
    // std::fs::File
    // std::net::TcpStreams
    // std::io::StdOut
    // Vec<u8>
    //
    // Personal Notes:
    // it is possible to write generic code that works for across the input and output channel
    // std::io::Read::read(&mut self, buf)
    fn generic_copy<R: Read, W: Write>(reader: &mut R, writer: &mut W) -> io::Result<u64> {
        let mut buf = [0; 8 * 1024];
        let mut written = 0;

        loop {
            let len = match reader.read(&mut buf) {
                Ok(0) => return Ok(written),
                Ok(len) => len,
                Err(e) => {
                    println!("✅ Error: {e:?}");
                    return Err(e);
                }
            };

            writer.write_all(&buf[..len])?;
            written += len as u64;
        }
    }

    let mut input_file = std::fs::File::open("hello.txt").unwrap();
    let mut output_file = std::fs::File::create("sample_file.txt").unwrap();

    println!("Input File: {input_file:?}");
    println!("Output File: {output_file:?}");

    let copied = generic_copy(&mut input_file, &mut output_file).unwrap();
    println!("Copied: {copied}");

    // Readers have several methods for reading data and they all take a mutabel reference to the reader itself
    // .read(&mut buff) reads some data from the data source and stores them in the buffer, the read method reads up until the len of buff
    // .read_to_end(&mut Vec<u8>): this reads all the data in the data source into the Vector
    // .read_to_string(&mut String): same as above but appends the data to the given String
    // ,read_exact(&mut buf): Reads exactly enough data to fill the buffer, if the reader runs out of data before the buffer length, an Error is returned
    //
}
