// Timer functionality
let timerInterval;
let startTime;

const startButton = document.getElementById('start');
const stopButton = document.getElementById('stop');
const resetButton = document.getElementById('reset');
const timerDisplay = document.getElementById('timer');

startButton.addEventListener('click', () => {
    if (!timerInterval) {
        startTime = Date.now();
        timerInterval = setInterval(() => {
            const elapsedTime = Date.now() - startTime;
            const seconds = Math.floor(elapsedTime / 1000) % 60;
            const minutes = Math.floor(elapsedTime / 60000) % 60;
            const hours = Math.floor(elapsedTime / 3600000);
            timerDisplay.textContent = `${hours.toString().padStart(2, '0')}:${minutes
                .toString()
                .padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
        }, 1000);
    }
});

stopButton.addEventListener('click', () => {
    clearInterval(timerInterval);
    timerInterval = null;
});

resetButton.addEventListener('click', () => {
    clearInterval(timerInterval);
    timerInterval = null;
    timerDisplay.textContent = "00:00:00";
});

// Scramble functionality
const scrambleDisplay = document.getElementById('scramble-display');
const scrambleButton = document.getElementById('generate-scramble');

const generateScramble = () => {
    const moves = ["U", "D", "L", "R", "F", "B"];
    const modifiers = ["", "'", "2"];
    let scramble = [];
    for (let i = 0; i < 20; i++) {
        const move = moves[Math.floor(Math.random() * moves.length)];
        const modifier = modifiers[Math.floor(Math.random() * modifiers.length)];
        scramble.push(move + modifier);
    }
    return scramble.join(" ");
};

scrambleButton.addEventListener('click', () => {
    scrambleDisplay.textContent = generateScramble();
});