use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*, BufReader};

extern crate rand;
use rand::Rng;

struct Pessoa {
    nome: String,
    idade: i32,
}

trait Voz  {
    fn falar(&self);

    fn tem_voz(&self) -> bool;
}

impl Voz for Pessoa{
    fn falar(&self) {
        println!("Olá meu nome é {}", self.nome);
    }

    fn tem_voz(&self) -> bool {
        if self.idade > 1 {
            return true;
        }

        return false;
    }
}


fn leitor_arquivo(){

    let mut arquivo = 
    File::open("nx.txt")
    .expect("Nao conseguiu ler o arquivo");

    let mut conteudo = String::new();
    arquivo.read_to_string(&mut conteudo)
    .expect("Não conseguiu ler o arquivo e alocar na variavel conteudo");

    println!("Conteudo do arquivo: \n\n{}", conteudo);
}

fn escritor_arquivo(){

    let mut arquivo = File::create("teste.txt")
        .expect("Nao conseguiu criar o arquivo");

    arquivo.write_all(b"Arquivo de testes sendo criado")
        .expect("Não conseguiu escrever no arquivo");
}

fn read_and_write() {

    let file = File::open("dados.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap();
        println!("{}", l);
    }

    let mut file = OpenOptions::new()
        .append(true)
        .open("dados.txt")
        .unwrap();
    let new_content = "Nova linha de texto\n";
    file.write_all(new_content.as_bytes()).unwrap();

    let file = File::open("dados.txt").unwrap();
    let reader = BufReader::new(file);
    let num_lines = reader.lines().count();
    println!("Numero de linhas {}", num_lines);
}

fn speaker(){
    let pessoa1 = Pessoa {
        nome: String::from("Caique"),
        idade: 42
    };

    let pessoa2 = Pessoa {
        nome: String::from("Bianca"),
        idade: 0
    };

    println!("Pode {} falar ? {}", pessoa1.nome, pessoa1.tem_voz());
    println!("Pode {} falar ? {}", pessoa2.nome, pessoa2.tem_voz());
}

fn match_fn() {
    let numero = 3;

    //operações "| ou" ,  "1..10" 
    match numero {
        1 => println!("o numero é {}", numero),
        2..10 => println!("o numero é {}", numero),
        _ => println!("Eu não sei que numero é esse!")
    };

    let nome = "Caique";

    match nome {
        "Bianca" => println!("bianca é designer"),
        "Caique" => println!("Caique é programador"),
        _=> println!("Eu não sei sua profissão")
    };
}

fn match_input(){
    let mut mensagem_usuario = String::new();
    println!("Digite algo na box message: \n\n");

    match io::stdin().read_line(&mut mensagem_usuario) {
        Ok(_) => println!("Successo. voce digitou {}", mensagem_usuario.to_uppercase()),
        
        Err(e) => println!("Error na sessão ({})", e)
    }
}

fn slice_string(){
    let nome = "Hello";
    let nome2 = String::from("Hello world");
    let hello = &nome2[0..5];

    println!("{}", hello);
}

fn methods_string(){
    {
    let minha_string = String::from("Oi meu nome é caique");
    println!("{}", minha_string);
    println!("{}", minha_string.replace("caique", "Bianca"));
    }

    {
        let minha_string = String::from("Fui hoje\n ao mercado\n comprar arroz");

        for i in minha_string.lines() {
            println!("({})", i);
        }
    }

    {
        let minha_string = String::from("Minha+sogra+é+muito+feliz");
        let token: Vec<&str> = minha_string.split("+").collect();
        println!("{:?}", token);
        println!("{:?}", token[1]);
    }

    {
        let minha_string = String::from("           Meu nome é caique");
        println!("{}", minha_string);
        println!("{}", minha_string.trim());
        //corta todos os espaços
    }

    {
        let minha_string = String::from("Deixa uma avaliação de 5 estrelas");
        
        match minha_string.chars().nth(6) {
            Some(c) => println!("{}", c),

            None => println!("não existe o carcater na 6 posição")
        }
    }
}

fn random_number(){
    {
    let valores_randomicos = rand::thread_rng().gen_range(5, 11);
    println!("{}", valores_randomicos);
    }
    {
    let valores_randomicos_flutuante = rand::thread_rng().gen_range(1., 100.);
    println!("{}", valores_randomicos_flutuante);
    }
    {
        let valores_randomicos = rand::thread_rng().gen_weighted_bool(1); //quanto maior o numero que eu colocar menor a chance de receber um true
        println!("{}", valores_randomicos);
    }
}

fn main() {
    println!("-------------------------------------");
    leitor_arquivo();
    println!("-------------------------------------");
    escritor_arquivo();
    println!("-------------------------------------");
    read_and_write();
    println!("-------------------------------------");
    speaker();
    println!("-------------------------------------");
    match_fn();
    println!("-------------------------------------");
    //match_input();
    println!("-------------------------------------");
    slice_string();
    println!("-------------------------------------");
    methods_string();
    println!("-------------------------------------");
    random_number();
}
