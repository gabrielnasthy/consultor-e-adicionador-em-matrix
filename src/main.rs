use std::io;

fn consultar(matriz: &[[i32; 5]; 5]) {

    println!("Em qual linha quer ler ");
    let  x = ler_e_convertir();
    println!("Em qual coluna quer adiconar ");
    let  y = ler_e_convertir();

    println!();
    println!("{} ", matriz[x as usize][y as usize]);
}

fn adicionar(matriz: &mut [[i32; 5]; 5]) {

    println!("Em qual linha quer adiconar ");
    let x = ler_e_convertir();
    println!("Em qual coluna quer adiconar ");
    let  y = ler_e_convertir();

    println!("qual valor quer adicionar a posicao {},{} ",x,y);
        matriz[x as usize][y as usize] = ler_e_convertir();
    }

fn ler() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    input.trim().to_string()
}

fn ler_e_convertir() -> i32 {
    let input = ler();
    input.trim().parse::<i32>().expect("not a number")
}

fn main() {
    let mut matriz = [[0; 5]; 5];

    loop {
        println!("Qual operação quer executar?");
        println!("1 - Consultar posição da matriz");
        println!("2 - Adicionar valor à matriz");
        println!("3 - Sair");

        let opcao = ler_e_convertir();

        match opcao {
            1 => {
                consultar(&matriz);
            }
            2 => {
                adicionar(&mut matriz);
            }
            3 => break,
            _ => println!("Opção inválida"),
        }
    }
}
