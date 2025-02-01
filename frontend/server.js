import express from 'express';
import path from 'path';
import methodOverride from 'method-override';
import cookieParser from 'cookie-parser';

import profileRoutes from './routes/profileRoutes.js';
import adminRoutes from './routes/adminRoutes.js';
import {getCookieByName, ensureAuthenticated, ensureNotAuthenticated, getUser, checkIfAdmin} from './utils.js';

const __dirname = import.meta.dirname;

const app = express();
app.use(express.json());
app.use(express.static('public'));
app.set('view engine', 'ejs');
app.set('views', path.join(__dirname, 'views'));
app.use(methodOverride('_method'));
app.use(express.urlencoded({extended: true}));
app.use(cookieParser());
app.use((_req, res, next) => {
    res.set('Cache-Control', 'no-store, no-cache, must-revalidate, private');
    next();
});
app.use(express.static(path.join(__dirname, 'public')));

export const API_URL = new URL('http://localhost:8080/api/v1');

// NOTE: Assuming these durations could change in the backend, it may be better to receive them with api requests
export const ACCESS_TOKEN_EXPIRY_TIME = 1000 * 60 * 15;
export const REFRESH_TOKEN_EXPIRY_TIME = 1000 * 60 * 60 * 24 * 30;
// NOTE: If I were to work more on this, I would apply TypeScript everywhere for sure

app.use((req, res, next) => {
    res.status(404);

    if (req.accepts('html')) {
        res.render('404', { url: req.url });
        return;
    }

    if (req.accepts('json')) {
        res.json({ error: 'Not found' });
        return;
    }

    res.type('txt').send('Not found');
});

app.use('/', profileRoutes);
app.use('/', adminRoutes);
app.use('/script.js', (req, res, next) => {
    res.setHeader('Content-Type', 'application/javascript');
    next();
});

app.get('/', async (req, res) => {
    const access_token = getCookieByName('access_token', req.cookies);
    let isAdmin = false;
    let logged;
    if (access_token !== null) {
        const result = await getUser(req, res, access_token);
        if (result.ok) {
            logged = true;
            const jsonResult = await result.json();
            isAdmin = checkIfAdmin(jsonResult.payload.logged_account.roles);
        } else {
            logged = false;
        }
    } else {
        logged = false;
    }
    res.render('index.ejs', {isLoggedIn: logged, isAdmin: isAdmin})
})

app.get('/login', ensureNotAuthenticated, async (req, res) => {
    res.render('login.ejs', {});
})

app.get('/register', ensureNotAuthenticated, (req, res) => {
    res.render('register.ejs', {});
})

app.post('/logout', async (req, res) => {
    const data = {
        refresh_token: getCookieByName('refresh_token', req.cookies)
    };

    await fetch(API_URL + '/auth/logout', {
        method: 'POST',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(data)
    });

    res.clearCookie('access_token');
    res.clearCookie('refresh_token');
    res.redirect('/');
})

app.post('/login', async (req, res) => {
    try {
        const result = await fetch(API_URL + '/auth/login', {
            method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(req.body)
        });

        if (!result.ok) {
            const errorData = await result.json();
            return res.status(result.status).json({
                error: errorData.error || 'Wystąpił błąd podczas logowania',
                status: result.status
            });
        }

        const jsonResult = await result.json();
        res.cookie('access_token', jsonResult.payload.access_token, {
            httpOnly: true,
            secure: true,
            maxAge: ACCESS_TOKEN_EXPIRY_TIME,
            sameSite: 'Strict'
        });
        res.cookie('refresh_token', jsonResult.payload.refresh_token, {
            httpOnly: true,
            secure: true,
            maxAge: REFRESH_TOKEN_EXPIRY_TIME,
            sameSite: 'Strict'
        });
        return res.json(jsonResult);
    } catch (error) {
        console.error('Login error:', error);
        res.status(500).json({error: 'Internal server error'});
    }
})

app.post('/register', async (req, res) => {
    try {
        const response = await fetch(API_URL + '/auth/register', {
            method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(req.body)
        });

        if (!response.ok) {
            const errorData = await response.json();
            return res.status(response.status).json({
                error: errorData.error || 'Wystąpił błąd podczas rejestracji.',
                status: response.status
            });
        }

        const jsonResult = await response.json();
        res.json(jsonResult);
    } catch (error) {
        console.error('Error in register endpoint:', error);
        res.status(500).json({
            error: 'Wystąpił wewnętrzny błąd serwera.',
            status: 500
        });
    }
});

app.post('/scrambles', async (req, res) => {
    try {
        const kind = req.body.kind;
        const count = req.body.count;

        if (!kind || !count) {
            return res.status(400).json({
                error: 'Nieprawidłowe parametry. Wymagane: kind i count.',
                status: 400
            });
        }

        const response = await fetch(API_URL + `/scrambles?kind=${kind}&count=${count}`, {
            method: 'GET',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            }
        });

        if (!response.ok) {
            const errorData = await response.json();
            return res.status(response.status).json({
                error: errorData.error || 'Wystąpił błąd podczas generowania scramble.',
                status: response.status
            });
        }

        const jsonResult = await response.json();
        res.json(jsonResult);
    } catch (error) {
        console.error('Error in scramble endpoint:', error);
        res.status(500).json({
            error: 'Wystąpił wewnętrzny błąd serwera.',
            status: 500
        });
    }
});

app.post('/new-session', ensureAuthenticated, async (req, res) => {
    try {
        const response = await fetch(API_URL + '/sessions/empty', {
            method: 'POST',
            headers: {
                'Authorization': `Bearer ${req.accessToken}`,
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(req.body)
        });

        if (!response.ok) {
            const errorData = await response.json();
            return res.status(response.status).json({
                error: errorData.error || 'Wystąpił błąd podczas tworzenia sesji wyników',
                status: response.status
            });
        }

        const data = await response.json();
        res.json(data);
    } catch (error) {
        console.error('Create session error:', error);
        res.status(500).json({ error: 'Internal server error' });
    }
})

app.post('/add-time', ensureAuthenticated, async (req, res) => {
    try {
        const access_token = getCookieByName('access_token', req.cookies);
        if (!access_token) {
            return res.status(401).json({
                error: 'Nieprawidłowy token dostępu',
                status: 401
            });
        }

        const { session_id, time } = req.body;

        if (!session_id || !time) {
            return res.status(400).json({
                error: 'Nieprawidłowe dane sesji lub czasu',
                status: 400
            });
        }

        const response = await fetch(API_URL + '/sessions/add-time', {
            method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${access_token}`
            },
            body: JSON.stringify(req.body)
        });

        if (!response.ok) {
            const errorData = await response.json();
            return res.status(response.status).json({
                error: errorData.error || 'Wystąpił błąd podczas zapisywania czasu',
                status: response.status
            });
        }

        const jsonResult = await response.json();
        res.json(jsonResult);
    } catch (error) {
        console.error('Error in add-time endpoint:', error);
        if (error.name === 'TokenExpiredError') {
            return res.status(401).json({
                error: 'Sesja wygasła. Zaloguj się ponownie.',
                status: 401
            });
        }
        res.status(500).json({
            error: 'Wystąpił wewnętrzny błąd serwera',
            status: 500
        });
    }
});

app.get('/sessions', ensureAuthenticated, async (req, res) => {
    try {
        const response = await fetch(API_URL + '/sessions', {
            method: 'GET',
            headers: {
                'Authorization': `Bearer ${req.cookies.access_token}`,
                'Accept': 'application/json',
            },
        });

        if (response.ok) {
            const data = await response.json();
            res.render('sessions', {
                isLoggedIn: true,
                username: req.cookies.username || 'Nieznany Użytkownik',
                sessions: data.payload.sessions,
            });
        } else {
            res.render('sessions', {isLoggedIn: false, username: null, sessions: []});
        }
    } catch (error) {
        console.error('Błąd podczas pobierania sesji:', error);
        res.render('sessions', {isLoggedIn: false, username: null, sessions: []});
    }
});

app.get('/session/:id', ensureAuthenticated, async (req, res) => {
    try {
        const response = await fetch(API_URL + `/sessions/${req.params.id}`, {
            method: 'GET',
            headers: {
                'Authorization': `Bearer ${req.cookies.access_token}`,
                'Accept': 'application/json',
            },
        });

        if (response.ok) {
            const data = await response.json();
            res.render('session', {isLoggedIn: true, session: data.payload.session});
        } else if (response.status === 404) {
            res.status(404).send('Session not found');
        } else {
            res.status(response.status).send('Error fetching session details');
        }
    } catch (error) {
        console.error('Error fetching session details:', error);
        res.status(500).send('Internal Server Error');
    }
});

const PORT = 3000;
app.listen(PORT)
console.log(`Server is listening on port http://localhost:${PORT}`)
