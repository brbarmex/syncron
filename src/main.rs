mod entities;
mod service;
mod adapter;

fn main()  {
    let url = "https://exemplo.com"; // Substitua pelo URL desejado

    let url = String::from("");
    match service::snapshot::fetch_data(&url) {
        Ok(resp) => {
            // Você tem a resposta da requisição aqui (por exemplo, resp.text() para obter o corpo da resposta)
            println!("Requisição bem-sucedida: {:?}", resp);
        }
        Err(err) => {
            eprintln!("Erro ao fazer a requisição: {:?}", err);
        }
    }
}
