use std::io::Read;
use std::fs::File;
use std::io::{BufWriter, Write};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Args{
    #[clap(short, long)]
    url: String,
}

fn main() {
    let args = Args::parse();
    let client = reqwest::blocking::Client::new();
    let origin_url = args.url;
    let mut res = client.get(origin_url).send().unwrap();
    // println!("Status for {}: {}", origin_url, res.status());

    let mut body  = String::new();
    res.read_to_string(&mut body).unwrap();
    let mut detected = false;

    let mut chars: Vec<char> = body.chars().collect();
    let mut keep: Vec<bool> = vec![];

    for i in 0..chars.len(){
        if detected{
            if chars[i] != '>'{
                keep.push(false);
            } else {
                keep.push(false);
                detected = false;
            }
        } else {
            if chars[i] == '<'{
                detected = true;
                keep.push(false);
            } else {
                keep.push(true);
            }
        }
    }


    let mut k: [bool;500000] = [false;500000];
    for (i,c) in keep.into_iter().enumerate(){
        k[i] = c;
    }

    let mut iter = k.iter();
    chars.retain(|_| *iter.next().unwrap());

    let special = ['\n', '\t', '\r', '!', '.', '?', ',', '@', '[', ']', ')','(',';','_','=', '|'];
    for i in 0..special.len(){
        chars.retain(|&x| x != special[i]);
    }

    let mut result: Vec<String> = vec![];
    let mut s = String::new();
    for i in chars{
        if i != ' '{
            s.push(i);
        } else {
            result.push(s.clone());
            s.clear();
        }
    }

    result.retain(|x| x != "");

    // println!("{:?}", result);

    let f = File::create("./pwd.txt").expect("unable to create file");
    let mut f = BufWriter::new(f);
    for i in result{
        writeln!(f, "{}", i).expect("unable to write to file");
    }

}
