fn main() {
    println!("Início do programa");
    const X: i32 = 5;
    let y = 6;
    let mut z = 7;
    z = z + 1;

    println!("No início os valores são: X={X}, y={y}, z={z}");

    {
        // Quando o bloco finalizar, tudo o que está dentro dele deixa de existir e após a execução do bloco as variáveis fora do bloco que estavam escondidas ou "sombreadas" voltam a valer
        const X: i32 = 555;
        let y = 667;
        let mut z = 777;
        z = z + 1;

        println!("Dentro do bloco os valores são: X={X}, y={y}, z={z}");
    }

    println!("Depois do bloco os valores são: X={X}, y={y}, z={z}");
}
