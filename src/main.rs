use num::BigUint;
use clap::Parser;

#[derive(Parser,Default,Debug)]
#[clap(author="Written by Azu :)", version, about="A prime number de/coder")]
struct Arguments {
    /// Can be "code" or "decode", the type of use case.
    use_case: String,
    #[clap(default_value_t=String::from("en"),short, long)]
    /// Language, can either be hu or en
    language: String,
}


fn main() {
    let args = Arguments::parse();
    let de_or_code = args.use_case;
    let lang = args.language;

    let abc: Vec<&str> = " abcdefghijklmnopqrstuvwxyz".split_terminator("").skip(1).collect::<Vec<&str>>();
    macro_rules! code {
        ($message:expr) => {
            code(&mut String::from($message), &abc)
        };
        ($message:expr, print) => {
            println!("{}", code(&mut String::from($message), &abc))
        };
    }

    //////////////////////////////////////////////////////////////////////////////////////////////////
    // itt tudod mar hasznalni
    let english = String::from("en");

    if de_or_code == "code" {
        match lang == english {
            true => {
                println!("Message:");
                let coded = code!(""); //inputot ker majd lefutatassnal
                println!("Code: {coded}");
            }
            false => {
                println!("Üzenet:");
                let coded = code!(""); //inputot ker majd lefutatassnal
                println!("Kód: {coded}");
            }
        }
    }
    //let ans = decode(coded.clone(), abc.clone()); //vissza fejti
    else if de_or_code == "decode" {
        match lang == english {
        true => {
            println!("Code:");
            let mut code = String::new();
            std::io::stdin().read_line(&mut code).unwrap();
            let code = BigUint::parse_bytes(code.trim_end().as_bytes(), 10).expect("\n--------------------------------\nCould not parse input into BigUint. Make sure to only have the (base-ten) digits and the enter at the end!\n--------------------------------");
            let decoded = decode(code, abc);
            println!("List: {:?}", decoded);
            print!("Char: ");
            for char in decoded.iter() {print!("{}", char);}
            print!("\n");},

        false => {
            println!("Kód:");
            let mut code = String::new();
            std::io::stdin().read_line(&mut code).unwrap();
            let code = BigUint::parse_bytes(code.trim_end().as_bytes(), 10).expect("\n--------------------------------\nNem lehetet átvinni a megadott értéket BigUint-ba. Legyen biztos, hogy 10-es számrendszeri számokat és csak az enter-t a végén írta be!\n--------------------------------\n");
            let decoded = decode(code, abc);
            println!("Lista: {:?}", decoded);
            print!("Karakter: ");
            for char in decoded.iter() {print!("{}", char);}
            print!("\n");
            }
        }
    }
    else {
        match lang == english {
            true => println!("Wrong argument(s)... (try again)"),
            false => println!("Rossz argumentum(ok)... (probálja újra)"),
        }
    }
    match lang == english {
        true => println!("done!"),
        false => println!("kész!")
    }    
}


fn code(message: &mut String, abc: &Vec<&str>) -> BigUint {
    std::io::stdin().read_line(message).unwrap();
    
    let len: usize = abc.len();
    let coded = message
    .trim_end()
        .split_terminator("")
        .skip(1)
        .map(|x| abc
            .iter()
            .position(|&r| r == x)
            .expect("\n--------------------------------\nA character in your message could not be found in my character list. Make sure it is one of these: ' abcdefghijklmnopqrstuvwxyz',\nThis means no capital letters or punctuation, just the latin alphabet plus space\n--------------------------------")
        )
        .enumerate()
        .map(|(i, x)| x+(len*i));
        let max = coded.clone().last().unwrap();
        let primes = gen_primes(max);
        let coded: BigUint = coded.map(|x| primes[x]).fold(BigUint::from(1u128), |first, next| BigUint::from(first) * BigUint::from(next));
        coded
    }
    
fn decode(mut code: BigUint, abc: Vec<&str>) -> Vec<&str> {
        
    let len: usize = abc.len();
    
    let mut primes: Vec<usize> = vec![2];
    let mut n: usize = 1;
    let mut divisors: Vec<usize> = Vec::new();

    while code != BigUint::from(1u32) {
        n += 1;
        let s = f64::sqrt(n as f64) as usize + 1;

        let mut i = 0;
        while primes[i] < s {i += 1}

        let mut answer = primes[..i].iter()
        .filter(|x| n % *x == 0)
        .take(1)
        .peekable();
        if !(answer.peek().is_some()) {
            primes.push(n);
        }
        let d = primes.last().unwrap();
        let l = BigUint::from(*d);
        let answer = &code % &l == BigUint::from(0u32);
        if answer {
            code /= l;
            divisors.push(*d);
        }
        //println!("{:?}", divisors);
    }
    let letters = divisors
    .iter()
    .map(|x| primes
        .iter()
        .position(|&r| r == *x)
        .unwrap()
    )
    .enumerate()
    .map(|(i, x)| x-(len*i)-1)
    .map(|x| abc[x])
    .collect();
    
    return letters;
    
}


fn gen_primes(max: usize) -> Vec<usize> {
    let mut primes: Vec<usize> = vec![2];
    let mut n: usize = 2;
    while primes.len() <= max {
        n += 1;
        let s = f64::sqrt(n as f64) as usize + 1;

        let mut i = 0;
        while primes[i] < s {i += 1}

        let mut answer = primes[..i].iter()
        .filter(|x| n % *x == 0)
        .take(1)
        .peekable();
        if !(answer.peek().is_some()) {
            primes.push(n);
        }
    }
    primes
}
