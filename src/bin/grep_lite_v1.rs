
fn main() {
    let _search_term = "oo";

    let quote = "
        Every face, every shop, 
        bedroom window, public-house, and
        dark square is a picture 
        feverishly turned--in search of what?                
        It is the same with books. 
        What do we seek 
        through millions of pages?";
    
    let mut tags: Vec<usize> = Vec::new();
    let mut _ctx: Vec<Vec<(usize, String)>> = Vec::new();

    // PASS 1
    for (index, line) in quote.lines().enumerate() {
        println!("{}: {}", index + 1, line);

        if line.contains(_search_term) {
            tags.push(index);
        }
    }

    println!("Tags = {:?}", tags);
}