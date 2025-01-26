const isLoggedIn = document.body.getAttribute('data-is-logged-in') === 'true';

let globalKind;
let globalSequence;
let sessionID;

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
const newSessionButton = document.getElementById('new-session');

// Funkcja formatowania czasu
function formatTime(ms) {
    const hours = String(Math.floor(ms / 3600000)).padStart(2, '0');
    const minutes = String(Math.floor((ms % 3600000) / 60000)).padStart(2, '0');
    const seconds = String(Math.floor((ms % 60000) / 1000)).padStart(2, '0');
    const milliseconds = String(ms % 1000).padStart(3, '0');
    return { hours, minutes, seconds, milliseconds };
}

if (newSessionButton !== null) {
    newSessionButton.addEventListener('click', async () => {
        let sessionName = "Sesja 1";
        try {
            const data = {
                name: sessionName
            };
    
            const response = await fetch(`http://localhost:3000/new-session`, {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(data)
            });
    
            const jsonResult = await response.json();
            
            if (response.ok) {
                sessionID = jsonResult.payload.session_id;
                alert('Stworzono nową sesję.');
            } else {
                alert(jsonResult.message);
            }
        } catch (error) {
            console.error('Błąd połączenia:', error);
            alert('Wystąpił błąd połączenia z serwerem.');
        }
    });    
}

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

    if ((isLoggedIn && sessionID !== undefined && sessionID !== null)) {
        const scramble = {
            kind: globalKind,
            sequence: globalSequence
        };

        const time = {
            millis: elapsedTime,
            recorded_at: Date.now(),
            scramble: scramble
        };

        const data = {
            session_id: sessionID,
            time: time
        };

        try {
            const response = await fetch('/add-time', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(data)
            });

            const jsonResult = await response.json();

            if (response.status == 200) {
                alert(`Zapisano czas w sesji.`);
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
    try {
        const data = {
            kind: kind,
            count: count
        };

        const response = await fetch(`http://localhost:3000/scrambles`, {
            method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(data)
        });

        const jsonResult = await response.json();

        if (response.status == 200) {
            globalSequence = jsonResult.payload.scrambles[0].sequence;
            globalKind = kind;
            return jsonResult.payload.scrambles[0].sequence;
        } else {
            return jsonResult.message;
        }
    } catch (error) {
        console.error('Błąd połączenia:', error);
        alert('Wystąpił błąd połączenia z serwerem.');
    }
}

// Wyświetlanie scramble na kliknięcie przycisku
scrambleButton.addEventListener('click', async () => {
    scrambleDisplay.textContent = await generateScramble("Three", 1);
});

// Generowanie scramble przy załadowaniu strony
document.addEventListener('DOMContentLoaded', async () => {
    scrambleDisplay.textContent = await generateScramble("Three", 1);
});

function getAccessToken() {
    const token = document.cookie.split('; ').find(row => row.startsWith('access_token='));
    return token ? token.split('=')[1] : null;
}

document.addEventListener('DOMContentLoaded', () => {
    const createSessionButton = document.getElementById('create-session-button');
    const sessionNameInput = document.getElementById('session-name');

    if (createSessionButton && sessionNameInput) {
        createSessionButton.addEventListener('click', async () => {
            const sessionName = sessionNameInput.value || `Sesja ${Date.now()}`;
            try {
                const response = await fetch('/new-session', {
                    method: 'POST',
                    headers: {
                        'Accept': 'application/json',
                        'Content-Type': 'application/json',
                        'Authorization': `Bearer ${getAccessToken()}`,
                    },
                    body: JSON.stringify({ name: sessionName }),
                });

                if (response.ok) {
                    alert('Nowa sesja została utworzona!');
                    sessionNameInput.value = ''; // Wyczyść pole tekstowe
                } else {
                    alert('Nie udało się utworzyć nowej sesji.');
                }
            } catch (error) {
                console.error('Błąd przy tworzeniu sesji:', error);
                alert('Wystąpił problem z połączeniem.');
            }
        });
    }
});
