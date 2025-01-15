// === TIMER === //
let timerInterval;
let startTime;
let elapsedTime = 0;

// Pobieranie elementów DOM
const hoursDisplay = document.getElementById('hours');
const minutesDisplay = document.getElementById('minutes');
const secondsDisplay = document.getElementById('seconds');
const millisecondsDisplay = document.getElementById('milliseconds');
const startButton = document.getElementById('start');
const stopButton = document.getElementById('stop');
const resetButton = document.getElementById('reset');

// Funkcja formatowania czasu
function formatTime(ms) {
    const hours = String(Math.floor(ms / 3600000)).padStart(2, '0');
    const minutes = String(Math.floor((ms % 3600000) / 60000)).padStart(2, '0');
    const seconds = String(Math.floor((ms % 60000) / 1000)).padStart(2, '0');
    const milliseconds = String(ms % 1000).padStart(3, '0');
    return { hours, minutes, seconds, milliseconds };
}

// Start Timer
startButton.addEventListener('click', () => {
    if (!timerInterval) {
        startTime = Date.now() - elapsedTime;
        timerInterval = setInterval(() => {
            elapsedTime = Date.now() - startTime;
            const { hours, minutes, seconds, milliseconds } = formatTime(elapsedTime);
            hoursDisplay.textContent = hours;
            minutesDisplay.textContent = minutes;
            secondsDisplay.textContent = seconds;
            millisecondsDisplay.textContent = milliseconds;
        }, 10);
    }
});

// Stop Timer
stopButton.addEventListener('click', () => {
    clearInterval(timerInterval);
    timerInterval = null;
});

// Reset Timer
resetButton.addEventListener('click', () => {
    clearInterval(timerInterval);
    timerInterval = null;
    elapsedTime = 0;
    hoursDisplay.textContent = '00';
    minutesDisplay.textContent = '00';
    secondsDisplay.textContent = '00';
    millisecondsDisplay.textContent = '000';
});

// === SCRAMBLE === //
const scrambleDisplay = document.getElementById('scramble-display');
const scrambleButton = document.getElementById('generate-scramble');

// Generowanie scramble z zasadami WCA
function generateScramble() {
    const moves = ["U", "D", "L", "R", "F", "B"];
    const modifiers = ["", "'", "2"];
    let scramble = [];
    let previousMove = null;

    for (let i = 0; i < 20; i++) {
        let move;
        do {
            move = moves[Math.floor(Math.random() * moves.length)];
        } while (move === previousMove); // Unikamy powtórzenia tego samego ruchu

        const modifier = modifiers[Math.floor(Math.random() * modifiers.length)];
        scramble.push(move + modifier);
        previousMove = move;
    }

    return scramble.join(" ");
}

// Wyświetlanie scramble na kliknięcie przycisku
scrambleButton.addEventListener('click', () => {
    scrambleDisplay.textContent = generateScramble();
});

// Generowanie scramble przy załadowaniu strony
document.addEventListener('DOMContentLoaded', () => {
    scrambleDisplay.textContent = generateScramble();
});
