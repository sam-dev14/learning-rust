/*

Tipos de dados primitivos:

 - Integers
 - Floats
 - Char
 - Boolean

*/

pub fn dados_primitivos() {
    // Integers = números inteiros.
    // Por padrão o compilador usa o i32, quando não  definimos o tipo.

    // Unsigned Integers = números não negativos (inclui zero e positivos).
    let _unsigned_num: u8 = 5; // u8, u16, u32, u64, u128 

    // Signed Integers = números positivos, negativos e zero.
    let _signed_num: i8 = 10; // i8, i16, i32, i64, i128

    // Floats = números com ponto flutuante.
    // Por padrão o compilador usa o f64, quando não definimos o tipo.
    let _float_num: f32 = 14.0; // f32, f64.

    // Tipo de alocação para inteiros baseado na arquitetura. Ou seja, se o notebook for 32 bits o int vai ser 32 bits e assim por diante.
    let _arch1: usize = 5;
    let _arch2: isize = 8;

    // Chars = caractere.
    let _caractere: char = 'a'; // Letras, números, símbolos, emojis.

    // Boolean = true ou false.
    let _b: bool = true;

    // Alias = um apelido para um tipo de dado.
    type Age = u8; // Age é do tipo u8.
    let _idade_antonio: Age = 25; // No lugar de usar o u8, eu uso o Age.
    // O compilador trata Age e u8 como a mesma coisa.

    // Conversão de tipos de dados.
    let _a: i32 = 10;
    let _b = _a as f64; // Converte o tipo de dado de i32 para f64.
}
