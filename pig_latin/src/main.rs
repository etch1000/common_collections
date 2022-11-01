fn pigit(wrd: &str) -> String {
    let mut chrs = wrd.chars();

    let fc = match chrs.next() {
        Some(c) => c,
        None => return String::new(),
    };

    match fc {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", wrd),
        _ => format!("{}-{}ay", chrs.as_str(), fc),
    }
}

fn main() {
    let vec_of_strs: Vec<String> = vec![
        String::from("first"),
        String::from("apple"),
        String::from("kraken"),
        String::from("ultimate"),
    ];

    for wrd in vec_of_strs {
        println!("{}", pigit(&wrd));
    }
}
