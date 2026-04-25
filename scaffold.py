import os
import json

def write_file(path, content):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, 'w', encoding='utf-8') as f:
        f.write(content)

base_dir = "qcm-app"

# 1. index.html
html_content = """<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>QCM Python</title>
    <link rel="stylesheet" href="styles.css">
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;600;800&display=swap" rel="stylesheet">
</head>
<body>
    <div class="background-shapes">
        <div class="shape shape-1"></div>
        <div class="shape shape-2"></div>
        <div class="shape shape-3"></div>
    </div>
    
    <div class="container glass-panel">
        <div id="intro-screen" class="screen active">
            <h1>QCM Algorithmique <br><span>& Programmation Python</span></h1>
            <p>Testez vos connaissances en algorithmique et Python (2ème Sciences).</p>
            <button id="start-btn" class="btn primary-btn">Commencer le Test</button>
        </div>

        <div id="quiz-screen" class="screen">
            <div class="progress-bar-container">
                <div id="progress-bar" class="progress-bar"></div>
            </div>
            <div class="question-header">
                <span id="question-number">Question 1/10</span>
                <span id="score-live">Score: 0</span>
            </div>
            <h2 id="question-text">Question text here</h2>
            <div id="options-container" class="options">
                <!-- Options will be injected here -->
            </div>
            <div class="actions">
                <button id="next-btn" class="btn secondary-btn" disabled>Suivant</button>
            </div>
        </div>

        <div id="result-screen" class="screen">
            <h2>Résultat Final</h2>
            <div class="score-circle">
                <span id="final-score">0</span><span class="total">/10</span>
            </div>
            <p id="feedback-message">Bravo !</p>
            <button id="restart-btn" class="btn primary-btn">Recommencer</button>
        </div>
    </div>

    <script src="main.js"></script>
</body>
</html>
"""

# 2. styles.css
css_content = """* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
    font-family: 'Inter', sans-serif;
}

body {
    background-color: #0f172a;
    color: #f8fafc;
    min-height: 100vh;
    display: flex;
    justify-content: center;
    align-items: center;
    overflow: hidden;
    position: relative;
}

.background-shapes {
    position: absolute;
    width: 100vw;
    height: 100vh;
    z-index: -1;
    overflow: hidden;
}

.shape {
    position: absolute;
    border-radius: 50%;
    filter: blur(80px);
}

.shape-1 {
    width: 400px;
    height: 400px;
    background: #8b5cf6;
    top: -100px;
    left: -100px;
    opacity: 0.5;
    animation: float 10s infinite alternate;
}

.shape-2 {
    width: 500px;
    height: 500px;
    background: #3b82f6;
    bottom: -150px;
    right: -100px;
    opacity: 0.4;
    animation: float 12s infinite alternate-reverse;
}

.shape-3 {
    width: 300px;
    height: 300px;
    background: #ec4899;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    opacity: 0.3;
    animation: pulse 8s infinite alternate;
}

@keyframes float {
    0% { transform: translateY(0) translateX(0); }
    100% { transform: translateY(50px) translateX(50px); }
}

@keyframes pulse {
    0% { transform: translate(-50%, -50%) scale(1); }
    100% { transform: translate(-50%, -50%) scale(1.2); }
}

.container {
    width: 90%;
    max-width: 600px;
    min-height: 400px;
    position: relative;
}

.glass-panel {
    background: rgba(30, 41, 59, 0.7);
    backdrop-filter: blur(16px);
    -webkit-backdrop-filter: blur(16px);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 24px;
    padding: 40px;
    box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
}

.screen {
    display: none;
    animation: fadeIn 0.5s ease-out forwards;
}

.screen.active {
    display: flex;
    flex-direction: column;
    height: 100%;
}

@keyframes fadeIn {
    from { opacity: 0; transform: translateY(20px); }
    to { opacity: 1; transform: translateY(0); }
}

#intro-screen {
    text-align: center;
    justify-content: center;
    align-items: center;
    gap: 20px;
}

h1 {
    font-size: 2.5rem;
    font-weight: 800;
    line-height: 1.2;
    margin-bottom: 10px;
}

h1 span {
    background: linear-gradient(to right, #60a5fa, #a78bfa);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
}

p {
    color: #94a3b8;
    font-size: 1.1rem;
    line-height: 1.6;
}

.btn {
    padding: 14px 28px;
    border-radius: 12px;
    font-size: 1.1rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.3s ease;
    border: none;
    outline: none;
}

.primary-btn {
    background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%);
    color: white;
    box-shadow: 0 10px 20px -10px rgba(99, 102, 241, 0.6);
}

.primary-btn:hover {
    transform: translateY(-2px);
    box-shadow: 0 15px 25px -10px rgba(99, 102, 241, 0.8);
}

.secondary-btn {
    background: #334155;
    color: white;
}

.secondary-btn:hover:not(:disabled) {
    background: #475569;
}

.secondary-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

/* Quiz Screen */
.progress-bar-container {
    width: 100%;
    height: 6px;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 3px;
    margin-bottom: 30px;
    overflow: hidden;
}

.progress-bar {
    height: 100%;
    background: linear-gradient(90deg, #3b82f6, #8b5cf6);
    width: 0%;
    transition: width 0.3s ease;
}

.question-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
    font-size: 0.9rem;
    color: #94a3b8;
    font-weight: 600;
}

h2 {
    font-size: 1.5rem;
    font-weight: 600;
    margin-bottom: 30px;
    line-height: 1.4;
}

.options {
    display: flex;
    flex-direction: column;
    gap: 15px;
    margin-bottom: 30px;
}

.option {
    background: rgba(255, 255, 255, 0.05);
    border: 2px solid rgba(255, 255, 255, 0.1);
    padding: 16px 20px;
    border-radius: 12px;
    cursor: pointer;
    font-size: 1rem;
    color: #e2e8f0;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
}

.option:hover {
    background: rgba(255, 255, 255, 0.1);
    border-color: rgba(255, 255, 255, 0.2);
    transform: translateX(5px);
}

.option.selected {
    background: rgba(99, 102, 241, 0.2);
    border-color: #6366f1;
}

.option.correct {
    background: rgba(34, 197, 94, 0.2);
    border-color: #22c55e;
}

.option.wrong {
    background: rgba(239, 68, 68, 0.2);
    border-color: #ef4444;
}

.actions {
    display: flex;
    justify-content: flex-end;
    margin-top: auto;
}

/* Result Screen */
#result-screen {
    text-align: center;
    justify-content: center;
    align-items: center;
}

.score-circle {
    width: 150px;
    height: 150px;
    border-radius: 50%;
    background: linear-gradient(135deg, rgba(99, 102, 241, 0.2), rgba(139, 92, 246, 0.2));
    border: 4px solid #8b5cf6;
    display: flex;
    justify-content: center;
    align-items: center;
    margin: 30px auto;
    box-shadow: 0 0 30px rgba(139, 92, 246, 0.3);
}

#final-score {
    font-size: 3.5rem;
    font-weight: 800;
    color: white;
}

.total {
    font-size: 1.5rem;
    color: #94a3b8;
    margin-top: 15px;
}

#feedback-message {
    font-size: 1.2rem;
    margin-bottom: 30px;
    color: #e2e8f0;
}
"""

# 3. main.js
js_content = """const questions = [
    {
        question: "Quel est l'équivalent en Python de la fonction algorithmique long(ch) ?",
        options: ["A) length(ch)", "B) len(ch)", "C) size(ch)"],
        correct: 1
    },
    {
        question: "À quoi correspond la fonction algorithmique pos(c, ch) en langage Python ?",
        options: ["A) ch.search(c)", "B) find(ch, c)", "C) ch.find(c)"],
        correct: 2
    },
    {
        question: "Si la variable p prend la valeur -1 après l'exécution de p = ch.find(c), qu'est-ce que cela signifie ?",
        options: ["A) Le caractère c se trouve à la fin de la chaine.", "B) Le caractère c n'existe pas dans la chaine ch.", "C) La chaine de caractères ch est vide."],
        correct: 1
    },
    {
        question: "Comment sait-on qu'une chaine ch de longueur l est vide ?",
        options: ["A) Si l = -1", "B) Si l = 1", "C) Si l = 0"],
        correct: 2
    },
    {
        question: "Quel est le rôle principal de la structure répétitive complète ?",
        options: ["A) Répéter un traitement jusqu'à ce qu'une condition devienne fausse.", "B) Répéter un traitement un nombre fini et connu à l'avance.", "C) Exécuter un traitement uniquement si une condition est vraie."],
        correct: 1
    },
    {
        question: "Quelle est la syntaxe correcte de la boucle complète en algorithme ?",
        options: ["A) Pour cpt de début à fin faire", "B) Tant que cpt < fin faire", "C) Répéter cpt jusqu'à fin"],
        correct: 0
    },
    {
        question: "Quel est l'intervalle des nombres entiers générés par la fonction spéciale range(n) en Python ?",
        options: ["A) [1 ... n]", "B) [0 ... n]", "C) [0 ... n-1]"],
        correct: 2
    },
    {
        question: "Dans la boucle Python for cpt in séquence:, que peut représenter le terme « séquence » ?",
        options: ["A) Uniquement une liste de valeurs numériques.", "B) Une chaine de caractères ou une liste de valeurs.", "C) Seulement une variable de type entier."],
        correct: 1
    },
    {
        question: "Est-il possible d'utiliser une structure conditionnelle à l'intérieur d'un traitement répétitif (une boucle) ?",
        options: ["A) Oui, c'est souvent utilisé.", "B) Non, cela provoque une erreur de syntaxe.", "C) Uniquement en algorithmique, mais pas en Python."],
        correct: 0
    },
    {
        question: "Quel type de variable est typiquement utilisé comme compteur (ex: cpt) dans une boucle complète ?",
        options: ["A) De type réel ou booléen.", "B) Uniquement de type entier.", "C) De type entier ou caractère."],
        correct: 1
    }
];

let currentQuestion = 0;
let score = 0;
let answered = false;

const introScreen = document.getElementById('intro-screen');
const quizScreen = document.getElementById('quiz-screen');
const resultScreen = document.getElementById('result-screen');

const startBtn = document.getElementById('start-btn');
const nextBtn = document.getElementById('next-btn');
const restartBtn = document.getElementById('restart-btn');

const questionText = document.getElementById('question-text');
const optionsContainer = document.getElementById('options-container');
const questionNumber = document.getElementById('question-number');
const scoreLive = document.getElementById('score-live');
const progressBar = document.getElementById('progress-bar');

startBtn.addEventListener('click', startQuiz);
nextBtn.addEventListener('click', () => {
    if (answered) {
        currentQuestion++;
        if (currentQuestion < questions.length) {
            loadQuestion();
        } else {
            showResult();
        }
    }
});
restartBtn.addEventListener('click', () => {
    currentQuestion = 0;
    score = 0;
    startQuiz();
});

function startQuiz() {
    introScreen.classList.remove('active');
    resultScreen.classList.remove('active');
    quizScreen.classList.add('active');
    loadQuestion();
}

function loadQuestion() {
    answered = false;
    nextBtn.disabled = true;
    const q = questions[currentQuestion];
    
    questionText.textContent = q.question;
    questionNumber.textContent = `Question ${currentQuestion + 1}/${questions.length}`;
    scoreLive.textContent = `Score: ${score}`;
    progressBar.style.width = `${(currentQuestion / questions.length) * 100}%`;
    
    optionsContainer.innerHTML = '';
    
    q.options.forEach((opt, index) => {
        const div = document.createElement('div');
        div.className = 'option';
        div.textContent = opt;
        div.addEventListener('click', () => selectOption(div, index));
        optionsContainer.appendChild(div);
    });
}

function selectOption(selectedDiv, index) {
    if (answered) return;
    answered = true;
    nextBtn.disabled = false;
    
    const correctIndex = questions[currentQuestion].correct;
    const options = optionsContainer.children;
    
    if (index === correctIndex) {
        selectedDiv.classList.add('correct');
        score++;
        scoreLive.textContent = `Score: ${score}`;
    } else {
        selectedDiv.classList.add('wrong');
        options[correctIndex].classList.add('correct');
    }
}

function showResult() {
    quizScreen.classList.remove('active');
    resultScreen.classList.add('active');
    
    document.getElementById('final-score').textContent = score;
    
    const feedback = document.getElementById('feedback-message');
    if (score === 10) feedback.textContent = "Parfait ! Excellent travail !";
    else if (score >= 7) feedback.textContent = "Très bien ! Vous avez une bonne maîtrise.";
    else if (score >= 5) feedback.textContent = "Pas mal, mais vous pouvez faire mieux !";
    else feedback.textContent = "Il faut réviser le cours !";
}
"""

# 4. Cargo.toml
cargo_content = """[package]
name = "qcm-app"
version = "0.1.0"
edition = "2021"
description = "QCM App"

[build-dependencies]
tauri-build = { version = "1" }

[dependencies]
tauri = { version = "1", features = ["api-all"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
"""

# 5. tauri.conf.json
tauri_conf = {
  "build": {
    "beforeDevCommand": "",
    "beforeBuildCommand": "",
    "devPath": "../src",
    "distDir": "../src"
  },
  "package": {
    "productName": "QCM App",
    "version": "0.1.0"
  },
  "tauri": {
    "allowlist": {
      "all": False
    },
    "bundle": {
      "active": True,
      "category": "Education",
      "copyright": "",
      "deb": { "depends": [] },
      "externalBin": [],
      "identifier": "com.qcm.app",
      "longDescription": "",
      "macOS": {
        "entitlements": None,
        "exceptionDomain": "",
        "frameworks": [],
        "providerShortName": None,
        "signingIdentity": None
      },
      "resources": [],
      "shortDescription": "",
      "targets": "all",
      "windows": {
        "certificateThumbprint": None,
        "digestAlgorithm": "sha256",
        "timestampUrl": ""
      }
    },
    "security": { "csp": None },
    "updater": { "active": False },
    "windows": [
      {
        "fullscreen": False,
        "height": 650,
        "resizable": True,
        "title": "QCM Python - 2ème Sciences",
        "width": 800
      }
    ]
  }
}

# 6. main.rs
main_rs = """#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
"""

# 7. build.rs
build_rs = """fn main() {
  tauri_build::build()
}
"""

# 8. GitHub Action workflow
github_action = """name: Build Windows EXE
on:
  push:
    branches: [ main, master ]
  pull_request:
    branches: [ main, master ]

jobs:
  build-tauri:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install Tauri CLI
        run: cargo install tauri-cli --version "^1.0.0"

      - name: Build Tauri App
        run: cargo tauri build
        working-directory: ./src-tauri
        
      - name: Upload EXE
        uses: actions/upload-artifact@v3
        with:
          name: QCM-App-Windows
          path: src-tauri/target/release/bundle/msi/*.msi
"""

write_file(f"{base_dir}/src/index.html", html_content)
write_file(f"{base_dir}/src/styles.css", css_content)
write_file(f"{base_dir}/src/main.js", js_content)
write_file(f"{base_dir}/src-tauri/Cargo.toml", cargo_content)
write_file(f"{base_dir}/src-tauri/tauri.conf.json", json.dumps(tauri_conf, indent=2))
write_file(f"{base_dir}/src-tauri/src/main.rs", main_rs)
write_file(f"{base_dir}/src-tauri/build.rs", build_rs)
write_file(f"{base_dir}/.github/workflows/build.yml", github_action)

print("Project generated successfully!")
