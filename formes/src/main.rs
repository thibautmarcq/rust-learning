// Bibliothèque de formes géométriques

trait Forme {
    //Declarations
    fn aire(&self) -> f64;
    fn perimetre(&self) -> f64;

    //Definition
    fn resume(&self) -> String {
        format!("Forme d'aire={} et de périmètre={}", self.aire(), self.perimetre())
    }
}

struct Cercle { rayon: f64 }
struct Rectangle { largeur: f64, hauteur: f64 }
struct Carre { cote : f64 }

impl Forme for Cercle {
    fn aire(&self) -> f64 {
        std::f64::consts::PI*self.rayon.powi(2)
    }
    fn perimetre(&self) -> f64 {
        2.*std::f64::consts::PI*self.rayon
    }
}

impl Forme for Rectangle {
    fn aire(&self) -> f64 {
        self.largeur*self.hauteur
    }
    fn perimetre(&self) -> f64 {
        (self.largeur+self.hauteur)*2.
    }
}

impl Forme for Carre {
    fn aire(&self) -> f64 {
        self.cote*self.cote
    }
    fn perimetre(&self) -> f64 {
        4.*self.cote
    }
}

fn plus_grande_aire<T: Forme>(formes: &[T]) -> &T {
    formes.iter().max_by(|x,y| x.aire().total_cmp(&y.aire())).unwrap()
}

fn aire_totale(formes: &[Box<dyn Forme>]) -> f64 {
    let mut sum = 0.;
    for forme in formes {
        sum += forme.aire();
    }
    sum
}


fn main() {
    let mut vect: Vec<Box<dyn Forme>> = Vec::new();
    vect.push(Box::new(Carre { cote: 2.2 }));
    vect.push(Box::new(Rectangle {
        largeur: 3.0,
        hauteur: 4.0,
    }));
    vect.push(Box::new(Cercle{rayon : 4.7}));

    for forme in &vect {
        println!("{}",forme.resume());
    }

    let cercles = vec![
        Cercle { rayon: 1.0 },
        Cercle { rayon: 3.0 },
        Cercle { rayon: 2.0 },
    ];
    println!("Plus grande aire (cercles) : {}", plus_grande_aire(&cercles).resume());
    println!("Aire totale : {}", aire_totale(&vect));
}
