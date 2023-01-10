fn main() {
    //declarando variavel - em rust variaveis sao imutaveis
    let mut name: &str = "Marcelo";
    name = "Alice";

    println!("Hello {}, Welcome!", name);
}
