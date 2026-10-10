/*

Tipos de dados primitivos
 - Integers
 - Floats
 - Char
 - Boolean
*/

pub fn data_types() {
    // Integers = números inteiros.
    // Por padrão o compilador usa o i32, quando não  definimos o tipo.

    // Unsigned Integers = números positivos.
    let unsigned_num: u8 = 5; // u8, u16, u32, u64, u128 

    // Signed Integers = números positivos e negativos.
    let signed_num: i8 = 10; // i8, i16, i32, i64, i128

    // Foats = números com ponto flutuante.
    // Por padrão o compilador usa o f64, quando não definimos o tipo.
    let float_num: f32 = 14.0; // f32, f64

    // Tipo de alocação para inteiros baseado na arquitetura. Ou seja, se o notebook for 32 bits o int vai ser 32 bits e assim por diante.
    let arch1: uisize = 5;
    let arch2: isize = 8;

    // Chars = caractere.
    let caractere: char = 'a'; // Letras, números, símbolos, emojis.

    // Boolean = true ou false.
    let b: bool = true;

    // Alias = um apelido para um tipo de dado.
    type Age = u8; // Age é do tipo u8.
    let idade_antonio: Age = 25; // No lugar de usar o u8, eu uso o Age.
    // Nesse caso o compilador não vai fazer a inferência de tipo.

    // Conversão de tipos de dados.
    let a: i32 = 10;
    let b = a as f64; // Converte o tipo de dado de i32 para f64.
}
