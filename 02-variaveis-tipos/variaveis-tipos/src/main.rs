// Comentários de uma linha.

/*
Comentátios
Várias 
linhas
*/ 

// O compilador infere (mais nem sempre) qual tipo de dado a variável está usando. Normalmente a comunidade deixa ele inferir, somente quando chega em funções e métodos que o desenvolvedor deve informar o tipo.
fn main() {

    // Variaveis
    let mut x: i32 = 10; // Signed, Aceita positivo e negativo.
    let y: u32 = 8; // Unsigned, ou seja não aceita números negativos.

    let z: f64 = 3.1433; // Ponto flutuante.

    // String declarado dessa forma ele pode ser mutável.
    let name: String = String::from("Samuel");

    // String delcarado dessa forma ele é imutável.
    // let name: &str = "Samuel";

    // Se eu declarar uma variável e não usar, o compilador pede para usar o underscore (_). Pois variáveis não usadas são considerados uma má prática de programação. O underscore é usado para o compilador ignorar a variável.

    println!("X é {x}, Y é {y}, Z é {z}");

    // Mutabilidade.
    x = 11; 
    // Não permite que eu declare um novo valor a uma variável existente, então eu tenho que declarar como mutável (mut) para conseguir alter o valor da variável.

    println!("X é {x}");

    // Escopo é o limite que determina onde variável, função ou valor pode ser acessado ou usado.
    // O escopo é definido com abertura e fechamento de chaves {}.

    println!("Nome é: {name}");
    println!("X é {x}");


    {

        // Escopo que muda o nome para João.
        let name: &str = "João";
        let base_calculo = 2;
        println!("Nome é: {name}");
        x = 1000 * base_calculo;

    }

    // Vai quebrar pois foi definida apenas no escopo interno.
    //println!("Base cálculo é: {}", base_calculo);

    // Escopo da função main.
    println!("Nome é: {name}");
    println!("X é {x}");

    // A variável tem seu escopo de vida independente de onde é declarada, ou seja, a name na main é diferente da name do escopo interno.

    // Shadowing: Nova variável com o mesmo nome mas com valor diferente (Não usa o mesmo espaço da memória).
    let _idade = 37;
    // idade = 16; Não consigo mudar o valor por causa da imutabilidade.
    let idade = 16;

    println!("Idade é: {idade}");

    // Constantes
    // Escrita normalmente em maiúsculo para ser mais fácil de identificar e diferenciar de variáveis normais.
    const TENTATIVAS: i8 = 10;
    // Sempre dizer o tipo de dado da constante.
    println!("Tentativas: {TENTATIVAS}");

    // Char
    let primeira_leta: char = 'A';
    println!("Primeira letra é: {primeira_leta}");

}
