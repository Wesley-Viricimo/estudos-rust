fn main() {
    // Tupla é um agregado de valores que podem ser de tipos diferentes
    
    let tup1: (i32, f64, bool) = (500, 6.4, true); // Tupla com os tipos de valores definidos
    let tup2 = (501, 6.3, true); // Tupla sem os tipos de valores definidos, neste caso o compilador tipa os valores de forma automática

    println!("Minha tuple tem: {:?}", tup2);

    //Desestruturação (destruturing) quebrando a tupla em suas partes e atribuindo os valores para as variáveis
    let (x1, y1, z1) = tup2;
    println!("Valores da tupla com desestruturação: {x1}, {y1}, {z1}");

    // Acessando os campos da tupla utilizando indexadores
    println!("Acessando os valores das posições da tupla: {:?} {:?} {:?}", tup1.0, tup1.1, tup1.2);


}
