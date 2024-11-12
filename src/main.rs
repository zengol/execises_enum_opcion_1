#[derive(Debug)]

enum Media {
    Book{title: String , autor : String},
    Movie{title:String , director: String},
    AudioBook{title: String},
    Serie{title: String},
   

}

// impl utilizando Pattern Matching with Enum. 

impl Media {
    fn description (&self) -> String {
          match self {
            Media::Book {title, autor} => {
                format!("Book: {} {}",title, autor)
            },
            Media::Movie {title,director} => {
                format!("Movie: {} {}", title, director)
            },
            Media::AudioBook {title} => {
                format!("AudioBook {} ", title)
            }
            Media::Serie {title} => {
                format!("Audiobook: {} ", title)
            }
        }
    } 

   
}
#[derive(Debug)]
struct Catalog {
    items: Vec<Media>
}

impl Catalog {
    //función que me sirve para (1) en main.
    fn new() ->Self {
        Catalog {items: vec![]}
    }
    //función que me sirve para (2) en main.
    fn add(&mut self, media: Media){
        self.items.push(media);
    }
}
 fn main() {
    //creamos un enum y asignamos un valor en AudioBook.
    let audiobook = Media::AudioBook { 
        title: String::from("An audiobook")
     };
     let book = Media::Book {
        title: String::from("Good Book"),
        autor: String::from("Good Autor"),
     };
     let movie = Media::Movie {
        title: String::from("Bad movie"),
        director: String::from("Bad director")
     };
     let serie= Media::Serie { title: String::from("Good serie") };
     /* 

     println!("{}", audiobook.description());
     println!("{}", book.description());
     println!("{}", movie.description());
     println!("{}", serie.description());
 */
     // (1) inicializa  la impl
     let mut catalog = Catalog::new();

     // (2) lo que le envie por argumento la fn add() lo va a agregar al vector self.items.
     catalog.add(audiobook);
     catalog.add(book);
     catalog.add(movie);
     catalog.add(serie);

     match catalog.items.get(1) {
         Option::Some(value) => {
            println!("Item: {:#?}", value)
         }
         Option::None => {
            println!("Nothing at that index")
         }
     }
 }
