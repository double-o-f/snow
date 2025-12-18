use std::{
    env,
    process,
    thread,
    io::Write,
    cell::Cell,
    time::{self, Duration},
};
use rand::Rng;
use terminal_size::{Width, Height, terminal_size};


thread_local!(
    static SNOW_STYLE: Cell<u8> = Cell::new(0);
    static SNOW_INTEN: Cell<u8> = Cell::new(2);
    static SNOW_SPEED: Cell<Duration> = Cell::new(time::Duration::from_millis(100));
    static SNOW_MELT:  Cell<u16> = Cell::new(64);
);


fn snow() {
    let mut flake_vec: Vec<(u16, u16)> = Vec::new();
    loop {
        let size = match terminal_size() {
            Some((Width(w), Height(h))) => (w, h),
            None => panic!("could not get terminal size")
        };

        for _ in 0..SNOW_INTEN.get() {
            flake_vec.push((rand::rng().random_range(1..=size.0), 1));
        }

        if SNOW_STYLE.get() == 0 {update_snow(&mut flake_vec, &size);}
        else if SNOW_STYLE.get() == 1 {update_snow_acc(&mut flake_vec, &size);}
        else if SNOW_STYLE.get() == 2 {update_snow_acc_melt(&mut flake_vec, &size);}

        std::io::stdout().flush().expect("how the fuck did you fail to flush");
        thread::sleep(SNOW_SPEED.get());
    }
}

fn update_snow(flake_vec: &mut Vec<(u16, u16)>, size: &(u16, u16)) {
    let mut i: usize = 0;
    while i < flake_vec.len() {
        if flake_vec[i].1 > 1 {
            print!("\x1b[{};{}f", flake_vec[i].1 - 1, flake_vec[i].0);
            print!(" ");
        }
        if flake_vec[i].1 <= size.1 {
            print!("\x1b[{};{}f", flake_vec[i].1, flake_vec[i].0);
            print!("*");
            flake_vec[i].1 += 1;
            i += 1;
        } else {
            flake_vec.remove(i);
        }
    }
}

fn update_snow_acc(flake_vec: &mut Vec<(u16, u16)>, size: &(u16, u16)) {
    let mut i: usize = 0;
    while i < flake_vec.len() {
        if flake_vec[i].1 <= size.1 {
            if flake_vec[i].1 > 1 {
                print!("\x1b[{};{}f", flake_vec[i].1 - 1, flake_vec[i].0);
                print!(" ");
            }
            print!("\x1b[{};{}f", flake_vec[i].1, flake_vec[i].0);
            print!("*");
            flake_vec[i].1 += 1;
            i += 1;
        } else {
            flake_vec.remove(i);
        }
    }

}

fn update_snow_acc_melt(flake_vec: &mut Vec<(u16, u16)>, size: &(u16, u16)) {
    let mut i: usize = 0;
    while i < flake_vec.len() {
        if flake_vec[i].1 > 1 {
            print!("\x1b[{};{}f", flake_vec[i].1 - 1, flake_vec[i].0);
            print!(" ");
        }
        if flake_vec[i].1 <= size.1 + SNOW_MELT.get() {
            print!("\x1b[{};{}f", flake_vec[i].1, flake_vec[i].0);
            print!("*");
            flake_vec[i].1 += 1;
            i += 1;
        } else {
            flake_vec.remove(i);
        }
    }
}

fn parse_arg(cur_arg: &mut u8, arg: &str) {
    if arg[0..=0] != *"-" || arg.len() < 2 { // if the first char in the arg is not "-" or the arg is only "-" exit
        eprintln!("invalid arg");
        process::exit(1);
    }
    let mut i: usize = 1;
    while i < arg.len() {
        if arg[i..=i] == *"c" {
            println!("\x1b[2J");
        }

        else if arg[i..=i] == *"a" {
            SNOW_STYLE.set(1);
        }
        else if arg[i..=i] == *"m" {
            SNOW_STYLE.set(2);
            if i + 1 == arg.len() {*cur_arg = 1; break;} // if current char is end of arg, check for num in next
            let num: &str = &arg[i + 1..];
            SNOW_MELT.set(num.parse().unwrap_or(SNOW_MELT.get())); // try to convert rest of arg to a number for -m
                           
        }
        else if arg[i..=i] == *"i" {
            if i + 1 == arg.len() {*cur_arg = 2; break;} // if current char is end of arg, check for number in next
            let num: &str = &arg[i + 1..];
            SNOW_INTEN.set(num.parse() // try to convert rest of arg to a number for -i
                           .unwrap_or_else(|_| {
                           eprintln!("invalid snow intensity");
                           process::exit(2);}));
        }
        else if arg[i..=i] == *"s" {
            if i + 1 == arg.len() {*cur_arg = 3; break;} // if current char is end of arg, check for num in next
            let num: &str = &arg[i + 1..];
            SNOW_SPEED.set(time::Duration::from_millis(num.parse() // try to convert rest of arg to a number for -s
                                                       .unwrap_or_else(|_| {
                                                       eprintln!("invalid snow speed");
                                                       process::exit(2);}))); // this is ass, i won't fix it
        }
        else {
            let arg: &str = &arg[i..=i];
            eprintln!("invalid arg {}", arg);
            process::exit(2);
        }
        i += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();


    // -c clear 0
    // -a acc 0  sty 1
    // -m [n] melt 1  sty 2
    // -i n intensity 2
    // -s n speed 3
    let mut cur_arg: u8 = 0;

    let mut i: usize = 1;
    while i < args.len() {
        match cur_arg {
            0 => parse_arg(&mut cur_arg, &args[i]), // parse arg if not currently looking for an args value
            1 => {
                SNOW_MELT.set(args[i].parse() // try to convert args[i] to a number for -m
                              .unwrap_or_else(|_| {
                              i -= 1;
                              SNOW_MELT.get()})); 
                SNOW_STYLE.set(2); // if can't, decrement and parse agian for arg
                cur_arg = 0;
            },
            2 => {
                SNOW_INTEN.set(args[i].parse() // try to convert args[i] to a number for -i
                               .unwrap_or_else(|_| {
                               eprintln!("invalid snow intensity");
                               process::exit(2);}));
                cur_arg = 0;
            },
            3 => {
                SNOW_SPEED.set(time::Duration::from_millis(args[i].parse() // try to convert rest of arg to a number for -s
                                                           .unwrap_or_else(|_| {
                                                           eprintln!("invalid snow speed");
                                                           process::exit(2);})));
                cur_arg = 0;
            },
            _ => panic!("cur_arg"),
        }
        i +=  1
    }


    if cur_arg > 1 {
        eprintln!("invalid arg");
        process::exit(2);
    }
    //print!("\x1b[1;31m"); //evil snow
    print!("\x1b[?25l");
    snow();
}
