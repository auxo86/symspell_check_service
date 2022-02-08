mod symspell_models;

use symspell::{AsciiStringStrategy, SymSpell, Verbosity};
use actix_web::{App, HttpResponse, HttpServer, post, Responder, web};
// use symspell_models::{QryString, SimilarWord};
use symspell_models::{QryString};

#[post("/word_check")]
async fn word_chk(dict_collection: web::Data<SymSpell<AsciiStringStrategy>>, qry_str: String) -> impl Responder {
    let instance_deserialized: QryString = serde_json::from_str(&qry_str.as_str()).unwrap();
    let symspell = dict_collection;
    let suggestions = symspell.lookup(instance_deserialized.str_orig.as_str(), Verbosity::Closest, 2);
    // let mut vec_suggestions_for_return: Vec<SimilarWord> = Vec::new();

    // for suggestion in &suggestions {
    //     vec_suggestions_for_return.push( SimilarWord {
    //         str_similar_word: suggestion.term.clone(),
    //         distance: suggestion.distance,
    //         count: suggestion.count,
    //     })
    // }

    // 把 Vec<Suggestions> 做序列化
    // let json_returned = serde_json::to_string(&vec_suggestions_for_return).unwrap();
    let json_returned = serde_json::to_string(&suggestions).unwrap();
    // 回傳序列化後的 String
    HttpResponse::Ok().body(json_returned)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut symspell: SymSpell<AsciiStringStrategy> = SymSpell::default();
    symspell.load_dictionary("data/frequency_dictionary_en_82_765.txt", 0, 1, " ");
    symspell.load_bigram_dictionary(
        "data/frequency_bigramdictionary_en_243_342.txt",
        0,
        2,
        " ",
    );

    let dict_collection = web::Data::new(symspell);

    HttpServer::new(move || {
        App::new()
            .app_data(dict_collection.clone())
            .service(word_chk)
    })
    .bind("127.0.0.1:20016")?
    .run()
    .await
}