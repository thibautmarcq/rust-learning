// Traffic light

#[derive(Debug)]
enum EtatFeu {
    Rouge,
    Orange,
    Vert,
    HorsService {raison : String},
}

fn etat_suivant(ef: EtatFeu) -> EtatFeu {
    match ef {
        EtatFeu::Rouge => EtatFeu::Vert,
        EtatFeu::Vert => EtatFeu::Orange,
        EtatFeu::Orange => EtatFeu::Rouge,
        EtatFeu::HorsService { raison } => EtatFeu::HorsService { raison },
    }
}

fn main() {
    let mut state = EtatFeu::Vert;
    println!("Etat de base : {:?}", state);

    for _ in 0..10 {
        std::io::stdin().read_line(&mut String::new()).unwrap();
        state = etat_suivant(state);
        println!("Etat suivant : {:?}", state);
    }

    state = EtatFeu::HorsService { raison: String::from("Canicule") };
    println!("Le feu est tombé en panne!");
    println!("Nouvel état : {:?}", state);
    println!("Etat suivant : {:?}", etat_suivant(state));
}

