const questions = [
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
