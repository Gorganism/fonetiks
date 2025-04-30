use std::io;
fn main() {
    println!("Input a string:");
    let mut english: String = String::new();
    // let _original = english.clone();
    io::stdin()
        .read_line(&mut english)
        .expect("Unable to read Stdin");
    println!("Enable Thornmode? [Y/n] ");
    let mut thornmode: String = String::new();
    io::stdin()
        .read_line(&mut thornmode)
        .expect("Unable to read Stdin");
    println!(" ");
    while ! thornmode.contains("Y") && ! thornmode.contains("y") && ! thornmode.contains("n") && ! thornmode.contains("N") {
        println!("That didn't work. Enable thornmode? [Y/n] ");
        let mut thornmode: String = String::new();
        io::stdin()
            .read_line(&mut thornmode)
            .expect("Unable to read Stdin");
        println!("{}",thornmode);
        if thornmode.contains("Y") || thornmode.contains("y") || thornmode.contains("n") || thornmode.contains("N") {
            break;
        }
    }
    if thornmode.contains("Y") || thornmode.contains("y") {
        println!("Thornmode is enabled. Letter 'Thorn' will be used.");
        english = english.replace("th","þ");
    } else if thornmode.contains("n") || thornmode.contains("N") {
        println!("Thornmode is disabled. Letter 'Eth' will be used.");
        english = english.replace("th","ð");
    } else {
        println!("Thornmode toggle could not be determined. The conversion will continue, but \"th\" substrings are not going to be affected.\n");
    }
    let english = english.replace("thom","tom");
    let english = english.replace("coope","cöpe");
    let english = english.replace("co-op","cöp");
    let english = english.replace("alk","ak");
    let english = english.replace("sh","ʃ");
    let english = english.replace("tio","ʃo");
    let english = english.replace("sio","ʃo");
    let english = english.replace("sure","ʃur");
    let english = english.replace("ll","l");
    // let english = english.replace("le","el");
    let english = english.replace("co","ko");
    let english = english.replace("cu","ku");
    let english = english.replace("ca","ka");
    let english = english.replace("ck","k");
    let english = english.replace("ic","ik");
    let english = english.replace("cr","kr");
    let english = english.replace("ci","si");
    let english = english.replace("ce","se");
    let english = english.replace("cy","sy");
    let english = english.replace("ch","c");
    let english = english.replace("ikh","ic");
    let english = english.replace("kn","gn");
    let english = english.replace("ec","ek");
    let english = english.replace("act","akt");
    let english = english.replace("cem","kem");
    let english = english.replace("whik","whic");
    let english = english.replace("nge","nje");
    let english = english.replace("ng","ŋ");
    let english = english.replace("nk","ŋk");
    let english = english.replace("ph","f");
    let english = english.replace("ause","auz");
    let english = english.replace("ause","auz");
    let english = english.replace("ouse","aus");
    let english = english.replace("cough","koff");
    let english = english.replace("laugh","laff");
    let english = english.replace("enough","enuf");
    let english = english.replace("tough","tuff");
    let english = english.replace("ough","o");
    let english = english.replace("gh","");
    let english = english.replace("exa","egza");
    let english = english.replace("exi","egzi");
    let english = english.replace("ax","aks");
    let english = english.replace("ox","oks");
    let english = english.replace("ux","uks");
    let english = english.replace("ix","iks");
    let english = english.replace("ex","eks");
    let english = english.replace("x","z");
    let english = english.replace("oo","u");
    let english = english.replace("þro","þru");
    let english = english.replace("þruw","þrow");
    let english = english.replace("of","ov");
    let english = english.replace("uld","ud");
    let english = english.replace("kss","ks");
    let english = english.replace("idk","idg");
    let english = english.replace("ture","cur");
    let english = english.replace("æcʃ","ækʃ");
    let english = english.replace("wið","wiþ");
    let english = english.replace("arsitekkur","arkitekcur");
    let english = english.replace("geo","jeo");
    let english = english.replace("rge","rje");
    println!("This is not a perfect system, and manual adjustments may be needed.\n\nYour output is:");
    println!("{}",english);
}
