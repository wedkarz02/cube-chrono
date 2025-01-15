require('dotenv').config()
const express = require('express')
const app = express()

const flash = require('express-flash')
const session = require('express-session')
const methodOverride = require('method-override')
const ejs = require('ejs')
const cookieParser = require('cookie-parser');

app.use(express.json());
app.use(express.static("public"));
app.set('view-engine', 'ejs')
app.use(methodOverride('_method'))
app.use(express.urlencoded({ extended: true }));
app.use(cookieParser());
app.use((_req, res, next) => {
  res.set("Cache-Control", "no-store, no-cache, must-revalidate, private");
  next();
});

const apiURL = new URL("http://localhost:8080/api/v1/");

app.get('/', ensureAuthenticated, (req, res) => {
  res.render('index.ejs', {})
})

app.get('/myprofile', ensureAuthenticated, (req, res) => {
  
  res.render('profile.ejs', {})
})

app.get('/login', ensureNotAuthenticated, (req, res) => {
  res.render('login.ejs', {  });
})

app.get('/register', ensureNotAuthenticated, (req, res) => {
  res.render('register.ejs', {  });
})

app.post('/logout', async (req, res) => {
  res.clearCookie('pass');
  res.clearCookie('access_token');
  res.clearCookie('refresh_token');

  const result = await fetch("http://localhost:8080/api/v1/auth/logout", {
    method: 'POST',
    headers: {
      'Accept': 'application/json',
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(getCookieByName('refresh_token'))
  });
  
  const jsonResult = await result.json();
  res.end()
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
  createCookies(jsonResult, res);
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

// app.get('/debug', (req, res) => {
//   value = req.cookies[1];
//   console.log(value);
// })

function createCookies(jsonResult, res) {
  if (jsonResult.access_token && jsonResult.refresh_token) {
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
    
    res.send(jsonResult);
  } else {
    res.status(401).send({ message: "Invalid credentials" });
  }
}

function ensureAuthenticated(req, res, next) {
  // let pass = getCookieByName("pass", req.cookies);
  // if(pass !== null) {
  //   res.cookie('pass', 2, {
  //     httpOnly: true,
  //     secure: true, 
  //     maxAge: 1000 * 60 * 20,
  //     sameSite: 'Strict'
  //   });
    
  //   return next();
  // }

  let acces_token = getCookieByName("access_token", req.cookies);
  if (acces_token !== null) { //dodać zabezpieczenia
    res.cookie('pass', 1, {
      httpOnly: true,
      secure: true, 
      maxAge: 1000 * 60 * 20,
      sameSite: 'Strict'
    });
    
    return next();
  }

  let refresh_token = getCookieByName("refresh_token", req.cookies);
  if (refresh_token !== null) { //dodać zabezpieczenia
    res.cookie('pass', 3, {
      httpOnly: true,
      secure: true, 
      maxAge: 1000 * 60 * 20,
      sameSite: 'Strict'
    });
    
    return next();
  }

  res.render('index.ejs') //strona dla gościa
}

function ensureNotAuthenticated(req, res, next) {
  let pass = getCookieByName("pass", req.cookies);
  let acces_token = getCookieByName("access_token", req.cookies);
  let refresh_token = getCookieByName("refresh_token", req.cookies);

  if (pass === null && acces_token === null && refresh_token === null) {
    return next();
  }
  res.redirect('/');
}

function getCookieByValue(searchValue, cookies) {
  for (const [key, value] of Object.entries(cookies)) {
    if (value === searchValue) {
      return value;
    }
  }
  return null;
}

function getCookieByName(searchKey, cookies) {
  for (const [key, value] of Object.entries(cookies)) {
    if (key === searchKey) {
      return value;
    }
  }
  return null;
}


// app.get('/login', checkNotAuthenticated, (req, res) => {
//   res.render('login.ejs')
// })

// app.post('/login', checkNotAuthenticated, passport.authenticate('local', {
//   successRedirect: '/',
//   failureRedirect: '/login',
//   failureFlash: true
// }))

// app.get('/register', checkNotAuthenticated, (req, res) => {
//   res.render('register.ejs')
// })

// app.post('/register', checkNotAuthenticated, async (req, res, next) => {
//   try {
//     const hashedPassword = await bcrypt.hash(req.body.password, 10);
//     const newUser = {
//       id: Date.now().toString(),
//       name: req.body.name,
//       email: req.body.email,
//       password: hashedPassword
//     };

//     users.push(newUser);

//     req.login(newUser, (err) => {
//       if (err) {
//         return next(err);
//       }
//       return res.redirect('/');
//     });
//   } catch {
//     res.redirect('/register');
//   }
// });


// app.delete('/logout', (req, res, next) => {
//   req.logOut((err) => {
//     if (err) {
//       return next(err);
//     }

//     res.redirect('/login');
//   });
// });

// function checkAuthenticated(req, res, next) {
//   if (req.isAuthenticated()) {
//     return next()
//   }

//   res.redirect('/login')
// }

// function checkNotAuthenticated(req, res, next) {
//   if (req.isAuthenticated()) {
//     return res.redirect('/')
//   }
//   next()
// }

app.listen(3000)