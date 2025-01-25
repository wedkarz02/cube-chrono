const express = require('express');
const router = express.Router();
const { getCookieByName, ensureAuthenticated } = require('../utils'); // Przenieś pomocnicze funkcje do osobnego pliku

// Zmieniamy require na dynamiczny import
let fetch;

(async () => {
  fetch = (await import('node-fetch')).default; // Dynamiczny import fetch

  // Przekierowanie na stronę profilu po zalogowaniu
  router.get('/myprofile', await ensureAuthenticated, async (req, res) => {
    const access_token = getCookieByName("access_token", req.cookies);
    if (access_token !== null) {
      const token = "Bearer ".concat(access_token);
      const result = await fetch("http://localhost:8080/api/v1/profiles/logged", {
        method: 'GET',
        headers: {
          'Accept': 'application/json',
          'Content-Type': 'application/json',
          'Authorization': token
        }
      });

      if (result.status == 403) {
        res.redirect('/');
      } else {
        const jsonResult = await result.json();
        const username = jsonResult.logged_account.username;
        res.render('profile.ejs', { username: username });
      }
    } else {
      res.redirect('/');
    }
  });

  router.put('/password', ensureAuthenticated, async (req, res) => {
    const access_token = getCookieByName("access_token", req.cookies);
    if (access_token !== null) {
      const data = {
        new_password: new_password,
        old_password: old_password
      };

      const token = "Bearer ".concat(access_token);
      const result = await fetch("http://localhost:8080/api/v1/profiles/logged/change-password", {
        method: 'PUT',
        headers: {
          'Accept': 'application/json',
          'Content-Type': 'application/json',
          'Authorization': token
        },
        body: JSON.stringify(data)
      });

      const jsonResult = await result.json();
      if (result.status === 200) {
                
      } else if (result.status === 400) {

      } else if (result.status === 401) {

      }
    } else {
      res.redirect('/');
    }
  });

  router.put('/username', ensureAuthenticated, async (req, res) => {
    const access_token = getCookieByName("access_token", req.cookies);
    if (access_token !== null) {
      const data = {
        username: username
      };

      const token = "Bearer ".concat(access_token);
      const result = await fetch("http://localhost:8080/api/v1/profiles/logged/change-username", {
        method: 'PUT',
        headers: {
          'Accept': 'application/json',
          'Content-Type': 'application/json',
          'Authorization': token
        },
        body: JSON.stringify(data)
      });

      const jsonResult = await result.json();
      if (result.status === 200) {
                
      } else if (result.status === 400) {

      } else if (result.status === 401) {
        
      }
    } else {
      res.redirect('/');
    }
  });

})();

module.exports = router;
