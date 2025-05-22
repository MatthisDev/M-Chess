use yew::prelude::*;

#[function_component(Info)]
pub fn info() -> Html {
    html! {
        <div id="presentation" class="tab-content">
                                <h2>{ "Contexte du Projet" }</h2>
                                <p>{ "Les échecs sont un jeu stratégique intemporel qui attire des joueurs de tous âges et niveaux. Avec l'évolution des technologies, développer une plateforme de jeu d'échecs en ligne, intégrant une intelligence artificielle compétitive et une interface intuitive, représente une opportunité d'offrir une expérience moderne et immersive aux utilisateurs." }</p>
                                <p>{ "Ce projet vise à concevoir un jeu d'échecs accessible aussi bien en mode hors ligne qu'en multijoueur via Internet ou un réseau local. Le développement reposera sur le langage Rust pour la robustesse du backend, tandis que l'interface utilisateur s'appuiera sur des technologies web modernes." }</p>

                                <h2>{ "Présentation de l'équipe de développement" }</h2>
                                <h3>{ "Matthis Guillet" }</h3>
                                <p>{ "Étudiant en deuxième année à l'EPITA, passionné d'informatique et de mathématiques depuis le collège. Il aime explorer de nouvelles disciplines et réaliser des projets pratiques pour appliquer ses connaissances. Il souhaite évoluer dans les domaines de l'IA et de la robotique en tant qu'ingénieur ou chercheur." }</p>

                                <h3>{ "Martin Madier" }</h3>
                                <p>{ "Également étudiant en deuxième année à l'EPITA, il s'intéresse à l'informatique et aux nouvelles technologies. Ce projet est une opportunité pour lui d'acquérir des compétences en IA et en programmation en Rust, tout en découvrant le fonctionnement d'une application web basée sur des requêtes API." }</p>

                                <h3>{ "Martin Pasquier" }</h3>
                                <p>{ "Étudiant en deuxième année à l’EPITA, passionné par l'informatique et la technologie. Il consacre du temps à des projets annexes afin d'approfondir ses compétences techniques. Doté d’une solide expérience en développement web, il s'intéresse aussi à la gestion de projets avec Git. En tant que chef de groupe, il assurera la coordination des différentes étapes du projet." }</p>

                                <h3>{ "Matteo Wermert" }</h3>
                                <p>{ "Étudiant en deuxième année à l’EPITA, fasciné par les échecs, un jeu qu'il admire pour sa richesse stratégique et son élégance. Ce projet lui permet d'associer cette passion à l’apprentissage du langage Rust, lui offrant ainsi une opportunité d’acquérir des compétences précieuses pour son avenir." }</p>

                                <h2>{ "Objectifs du Projet" }</h2>
                                <h3>{ "Objectifs Généraux" }</h3>
                                <p>{ "Notre objectif est de développer un jeu d'échecs en ligne complet et fonctionnel, doté d'une intelligence artificielle capable de calculer des coups optimaux selon différents niveaux de difficulté. Une interface web performante et intuitive garantira une expérience utilisateur fluide et agréable." }</p>

                                <h3>{ "Objectifs Spécifiques" }</h3>
                                <p>{ "Le projet permettra aux utilisateurs de jouer contre une intelligence artificielle ou contre un autre joueur, en local ou en ligne. L'interface web sera conçue pour offrir une navigation rapide et intuitive grâce à une architecture web moderne." }</p>
                            </div>
    }
}
