use std::io::{self, BufRead, Read, Seek, SeekFrom, Write};

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
    // .read_exact(&mut buf): Reads exactly enough data to fill the buffer, if the reader runs out of data before the buffer length, an Error is returned
    //
    // Some other adapters methods include
    // .bytes(): returns an iterator over the input stream source, the Iterator Item type is io::Result<u8>, so an error check is carried out for each item
    // .chain(reader): return a new reader that combines the input stream from both readers
    // .take(n): return a new reader that reads from the same source of the value but is limited to n bytes on input

    let input_file = std::fs::File::open("hello.txt").unwrap();

    #[allow(clippy::unbuffered_bytes)]
    let byte_iter = input_file.bytes();
    println!("Byte Iter: {byte_iter:?}");

    for byte in byte_iter {
        println!("Byte: {}", byte.unwrap());
    }

    // Buffer Readers implement both Read and a second trait BufRead, which provides the following methods
    // .read_line(&mut line): reads a line of text and append it to line which is a String, the newline character is always included at the end
    // .lines(): this returns an iterator over the lines of the input
    // .read_until(stop_byte, &mut byte_vec): just like readline but you choose the stop_byte
    // .split(stop_byte)
    //

    #[allow(dead_code)]
    fn grep<R: BufRead>(target: &str, reader: R) -> io::Result<()> {
        for line_result in reader.lines() {
            let line = line_result.unwrap();
            if line.contains(target) {
                println!("{line}");
            }
        }

        Ok(())
    }

    // File in rust do not implemented the BufRead trait by default, they implement the Read trait
    // but you can create a BufReader from a File instance with the BufReader::new(file) method call
    //
    //
    // Writers: As we have seen so far, Input is mostly done with methods
    // to send output to a writer, we ues the write! or writeln! macro
    // the major difference is that the write! macro takes an extra first argument which is the writer
    // and they return a result, so errors must be handled
    //
    // Writer methods
    // .write(&buf): write values from the buffer into the underlying stream, returns Result<usize>, the usize might be less than buf.len()
    // .write_all(&buf) returns Result<()>
    // .flush(): flushes any buffered data to the underlying stream
    //
    // Readers and Writers are closed automatically when they are dropped
    // Just like BufReader::new(reader) adds a buffer to any reader BufWriter::new(writer) adds a buffer to a writer
    // when BufWriter is dropped all remaining buffered data is flushed to the underlying stream
    //
    // Opening a file with std::fs::File::open(path) opens the file in read only mode and returns a Result<File>
    // Opening a file with std::fs::File::create(path) creates in writing mode and return Result<File> too
    let _readonly_file = std::fs::File::create("hello.txt");
    let _writeonly_file = std::fs::File::create("hello.txt");

    // if neither of this use cases fit your need, you can use std::fs::OpenOptions to configure the mode of a file
    use std::fs::OpenOptions;

    let mut _log = OpenOptions::new()
        .append(true) // appends tot he file it it exist already
        .open("server.log")
        .unwrap();

    let _file = OpenOptions::new()
        .write(true)
        .create_new(true) // this fails if the file exist already
        .open("new_file.txt")
        .unwrap();

    // you can use the seek method of the Seek trait which is implemented for Files
    _log.seek(SeekFrom::Current(8)).unwrap();
    // seeking in a file is always slow
    //

    // Working with Files and Directories
    // OsStr is a super set of UTF-8 and can handle all filenames, environment variabes etc as provided by the operating system
    // Path is exactly like OsStr but with useful filename related methods
    // for each string type there is a corresponding heap owned type
    // std::ffi::OsStr has OsString
    // std::path::Path has PathBuf
    //
    // Path methods
    // .new() -> Create a new path from a &str or an &OsStr
    // .parent() -> return the parent of a path, return Option<&Path>
    // .file_name() -> return the last of a path, return Option<&OsStr>
    // .is_absolute(), .is_relative() -> return boolean indicating the cases
    // .join(path2) -> join 2 pathing a new PathBuf

    println!("Program End");
}
