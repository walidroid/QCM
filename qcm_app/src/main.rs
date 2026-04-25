#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

struct Question {
    text: &'static str,
    options: [&'static str; 3],
    correct_idx: usize,
}

const QUESTIONS: &[Question] = &[
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
        options: ["A) Oui, c'est souvent utilisé.", "B) Non, cela provoque une erreur de syntaxe.", "C) Uniquement en algorithmique, mais pas en Python."],
        correct_idx: 0,
    },
    Question {
        text: "Quel type de variable est typiquement utilisé comme compteur (ex: cpt) dans une boucle complète ?",
        options: ["A) De type réel ou booléen.", "B) Uniquement de type entier.", "C) De type entier ou caractère."],
        correct_idx: 1,
    },
];

#[derive(PartialEq)]
enum AppState {
    Intro,
    Quiz,
    Result,
}

struct QcmApp {
    state: AppState,
    current_question: usize,
    score: usize,
    selected_option: Option<usize>,
    show_feedback: bool,
}

impl Default for QcmApp {
    fn default() -> Self {
        Self {
            state: AppState::Intro,
            current_question: 0,
            score: 0,
            selected_option: None,
            show_feedback: false,
        }
    }
}

impl QcmApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut style = (*cc.egui_ctx.style()).clone();
        
        // Setup modern font sizes
        style.text_styles = [
            (egui::TextStyle::Heading, egui::FontId::new(28.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Body, egui::FontId::new(18.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Button, egui::FontId::new(20.0, egui::FontFamily::Proportional)),
        ].into();
        
        style.spacing.item_spacing = egui::vec2(10.0, 20.0);
        
        let mut visuals = egui::Visuals::dark();
        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(30, 30, 40);
        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(50, 50, 65);
        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(70, 70, 90);
        visuals.widgets.active.bg_fill = egui::Color32::from_rgb(100, 100, 200);
        
        style.visuals = visuals;
        cc.egui_ctx.set_style(style);
        
        Self::default()
    }
}

impl eframe::App for QcmApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                
                match self.state {
                    AppState::Intro => {
                        ui.heading("QCM Algorithmique & Programmation");
                        ui.add_space(20.0);
                        ui.label("Testez vos connaissances en Python (2ème Sciences).");
                        ui.add_space(50.0);
                        
                        let btn = egui::Button::new("Commencer le Test").fill(egui::Color32::from_rgb(100, 100, 250));
                        if ui.add_sized([250.0, 60.0], btn).clicked() {
                            self.state = AppState::Quiz;
                        }
                    }
                    AppState::Quiz => {
                        let q = &QUESTIONS[self.current_question];
                        
                        ui.horizontal(|ui| {
                            ui.label(format!("Question {}/{}", self.current_question + 1, QUESTIONS.len()));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(format!("Score: {}", self.score));
                            });
                        });
                        
                        ui.add(egui::ProgressBar::new((self.current_question as f32) / (QUESTIONS.len() as f32))
                            .fill(egui::Color32::from_rgb(100, 100, 250)));
                        ui.add_space(30.0);
                        
                        ui.heading(q.text);
                        ui.add_space(30.0);
                        
                        for (i, option_text) in q.options.iter().enumerate() {
                            let is_selected = self.selected_option == Some(i);
                            let mut button_color = ui.visuals().widgets.inactive.bg_fill;
                            
                            if self.show_feedback {
                                if i == q.correct_idx {
                                    button_color = egui::Color32::from_rgb(46, 204, 113); // Green
                                } else if is_selected {
                                    button_color = egui::Color32::from_rgb(231, 76, 60); // Red
                                }
                            } else if is_selected {
                                button_color = egui::Color32::from_rgb(100, 100, 250); // Accent blue
                            }
                            
                            let button = egui::Button::new(*option_text)
                                .fill(button_color)
                                .min_size(egui::vec2(ui.available_width() * 0.8, 50.0));
                                
                            if ui.add(button).clicked() && !self.show_feedback {
                                self.selected_option = Some(i);
                            }
                        }
                        
                        ui.add_space(40.0);
                        
                        if self.show_feedback {
                            let next_btn = egui::Button::new("Suivant").fill(egui::Color32::from_rgb(100, 100, 250));
                            if ui.add_sized([200.0, 50.0], next_btn).clicked() {
                                self.current_question += 1;
                                self.selected_option = None;
                                self.show_feedback = false;
                                
                                if self.current_question >= QUESTIONS.len() {
                                    self.state = AppState::Result;
                                }
                            }
                        } else {
                            if ui.add_enabled_ui(self.selected_option.is_some(), |ui| {
                                ui.add_sized([200.0, 50.0], egui::Button::new("Valider"))
                            }).inner.clicked() {
                                self.show_feedback = true;
                                if self.selected_option == Some(q.correct_idx) {
                                    self.score += 1;
                                }
                            }
                        }
                    }
                    AppState::Result => {
                        ui.heading("Résultat Final");
                        ui.add_space(40.0);
                        
                        ui.label(egui::RichText::new(format!("{}/{}", self.score, QUESTIONS.len()))
                            .size(60.0)
                            .color(egui::Color32::from_rgb(100, 100, 250)));
                        
                        ui.add_space(30.0);
                        
                        let feedback = if self.score == 10 {
                            "Parfait ! Excellent travail !"
                        } else if self.score >= 7 {
                            "Très bien ! Vous avez une bonne maîtrise."
                        } else if self.score >= 5 {
                            "Pas mal, mais vous pouvez faire mieux !"
                        } else {
                            "Il faut réviser le cours !"
                        };
                        
                        ui.label(feedback);
                        ui.add_space(50.0);
                        
                        if ui.add_sized([250.0, 60.0], egui::Button::new("Recommencer")).clicked() {
                            *self = QcmApp::default();
                            self.state = AppState::Quiz;
                        }
                    }
                }
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("QCM Python - 2ème Sciences"),
        ..Default::default()
    };
    eframe::run_native(
        "QCM Python",
        options,
        Box::new(|cc| Ok(Box::new(QcmApp::new(cc)))),
    )
}
