fn main() {
    //tupla com valores definidos
    let mut numbers = (1, 2, 3);

    //alterando valor da posição 0 da tupla
    numbers.0 = 50;
    //desestruturando valores da tupla
    let (a, b, c) = numbers;
    println!("{:?} {:?} {:?}", a, b, c);


    //declaração do array
    let array = [1.1, 2.8, 3.3];

    //operação slice "fatia" o array e eu defino qual o elemento inicial e o final do meu array
    println!("{:?}", &array[1..3]);
}
