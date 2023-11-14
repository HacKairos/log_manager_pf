use std::io;

fn main() {
    // Pede para o usuário digitar um nome
    println!("Digite um nome:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Falha ao ler a entrada do usuário.");
    // Remove quebras de linha e espaços em branco do nome
    name = name.trim().to_string();
    // Loop que itera 27 vezes
    for count in 0..27 {
        // Gera um novo nome a partir do nome atual
        let new_name = name.chars()
        // Mapeia cada caractere do nome atual para um novo caractere
        .map(|c| if c == 'a' { 'z' } else { (((c as u8) - b'a' + 25) % 26 + b'a') as char })
        // Junta os caracteres em uma string
        .collect::<String>();
        // Imprime o número da iteração e o novo nome gerado
        println!("{} - The new name is: {}", count, name);
        // Atribui o novo nome à variável 'name' para a próxima iteração
        name = new_name;
    }
}
