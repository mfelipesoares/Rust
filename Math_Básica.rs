//Função principal do Rust:
fn main(){
    tabuada(5);
    divisao(5.00);
    exponenciacao(2);
    logaritmico(2.00,2.00);
    radiciacao(1.00);
    
}

//Função que gera a tabuada de multiplicação do número informado:
fn tabuada(numero:i32){
    let mut contador:i32 = 0;
    println!("Multiplicação: ");
    while contador < 10 {
        contador += 1;
        println!("{} x {} = {}", numero, contador, numero * contador)
    }

}

//Função que gera tabuada de divisão do número informado:
fn divisao(numero:f32){
    let mut contador:f32 = 0.00;
    println!("Divisão: ");
    while contador < 10.00 {
        contador += 1.00;
        println!("{} / {} = {}", numero, contador, numero / contador)
    } 

}

//Função que gera os exponenciais do número informado:
fn exponenciacao(numero:i32){
    let mut contador:u32 = 0;
    println!("Exponenciação: ");
    while contador < 10{
        contador += 1;
        let expo = numero.pow(contador);
        println!("{} ** {} = {}", numero, contador, expo )

    }

}

//Função que gera o log do número e da base informada:
fn logaritmico(numero:f32, base:f32){
    let mut contador:f32 = 0.00;
    println!("Logaritmico: ");
    while contador < 10.00 {
        contador += 1.00;
        let i:f32 = base + contador;
        let loga:f32 = numero.log(i);
        println!("log{} {} = {}", i, numero, loga)
    }

}

//Função que gera a raiz do número informado:
fn radiciacao(mut  numero:f32){
    let mut contador:f32 = 0.00;
    println!("Radiciação: ");
    while contador < 10.00 {
        contador += 1.00;
        let raiz = numero.sqrt();
        println!("√{} = {}", numero, raiz);
        numero += 1.00
    }

}