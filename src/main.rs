use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    println!(" ");
    println!("___Welcome to the number generator !!___");
    println!("Please select the number you want to generate:\n");
    println!("1. BL 014");
    println!("2. BL 019");
    println!("3. GP 013");
    println!("4. GP 017");
    println!("5. ROBI 018");
    println!("6. TELE 015");

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u8 = match input.trim().parse() {
        Ok(num) if (1..=6).contains(&num) => num,
        _ => {
            println!("Please enter a number between 1 and 5");
            return;
        }
    };

    match input {
        1 => bl_014(),
        2 => bl_019(),
        3 => gp_013(),
        4 => gp_017(),
        5 => robi_018(),
        6 => tele_015(),
        _ => unreachable!(),
    }
}

fn bl_014() {
    let file = File::create("bl_014.txt").expect("Unable to create file");
    let mut writer = BufWriter::new(file);

    for i in 0..=99999999 {
        let line = format!("014{:08}\n", i);
        writer
            .write_all(line.as_bytes())
            .expect("Unable to write data");
    }
}

fn bl_019() {
    let file = File::create("bl_019.txt").expect("Unable to create file");
    let mut writer = BufWriter::new(file);

    for i in 0..=99999999 {
        let line = format!("019{:08}\n", i);
        writer
            .write_all(line.as_bytes())
            .expect("Unable to write data");
    }
}

fn gp_013() {
    let file = File::create("gp_013.txt").expect("Unable to create file");
    let mut writer = BufWriter::new(file);

    for i in 0..=99999999 {
        let line = format!("013{:08}\n", i);
        writer
            .write_all(line.as_bytes())
            .expect("Unable to write data");
    }
}

fn gp_017() {
    let file = File::create("gp_017.txt").expect("Unable to create file");
    let mut writer = BufWriter::new(file);

    for i in 0..=99999999 {
        let line = format!("017{:08}\n", i);
        writer
            .write_all(line.as_bytes())
            .expect("Unable to write data");
    }
}

fn robi_018() {
    let file = File::create("robi_018.txt").expect("Unable to create file");
    let mut writer = BufWriter::new(file);

    for i in 0..=99999999 {
        let line = format!("018{:08}\n", i);
        writer
            .write_all(line.as_bytes())
            .expect("Unable to write data");
    }
}

fn tele_015() {
    let file = File::create("tele_015.txt").expect("Unable to create file");
    let mut writer = BufWriter::new(file);

    for i in 0..=99999999 {
        let line = format!("015{:08}\n", i);
        writer
            .write_all(line.as_bytes())
            .expect("Unable to write data");
    }
}
