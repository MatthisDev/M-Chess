use chess_back::components::board::Board;
use std::io;
    
fn main() {
    let text_polychess_2 = [
        "██████╗  ██████╗ ██╗  ██╗   ██╗ ██████╗██╗  ██╗███████╗███████╗███████╗",
        "██╔══██╗██╔═══██╗██║  ╚██╗ ██╔╝██╔════╝██║  ██║██╔════╝██╔════╝██╔════╝",
        "██████╔╝██║   ██║██║   ╚████╔╝ ██║     ███████║█████╗  ███████╗███████╗",
        "██╔═══╝ ██║   ██║██║    ╚██╔╝  ██║     ██╔══██║██╔══╝  ╚════██║╚════██║",
        "██║     ╚██████╔╝███████╗██║   ╚██████╗██║  ██║███████╗███████║███████║",
        "╚═╝      ╚═════╝ ╚══════╝╚═╝    ╚═════╝╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝",
    ];

    println!("\n   {}", text_polychess_2.join("\n   "));

    fn est_une_coordonnee(string: &str) -> bool {
        if string.len() != 2 {
            return false;
        }
        if !string.chars().next().unwrap().is_alphabetic()
            || !string.chars().nth(1).unwrap().is_digit(10)
        {
            return false;
        }
        true
    }

    fn test_couleur(position: &str, board: &Board) -> String {
        let index = board.casename_to_index(position);
        let piece = board.clone().get_piece(index);
        piece.color.to_string()
    }

    fn tour_du_joueur(couleur: &str, board: &mut Board) {
        let liste_aide_joueur = [
            (
                "Pour jouer",
                vec![
                    "jouer [coordonnées départ] [coordonnées arrivée]",
                    "[coordonnées départ] [coordonnées arrivée]",
                ],
            ),
            ("Pour connaitre les coups possibles", vec!["coups"]),
            (
                "Pour connaitre la liste des coups possibles d'une piece",
                vec!["piece"],
            ),
            ("Pour afficher l'échiquier", vec!["afficher"]),
            (
                "Pour afficher l'échiquier avec les coups possibles d'une piece",
                vec!["afficher [coordonnées]"],
            ),
            ("Pour quitter la partie", vec!["fin de partie"]),
        ];

        let aide_joueur: String = liste_aide_joueur
            .iter()
            .enumerate()
            .map(|(numero, (desc, cmds))| {
                format!(
                    "{}. {} : {} ou {}{}{}",
                    numero + 1,
                    desc,
                    cmds.join(" ou "),
                    numero + 1,
                    if numero == 0 {
                        " [coordonnées départ] [coordonnées arrivée]"
                    } else {
                        ""
                    },
                    if numero == 2 { " [coordonnées]" } else { "" }
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        let mut ne_plus_afficher_laide = false;

        loop {
            println!("Tour des {}", couleur);
            let entree_joueur = if ne_plus_afficher_laide {
                println!("Pour afficher l'aide, entrez : aide");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                input.trim().to_string()
            } else {
                println!("{}", aide_joueur);
                println!("Pour ne plus afficher l'aide, entrez : fin aide");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                input.trim().to_string()
            };

            if entree_joueur == "fin aide" {
                ne_plus_afficher_laide = true;
            } else if entree_joueur == "fin partie" {
                println!(
                    "\nVous venez de quitter la partie. Le joueur {} a abandonné",
                    couleur
                );
                std::process::exit(0);
            } else if entree_joueur.is_empty() {
                continue;
            } else if entree_joueur == "2"
                || liste_aide_joueur[1].1.contains(&entree_joueur.as_str())
            {
                println!(
                    "{}",
                    board
                        .piece_list_able_tomove_case_fmt_checked(couleur)
                        .join(", ")
                );
            } else if entree_joueur.len() >= 3
                && liste_aide_joueur[2]
                    .1
                    .contains(&&entree_joueur[..entree_joueur.len() - 3])
            {
                if test_couleur(&entree_joueur[entree_joueur.len() - 2..], board) != couleur {
                    println!("Case invalide");
                } else if est_une_coordonnee(&entree_joueur[entree_joueur.len() - 2..]) {
                    println!(
                        "{}",
                        board
                            .possible_move_list_case_fmt(&entree_joueur[entree_joueur.len() - 2..])
                            .join(", ")
                    );
                }
            } else if entree_joueur == "4"
                || liste_aide_joueur[3].1.contains(&entree_joueur.as_str())
            {
                board.display();
            } else if entree_joueur.len() >= 3
                && liste_aide_joueur[4]
                    .1
                    .contains(&&entree_joueur[..entree_joueur.len() - 3])
            {
                if est_une_coordonnee(&entree_joueur[entree_joueur.len() - 2..]) {
                    if test_couleur(&entree_joueur[entree_joueur.len() - 2..], board) != couleur {
                        println!("Case invalide");
                    } else {
                        board.display_possible_actions(&entree_joueur[entree_joueur.len() - 2..]);
                    }
                }
            } else if entree_joueur.len() >= 6
                && liste_aide_joueur[0]
                    .1
                    .iter()
                    .any(|&s| s.starts_with(&entree_joueur[..entree_joueur.len() - 6]))
            {
                deplacement(&entree_joueur, couleur, board);
                break;
            }
        }
    }

    fn deplacement(entree_joueur: &str, couleur: &str, board: &mut Board) {
        let mut valeur_deplacement = entree_joueur[entree_joueur.len() - 5..].to_string();
        loop {
            if entree_joueur == "fin partie" {
                println!(
                    "\nVous venez de quitter la partie. Le joueur {} a abandonné.",
                    couleur
                );
                std::process::exit(0);
            }
            if est_une_coordonnee(&valeur_deplacement[..2])
                && est_une_coordonnee(&valeur_deplacement[3..])
                && valeur_deplacement.chars().nth(2).unwrap() == ' '
            {
                if test_couleur(&valeur_deplacement[..2], board) == couleur {
                    if board
                        .piece_list_able_tomove_case_fmt_checked(couleur)
                        .contains(&valeur_deplacement[..2].to_string())
                    {
                        if board
                            .checked_moves_list(
                                board.casename_to_index(&valeur_deplacement[..2]),
                                couleur,
                            )
                            .contains(&valeur_deplacement[3..].to_string())
                        {
                            board.move_piece(&valeur_deplacement[..2], &valeur_deplacement[3..]);
                            break;
                        } else {
                            println!("Cette pièce ne peut pas se déplacer sur cette case");
                        }
                    } else {
                        println!("Cette pièce ne peut pas bouger");
                    }
                } else {
                    println!("La pièce choisie n'est pas {}", couleur);
                }
            } else {
                println!("Entrée invalide, entrez un déplacement valide");
            }
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            valeur_deplacement = input.trim().to_string();
        }
    }

    fn jouer_en_mode_jcj() {
        let mut board = Board::init();
        board.display();
        for _ in 0..50 {
            tour_du_joueur("blanc", &mut board);
            board.display();
            if board.is_checkmate_white() {
                println!("Les blancs ont gagné");
                break;
            }
            tour_du_joueur("noir", &mut board);
            board.display();
            if board.is_checkmate_black() {
                println!("Les noirs ont gagné");
                break;
            }
        }
    }

    let mode_de_jeu = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    };

    if mode_de_jeu == "JcJ" {
        jouer_en_mode_jcj();
    } else if mode_de_jeu == "JcIA" {
    }
}
