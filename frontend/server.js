require('dotenv').config();
const express = require('express');
const path = require('path');
const app = express();
const flash = require('express-flash');
const session = require('express-session');
const methodOverride = require('method-override');
const ejs = require('ejs');
const cookieParser = require('cookie-parser');
const profileRoutes = require('./routes/profileRoutes');
const eventRoutes = require('./routes/eventRoutes');
const rankingRoutes = require('./routes/rankingRoutes');
const adminRoutes = require('./routes/adminRoutes');
const { getCookieByName, ensureAuthenticated, ensureNotAuthenticated } = require('./utils');

app.use(express.json());
app.use(express.static("public"));
app.set('view-engine', 'ejs');
app.use(methodOverride('_method'));
app.use(express.urlencoded({ extended: true }));
app.use(cookieParser());
app.use((_req, res, next) => {
  res.set("Cache-Control", "no-store, no-cache, must-revalidate, private");
  next();
});
app.use(express.static(path.join(__dirname, 'public')));
const apiURL = new URL("http://localhost:8080/api/v1/");

app.use('/', profileRoutes);
app.use('/', eventRoutes);
app.use('/', rankingRoutes);
app.use('/', adminRoutes);

app.get('/', async (req, res) => {
  const access_token = getCookieByName("access_token", req.cookies);
  let logged;
  if (access_token !== null) {
    const result = await getUser(req, res, access_token);
    if (result.status === 200) {
      logged = true;
    } else {
      logged = false;
    }
  } else {
    logged = false;
  }
  res.render('index.ejs', { isLoggedIn: logged, isAdmin: true })
})

async function getUser(req, res, access_token) {
  const token = "Bearer ".concat(access_token);
  const result = await fetch("http://localhost:8080/api/v1/profiles/logged", {
    method: 'GET',
    headers: {
      'Accept': 'application/json',
      'Content-Type': 'application/json',
      'Authorization': token
    }
  });
  return result;
}

app.get('/login', ensureNotAuthenticated, async (req, res) => {
  // const result = await fetch("http://localhost:8080/api/v1/auth/login", {
  //   method: 'POST',
  //   headers: {
  //     'Accept': 'application/json',
  //     'Content-Type': 'application/json'
  //   }
  // });
  
  // // const jsonResult = await result.json();
  // // console.log(jsonResult);
  // console.log(result);
  // console.log(result.status);
  res.render('login.ejs', {  });
})

app.get('/register', ensureNotAuthenticated, (req, res) => {
  res.render('register.ejs', {  });
})

app.post('/logout', async (req, res) => {
  const data = {
    refresh_token: getCookieByName('refresh_token', req.cookies)
  };

  const result = await fetch("http://localhost:8080/api/v1/auth/logout", {
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
  const result = await fetch("http://localhost:8080/api/v1/auth/login", {
    method: 'POST',
    headers: {
      'Accept': 'application/json',
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(req.body)  
  });
  
  const jsonResult = await result.json();
  
  if (result.status == 200) {
    res.cookie('access_token', jsonResult.access_token, {
      httpOnly: true,
      secure: true, 
      maxAge: 1000 * 60 * 15,
      sameSite: 'Strict'
    });

    res.cookie('refresh_token', jsonResult.refresh_token, {
      httpOnly: true,  
      secure: true,  
      maxAge: 30 * 24 * 60 * 60 * 1000,
      sameSite: 'Strict' 
    });
    
    res.send(result);
  } else {
    res.status(401).send({ message: "Invalid credentials" });
  }

})

app.post('/register', async (req, res) => {
    const result = await fetch("http://localhost:8080/api/v1/auth/register", {
      method: 'POST',
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(req.body)  
    });
    const jsonResult = await result.json();
    res.send(jsonResult);
  //dodać automatyczne logowanie po rejestracji.
})

app.use('/script.js', (req, res, next) => {
  res.setHeader('Content-Type', 'application/javascript');
  next();
});

app.listen(3000)