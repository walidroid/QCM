#![windows_subsystem = "windows"]
slint::slint! {
    import { VerticalBox, HorizontalBox } from "std-widgets.slint";

    component OptionButton inherits Rectangle {
        in property <string> text;
        in property <bool> is-selected;
        in property <bool> is-correct;
        in property <bool> is-wrong;
        in property <bool> disabled;
        callback clicked();

        height: 60px;
        border-radius: 12px;
        border-width: 2px;
        border-color: transparent;
        background: rgba(255, 255, 255, 0.05);

        states [
            correct when is-correct: {
                background: rgba(16, 185, 129, 0.15);
                border-color: #10b981;
            }
            wrong when is-wrong: {
                background: rgba(239, 68, 68, 0.15);
                border-color: #ef4444;
            }
            hover when ta.has-hover && !disabled: {
                background: rgba(255, 255, 255, 0.1);
                border-color: #6366f1;
            }
        ]

        animate background { duration: 200ms; }
        animate border-color { duration: 200ms; }

        Text {
            text: root.text;
            color: #f8fafc;
            font-size: 16px;
            vertical-alignment: center;
            x: 20px;
        }

        ta := TouchArea {
            clicked => { if (!root.disabled) { root.clicked(); } }
        }
    }

    export component App inherits Window {
        title: "QCM Python";
        width: 800px;
        height: 600px;
        background: #0f172a;
        default-font-family: "Segoe UI";

        in property <string> question-text: "Question ?";
        in property <string> opt-a: "A";
        in property <string> opt-b: "B";
        in property <string> opt-c: "C";
        in property <int> current-q: 1;
        in property <int> total-q: 10;
        in property <int> score: 0;
        in property <bool> show-results: false;
        
        in property <int> selected-idx: -1;
        in property <int> correct-idx: -1;

        callback option-clicked(int);
        callback next-clicked();
        callback restart-clicked();

        VerticalBox {
            padding: 40px;
            alignment: center;

            if !show-results : VerticalBox {
                spacing: 20px;
                alignment: start;

                Text {
                    text: "Algorithmique et Programmation en Python";
                    font-size: 24px;
                    color: #6366f1;
                    font-weight: 700;
                    horizontal-alignment: center;
                }

                Rectangle { height: 20px; }

                Text {
                    text: question-text;
                    font-size: 20px;
                    color: #f8fafc;
                    wrap: word-wrap;
                }

                Rectangle { height: 20px; }

                OptionButton {
                    text: opt-a;
                    is-selected: selected-idx == 0;
                    is-correct: correct-idx == 0 && selected-idx != -1;
                    is-wrong: selected-idx == 0 && correct-idx != 0;
                    disabled: selected-idx != -1;
                    clicked => { option-clicked(0); }
                }
                OptionButton {
                    text: opt-b;
                    is-selected: selected-idx == 1;
                    is-correct: correct-idx == 1 && selected-idx != -1;
                    is-wrong: selected-idx == 1 && correct-idx != 1;
                    disabled: selected-idx != -1;
                    clicked => { option-clicked(1); }
                }
                OptionButton {
                    text: opt-c;
                    is-selected: selected-idx == 2;
                    is-correct: correct-idx == 2 && selected-idx != -1;
                    is-wrong: selected-idx == 2 && correct-idx != 2;
                    disabled: selected-idx != -1;
                    clicked => { option-clicked(2); }
                }

                Rectangle { height: 20px; }

                HorizontalBox {
                    alignment: space-between;
                    Text {
                        text: "Question " + current-q + " sur " + total-q;
                        color: #94a3b8;
                        font-size: 14px;
                        vertical-alignment: center;
                    }
                    Rectangle {
                        width: 120px;
                        height: 40px;
                        background: selected-idx != -1 ? #6366f1 : #334155;
                        border-radius: 8px;
                        Text {
                            text: current-q == total-q ? "Terminer" : "Suivant";
                            color: white;
                            font-size: 16px;
                            font-weight: 600;
                            horizontal-alignment: center;
                            vertical-alignment: center;
                        }
                        TouchArea {
                            clicked => { if (selected-idx != -1) { next-clicked(); } }
                        }
                        animate background { duration: 200ms; }
                    }
                }
            }

            if show-results : VerticalBox {
                spacing: 20px;
                alignment: center;
                Text {
                    text: "Résultats du QCM";
                    font-size: 32px;
                    color: #6366f1;
                    font-weight: 700;
                    horizontal-alignment: center;
                }
                Text {
                    text: "Votre score est de";
                    font-size: 20px;
                    color: #f8fafc;
                    horizontal-alignment: center;
                }
                Text {
                    text: score + " / " + total-q;
                    font-size: 48px;
                    color: #10b981;
                    font-weight: 800;
                    horizontal-alignment: center;
                }
                Rectangle { height: 40px; }
                Rectangle {
                    width: 200px;
                    height: 50px;
                    background: ta2.has-hover ? #4f46e5 : #6366f1;
                    border-radius: 8px;
                    horizontal-stretch: 0;
                    Text {
                        text: "Recommencer";
                        color: white;
                        font-size: 18px;
                        font-weight: 600;
                        horizontal-alignment: center;
                        vertical-alignment: center;
                    }
                    ta2 := TouchArea {
                        clicked => { restart-clicked(); }
                    }
                    animate background { duration: 200ms; }
                }
            }
        }
    }
}

struct Question {
    text: &'static str,
    options: [&'static str; 3],
    correct_idx: i32,
}

fn get_questions() -> Vec<Question> {
    vec![
        Question {
            text: "Quel est l'équivalent en Python de la fonction algorithmique long(ch) ?",
            options: ["A) length(ch)", "B) len(ch)", "C) size(ch)"],
            correct_idx: 1,
        },
        Question {
            text: "À quoi correspond la fonction algorithmique pos(c, ch) en langage Python ?",
            options: ["A) ch.search(c)", "B) find(ch, c)", "C) ch.find(c)"],
            correct_idx: 2,
        },
        Question {
            text: "Si la variable p prend la valeur -1 après l'exécution de p = ch.find(c), qu'est-ce que cela signifie ?",
            options: ["A) Le caractère c se trouve à la fin de la chaine.", "B) Le caractère c n'existe pas dans la chaine ch.", "C) La chaine de caractères ch est vide."],
            correct_idx: 1,
        },
        Question {
            text: "Comment sait-on qu'une chaine ch de longueur l est vide ?",
            options: ["A) Si l = -1", "B) Si l = 1", "C) Si l = 0"],
            correct_idx: 2,
        },
        Question {
            text: "Quel est le rôle principal de la structure répétitive complète ?",
            options: ["A) Répéter un traitement jusqu'à ce qu'une condition devienne fausse.", "B) Répéter un traitement un nombre fini et connu à l'avance.", "C) Exécuter un traitement uniquement si une condition est vraie."],
            correct_idx: 1,
        },
        Question {
            text: "Quelle est la syntaxe correcte de la boucle complète en algorithme ?",
            options: ["A) Pour cpt de début à fin faire", "B) Tant que cpt < fin faire", "C) Répéter cpt jusqu'à fin"],
            correct_idx: 0,
        },
        Question {
            text: "Quel est l'intervalle des nombres entiers générés par la fonction spéciale range(n) en Python ?",
            options: ["A) [1 ... n]", "B) [0 ... n]", "C) [0 ... n-1]"],
            correct_idx: 2,
        },
        Question {
            text: "Dans la boucle Python for cpt in séquence:, que peut représenter le terme « séquence » ?",
            options: ["A) Uniquement une liste de valeurs numériques.", "B) Une chaine de caractères ou une liste de valeurs.", "C) Seulement une variable de type entier."],
            correct_idx: 1,
        },
        Question {
            text: "Est-il possible d'utiliser une structure conditionnelle à l'intérieur d'un traitement répétitif (une boucle) ?",
            options: ["A) Oui, c'est souvent utilisé (par exemple pour compter les voyelles dans un mot).", "B) Non, cela provoque une erreur de syntaxe.", "C) Uniquement en algorithmique, mais pas en Python."],
            correct_idx: 0,
        },
        Question {
            text: "Quel type de variable est typiquement utilisé comme compteur (ex: cpt) dans une boucle complète ?",
            options: ["A) De type réel ou booléen.", "B) Uniquement de type entier.", "C) De type entier ou caractère."],
            correct_idx: 1,
        },
    ]
}

fn main() -> Result<(), slint::PlatformError> {
    let app = App::new()?;
    let questions = std::rc::Rc::new(get_questions());
    
    app.set_question_text(questions[0].text.into());
    app.set_opt_a(questions[0].options[0].into());
    app.set_opt_b(questions[0].options[1].into());
    app.set_opt_c(questions[0].options[2].into());
    app.set_correct_idx(questions[0].correct_idx);
    app.set_total_q(questions.len() as i32);

    let app_weak = app.as_weak();
    app.on_option_clicked(move |idx| {
        if let Some(app) = app_weak.upgrade() {
            if app.get_selected_idx() == -1 {
                app.set_selected_idx(idx);
                if idx == app.get_correct_idx() {
                    app.set_score(app.get_score() + 1);
                }
            }
        }
    });

    let app_weak2 = app.as_weak();
    let questions2 = std::rc::Rc::clone(&questions);
    app.on_next_clicked(move || {
        if let Some(app) = app_weak2.upgrade() {
            let current = app.get_current_q() as usize;
            
            if current < questions2.len() {
                let next_q = &questions2[current];
                app.set_question_text(next_q.text.into());
                app.set_opt_a(next_q.options[0].into());
                app.set_opt_b(next_q.options[1].into());
                app.set_opt_c(next_q.options[2].into());
                app.set_correct_idx(next_q.correct_idx);
                app.set_selected_idx(-1);
                app.set_current_q((current + 1) as i32);
            } else {
                app.set_show_results(true);
            }
        }
    });

    let app_weak3 = app.as_weak();
    let questions3 = std::rc::Rc::clone(&questions);
    app.on_restart_clicked(move || {
        if let Some(app) = app_weak3.upgrade() {
            app.set_score(0);
            app.set_current_q(1);
            app.set_show_results(false);
            app.set_selected_idx(-1);
            
            let first_q = &questions3[0];
            app.set_question_text(first_q.text.into());
            app.set_opt_a(first_q.options[0].into());
            app.set_opt_b(first_q.options[1].into());
            app.set_opt_c(first_q.options[2].into());
            app.set_correct_idx(first_q.correct_idx);
        }
    });

    app.run()
}
