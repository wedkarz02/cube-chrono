require('dotenv').config();
const express = require('express');
const path = require('path');
const app = express();
const methodOverride = require('method-override');
const cookieParser = require('cookie-parser');
const profileRoutes = require('./routes/profileRoutes');
const adminRoutes = require('./routes/adminRoutes');
const { getCookieByName, ensureAuthenticated, ensureNotAuthenticated, getUser, checkIfAdmin } = require('./utils');

app.use(express.json());
app.use(express.static('public'));
app.set('view engine', 'ejs');
app.set('views', path.join(__dirname, 'views'));
app.use(methodOverride('_method'));
app.use(express.urlencoded({ extended: true }));
app.use(cookieParser());
app.use((_req, res, next) => {
  res.set('Cache-Control', 'no-store, no-cache, must-revalidate, private');
  next();
});
app.use(express.static(path.join(__dirname, 'public')));

const API_URL = new URL('http://localhost:8080/api/v1');

// NOTE: Assuming these durations could change in the backend, it may be better to receive them with api requests
const ACCESS_TOKEN_EXPIRY_TIME = 1000 * 60 * 15;
const REFRESH_TOKEN_EXPIRY_TIME = 1000 * 60 * 60 * 24 * 30;

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
    if (result.status === 200) {
      logged = true;
      const jsonResult = await result.json();
      isAdmin = checkIfAdmin(jsonResult.payload.logged_account.roles);
    } else {
      logged = false;
    }
  } else {
    logged = false;
  }
  res.render('index.ejs', { isLoggedIn: logged, isAdmin: isAdmin })
})

app.get('/login', ensureNotAuthenticated, async (req, res) => {
  res.render('login.ejs', {  });
})

app.get('/register', ensureNotAuthenticated, (req, res) => {
  res.render('register.ejs', {  });
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
  const result = await fetch(API_URL + '/auth/login', {
    method: 'POST',
    headers: {
      'Accept': 'application/json',
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(req.body)
  });

  const jsonResult = await result.json();

  if (result.status === 200) {
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

    res.send(result);
  } else {
    res.status(401).send({ message: 'Invalid credentials' });
  }

})

app.post('/register', async (req, res) => {
    const result = await fetch(API_URL + '/auth/register', {
      method: 'POST',
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(req.body)
    });
    const jsonResult = await result.json();
    res.send(jsonResult);
})

app.post('/scrambles', async (req, res) => {
  const kind = req.body.kind;
  const count = req.body.count;

  const response = await fetch(API_URL + `/scrambles?kind=${kind}&count=${count}`, {
    method: 'GET',
    headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json'
    }
  });

  const jsonResult = await response.json();
  res.json(jsonResult);
})

app.post('/new-session', ensureAuthenticated, async (req, res) => {
  const access_token = getCookieByName('access_token', req.cookies);
  const token = 'Bearer '.concat(access_token);
  const response = await fetch(API_URL + '/sessions/empty', {
    method: 'POST',
    headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json',
        'Authorization': token
    },
    body: JSON.stringify(req.body)
  });
  const jsonResult = await response.json();
  res.json(jsonResult);
})

app.post('/add-time', ensureAuthenticated, async (req, res) => {
  const access_token = getCookieByName('access_token', req.cookies);
  const token = 'Bearer '.concat(access_token);
  const response = await fetch(`http://localhost:8080/api/v1/sessions/add-time`, {
    method: 'POST',
    headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json',
        'Authorization': token
    },
    body: JSON.stringify(req.body)
  });
  const jsonResult = await response.json();
  res.json(jsonResult);
})

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
      res.render('sessions', { isLoggedIn: false, username: null, sessions: [] });
    }
  } catch (error) {
    console.error('Błąd podczas pobierania sesji:', error);
    res.render('sessions', { isLoggedIn: false, username: null, sessions: [] });
  }
});

app.get('/session/:id', ensureAuthenticated, async (req, res) => {
  try {
    const sessionId = req.params.id;
    const response = await fetch(`http://localhost:8080/api/v1/sessions/${sessionId}`, {
      method: 'GET',
      headers: {
        'Authorization': `Bearer ${req.cookies.access_token}`,
        'Accept': 'application/json',
      },
    });

    if (response.ok) {
      const data = await response.json();
      res.render('session', { isLoggedIn: true, session: data.payload.session });
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

app.listen(3000)
