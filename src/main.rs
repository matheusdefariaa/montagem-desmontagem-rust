use sys_mount::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("\n\tEntrada Inválida");
        ajuda();
    }

    else {
        let arg: &str = &args[1];
        
        if arg == "-m" {
            montagem();
        }

        else if arg == "-d" {
            desmontagem();
        }

        else if arg == "-h" {
            ajuda();
        }

        else {
            println!("Entrada Inválida");
            println!("Digite \"-h\" para ajuda");
        }
    }
}

fn montagem() {
    let _mount  = Mount::builder()
    .fstype("ext4")
    .mount("/dev/sda", "/mnt").unwrap();

    println!("Montagem realizada com sucesso");
}

fn desmontagem() {
    let _desmontagem = unmount("/mnt",UnmountFlags::DETACH);
    println!("Desmontagem realizada com sucesso");
}

fn ajuda() {
    println!("----------------------------------");
    println!("Autor: Matheus de Faria");
    println!("Versão: 0.1");
    println!("Data de lançamento: 30/01/2025\n");
    println!("\tMontagem e Desmontagem de disco\n");
    println!("Montagem:\n\t mnt -m");
    println!("Desmontagem:\n\tmnt -d");
    println!("----------------------------------");
}