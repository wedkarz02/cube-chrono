const isLoggedIn = document.body.getAttribute('data-is-logged-in') === 'true';

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
// const newSessionButton = document.getElementById('new-session');

// Funkcja formatowania czasu
function formatTime(ms) {
    const hours = String(Math.floor(ms / 3600000)).padStart(2, '0');
    const minutes = String(Math.floor((ms % 3600000) / 60000)).padStart(2, '0');
    const seconds = String(Math.floor((ms % 60000) / 1000)).padStart(2, '0');
    const milliseconds = String(ms % 1000).padStart(3, '0');
    return { hours, minutes, seconds, milliseconds };
}

// newSessionButton.addEventListener('click', () => {
    
// });    

// Start Timer
startButton.addEventListener('click', () => {
    if (!timerInterval) {
        startTime = Date.now();
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
stopButton.addEventListener('click', async () => {
    clearInterval(timerInterval);
    timerInterval = null;

    if (isLoggedIn) {
        const data = {
            millis: elapsedTime
            //user: 
        };

        try {
            const response = await fetch('/solveTime', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(data)
            });

            if (response.status == 200) {
                alert(`Ukończono w ${elapsedTime} milisekund!`);
            } else {
                alert('Błąd zapisu!');
            }
        } catch (error) {
            console.error('Błąd połączenia:', error);
            alert('Wystąpił błąd połączenia z serwerem.');
        }
    }    
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

async function generateScramble(kind, count) {
    // try {
        const response = await fetch(`http://localhost:8080/api/v1/scrambles?kind=${kind}&count=${count}`, {
            method: 'GET',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            }
        });

        console.log(response);
        const jsonResult = await response.json();
        console.log(jsonResult);

        if (response.status == 200) {
            return jsonResult.payload.scrambles[0].sequence;
        } else {
            return jsonResult.message;
        }
    // } catch (error) {
    //     console.error('Błąd połączenia:', error);
    //     alert('Wystąpił błąd połączenia z serwerem.');
    // }
}

// Wyświetlanie scramble na kliknięcie przycisku
scrambleButton.addEventListener('click', async () => {
    scrambleDisplay.textContent = await generateScramble("Three", 1);
});

// Generowanie scramble przy załadowaniu strony
document.addEventListener('DOMContentLoaded', async () => {
    scrambleDisplay.textContent = await generateScramble("Three", 1);
});
