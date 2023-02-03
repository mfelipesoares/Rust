fn main(){
    teste_while(1);
    teste_loop(1);
    teste_matchrange(1);
}

//While:
fn teste_while(i:i8){
    let mut j:i8 = 0;
    println!("While: ");
    while j < 10 {
        j += 1;
        println!("{}", i + j)
    }

}

//Loop:
fn teste_loop(mut i:i8){
    println!("Loop: ");
    loop{
        i += 1;
        println!("o valor de i é {}", i);
        if i == 10{
            break;
        }
    }
}

//Match:
fn teste_matchrange(a:i8){
    println!("{}, Match: ", a);
    let linguagem = "Assembly";
    let proposito = match linguagem{
        "JavaScript" => "NodeJs",
        "Assembly" => "NASM",
        _ => "Desconhecido"
    };
    println!("O proposito de {} é {}.", linguagem, proposito);
}