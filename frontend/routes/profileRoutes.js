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
      const result = await fetch("http://localhost:8080/api/v1/profiles", {
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

  router.put('/myprofile', ensureAuthenticated, async (req, res) => {
    const access_token = getCookieByName("access_token", req.cookies);
    if (access_token !== null) {
      const token = "Bearer ".concat(access_token);
      const result = await fetch("http://localhost:8080/api/v1/profiles", {
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
        const userID = jsonResult.logged_account._id;
        //....
      }
    } else {
      res.redirect('/');
    }
  });

})();

module.exports = router;
