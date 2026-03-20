#![allow(dead_code, unused_imports, unused_variables)]

use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

mod fork_and_join_sample {
    use std::{collections::HashMap, sync::Arc};

    use super::*;

    pub type GigaMap = HashMap<String, String>;

    fn process_files(_filenames: Vec<String>, _glossary: &GigaMap) -> io::Result<()> {
        thread::sleep(Duration::from_secs(2));
        Ok(())
    }

    fn split_vec_into_chunks(filesnames: Vec<String>, n_threads: usize) -> Vec<Vec<String>> {
        filesnames
            .chunks(n_threads)
            .map(|chunk| chunk.to_vec())
            .collect()
    }

    pub fn process_files_in_parallel(
        filenames: Vec<String>,
        glossary: Arc<GigaMap>,
    ) -> io::Result<()> {
        const NTHREADS: usize = 8;
        let worklists = split_vec_into_chunks(filenames, NTHREADS);

        let mut thread_handles = Vec::with_capacity(NTHREADS);
        for (index, worklist) in worklists.into_iter().enumerate() {
            let glossary_child = glossary.clone();
            thread_handles.push(thread::spawn(move || {
                println!("Starting New Thread: {index}");
                process_files(worklist, &glossary_child)
            }))
        }

        for handle in thread_handles {
            // joining threads ensure that there is proper cleaning up after threads
            // since Rust programs ends when the main thread ends, joining
            // ensure this main thread waits for all the child threads to finish
            handle.join().unwrap()?;
        }

        Ok(())
    }
}

mod mandelbrot {
    use image::ColorType;
    use image::png::PNGEncoder;
    use std::fs::File;
    use std::str::FromStr;

    pub use num::Complex;

    fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
        let mut z = Complex::new(0.0, 0.0);
        for i in 0..limit {
            if z.norm_sqr() > 4.0 {
                return Some(i);
            }
            z = z * z + c;
        }

        None
    }

    fn parse_pair<T: FromStr>(s: &str, seperator: char) -> Option<(T, T)> {
        match s.find(seperator) {
            None => None,
            Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None,
            },
        }
    }

    #[allow(clippy::manual_map)]
    fn parse_complex(s: &str) -> Option<Complex<f64>> {
        match parse_pair(s, ',') {
            Some((re, im)) => Some(Complex { re, im }),
            None => None,
        }
    }

    fn pixel_to_point(
        bounds: (usize, usize),
        pixel: (usize, usize),
        upper_left: Complex<f64>,
        lower_right: Complex<f64>,
    ) -> Complex<f64> {
        let (width, height) = (
            lower_right.re - upper_left.re,
            upper_left.im - lower_right.im,
        );
        Complex {
            re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
            im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
        }
    }

    pub fn render(
        pixels: &mut [u8],
        bounds: (usize, usize),
        upper_left: Complex<f64>,
        lower_right: Complex<f64>,
    ) {
        assert!(pixels.len() == bounds.0 * bounds.1);
        for row in 0..bounds.1 {
            for column in 0..bounds.0 {
                let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);

                pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                    None => 0,
                    Some(count) => 255 - count as u8,
                };
            }
        }
    }

    pub fn write_image(
        filename: &str,
        pixels: &[u8],
        bounds: (usize, usize),
    ) -> Result<(), std::io::Error> {
        let output = File::create(filename)?;

        let encoder = PNGEncoder::new(output);
        #[allow(clippy::needless_borrow)]
        encoder.encode(
            &pixels,
            bounds.0 as u32,
            bounds.1 as u32,
            ColorType::Gray(8),
        )?;

        Ok(())
    }

    pub fn run_program() {
        let filename = "sample_plot.png";

        let bounds = (4000, 3000);
        let upper_left = Complex {
            re: -1.20,
            im: 0.35,
        };
        let lower_right = Complex { re: -1.0, im: 0.20 };

        let mut pixels = vec![0; bounds.0 * bounds.1];

        render(&mut pixels, bounds, upper_left, lower_right);
        write_image(filename, &pixels, bounds).expect("error writing PNG file");
    }
}

mod inverted_index {
    use byteorder::{LittleEndian, WriteBytesExt};
    use std::collections::HashMap;
    use std::fs;
    use std::io;
    use std::path::PathBuf;
    use std::sync::mpsc::{self, Receiver, Sender};
    use std::thread;
    use std::thread::JoinHandle;

    fn tokenize(text: &str) -> Vec<&str> {
        text.split(|ch: char| !ch.is_alphanumeric())
            .filter(|word| !word.is_empty())
            .collect()
    }

    type Hit = Vec<u8>;

    #[derive(Default)]
    pub struct InMemoryIndex {
        pub word_count: usize,
        pub map: HashMap<String, Vec<Hit>>,
    }

    impl InMemoryIndex {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn from_single_document(doc_id: usize, text: String) -> Self {
            let document_id = doc_id as u32;
            let mut index = InMemoryIndex::new();

            let text = text.to_lowercase();
            let tokens = tokenize(&text);

            for (i, token) in tokens.iter().enumerate() {
                let hits = index.map.entry(token.to_string()).or_insert_with(|| {
                    let mut hit = Vec::with_capacity(4 + 4);
                    hit.write_u32::<LittleEndian>(document_id).unwrap();
                    vec![hit]
                });
                hits[0].write_u32::<LittleEndian>(i as u32).unwrap();
                index.word_count += 1;
            }

            index
        }
    }

    fn read_files(sender: Sender<String>, documents: Vec<PathBuf>) -> io::Result<()> {
        for filename in documents {
            let text = fs::read_to_string(filename)?;
            println!("Text: {text}");

            if sender.send(text).is_err() {
                break;
            }
        }

        Ok(())
    }

    pub fn start_file_reader_thread(
        documents: Vec<PathBuf>,
    ) -> (Receiver<String>, JoinHandle<std::io::Result<()>>) {
        let (sender, receiver) = mpsc::channel();

        let handle = thread::spawn(move || {
            println!("About to Read Files");
            read_files(sender, documents)
        });

        (receiver, handle)
    }

    pub fn start_file_indexing_thread(
        texts: Receiver<String>,
    ) -> (Receiver<InMemoryIndex>, JoinHandle<io::Result<()>>) {
        let (sender, receiver) = mpsc::channel();

        let handle = thread::spawn(move || {
            for (doc_id, text) in texts.into_iter().enumerate() {
                let index = InMemoryIndex::from_single_document(doc_id, text);
                println!("Index: {:?}", index.map);

                if sender.send(index).is_err() {
                    break;
                }
            }

            Ok(())
        });

        (receiver, handle)
    }

    pub fn start_in_memory_merge_thread(
        file_indexes: Receiver<InMemoryIndex>,
    ) -> (Receiver<InMemoryIndex>, JoinHandle<io::Result<()>>) {
        let (sender, receiver) = mpsc::channel::<InMemoryIndex>();

        let handle = thread::spawn(move || Ok(()));

        (receiver, handle)
    }

    pub fn start_index_writer_thread(
        big_indexes: Receiver<InMemoryIndex>,
    ) -> (Receiver<InMemoryIndex>, JoinHandle<io::Result<()>>) {
        let (sender, receiver) = mpsc::channel::<InMemoryIndex>();

        let handle = thread::spawn(move || Ok(()));

        (receiver, handle)
    }

    pub fn merge_index_files(files: Receiver<InMemoryIndex>) -> io::Result<()> {
        let (sender, receiver) = mpsc::channel::<InMemoryIndex>();

        let handle: JoinHandle<io::Result<()>> = thread::spawn(move || Ok(()));

        Ok(())
    }

    pub fn run_pipeline(documents: Vec<PathBuf>) -> io::Result<()> {
        let (texts, h1) = start_file_reader_thread(documents);
        let (pints, h2) = start_file_indexing_thread(texts);
        let (gallons, h3) = start_in_memory_merge_thread(pints);
        let (files, h4) = start_index_writer_thread(gallons);
        let result = merge_index_files(files);

        let r1 = h1.join().unwrap();
        let r2 = h2.join().unwrap();
        let r3 = h3.join().unwrap();
        let r4 = h4.join().unwrap();

        // notice how the Error propagation is not handle until all the threads have been joine
        r1?;
        r4?;

        result
    }
}

mod mutex {
    // a mutex is used to force different threads to take turns when accessing certain data
    // Mutex<T> is a guard around the value T
}

fn main() -> io::Result<()> {
    // fork-join pattern of work with concurrency
    // let files = vec![0.to_string(); 1000];
    // fork_and_join_sample::process_files_in_parallel(files, Arc::new(fork_and_join_sample::GigaMap::new()))?;

    // use inverted_index::*;
    // let documents = vec![PathBuf::from("sample_file.txt")];
    // run_pipeline(documents).unwrap();

    // Essentially a Mutex (Mutual Exclusion) is used to guard access to a value and ensure that at any point in time
    // only one thread has access to that value
    //
    // RwLock is a little bit like a Mutex, but it allows mutiple shared Readers or one Writer just like Rust references
    // the read method on RwLock provides shared readonly access to a type
    // the wrote method me

    // CondVar: is a way to conditionally wait for a certain condition in a thread before execution
    //

    println!("Finished the Program ✅");
    Ok(())
}
