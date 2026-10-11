/*

Tipos de dados compostos:

 - &str e String
 - Arrays
 - Vetores
 - Tuplas
 - Tupla vazia

*/

pub fn dados_compostos() {
    // Strings (&str e String)
    let _string_tamanho_fixo: &str = "Samuel"; // Tamanho fixo.
    let mut string_tamanho_flexivel: String = String::from("Alice"); // Tamanho flexível.
    string_tamanho_flexivel.push('s');

    // Arrays = Tamanho fixo definido, mas se for mut altera valores, mas nunca o tamanho.
    let array1: [i32; 5] = [1, 2, 3, 4, 5]; // Primeiro o tipo e depois o tamanho do array.
    let numero1 = array1[3]; // Acessando o valor de um array.
    println!("{}", numero1);

    let _array2 = [0; 10]; // Criação de Array sem inserir valores, ele automaticamente vai preencher com 0, e o tamanho do array é 10.

    // Vetores = Tamanho flexível, mas se for mut pode alterar valores e mudar o tamanho.
    let vetor1: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7]; // Para criar um vetor usamos o vec![] e se quisermos definir tipos de dados usamos Vec<tipo_de_dados>.
    let numero2 = vetor1[2];
    println!("{}", numero2);

    // Tuplas = Diferente de Arrays e Vetores que precisam ser do mesmo tipo de dados, Tuplas pode misturar inteiros, floats, Strings, etc. Também tem o tamanho fixo, como o Array.
    let minhas_informacoes: (&'static str, &'static str, &'static str, i8) = 
        ("Nome", "Samuel", "Idade", 20);

    let idade = minhas_informacoes.3; // Acessando uma tupla.
    println!("{}", idade);

    // Destruturação de Tuplas
    let (nome, nome_valor, idade, idade_valor) = minhas_informacoes;
    
    // Tupla vazia
    let unit: () = ();


}
