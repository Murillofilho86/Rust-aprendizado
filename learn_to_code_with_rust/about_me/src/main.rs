fn main() {
    let mut array = [""; 4];
    array[0] = "A resposta fundamental";
    array[1] = "para a vida,";
    array[2] = "o universo";
    array[3] = "e tudo mais";

    for text in array.iter(){
        println!("{}", text);
    }
}
