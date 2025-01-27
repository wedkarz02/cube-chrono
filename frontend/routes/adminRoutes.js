const express = require('express');
const router = express.Router();
const { getCookieByName, ensureAuthenticated, getUser, checkIfAdmin } = require('../utils');

let fetch;

(async () => {
  fetch = (await import('node-fetch')).default;

  router.get('/admin', ensureAdmin, async (req, res) => {
    const access_token = getCookieByName("access_token", req.cookies);
    if (access_token !== null) {
      try {
      const token = "Bearer ".concat(access_token);
      const result = await fetch("http://localhost:8080/api/v1/profiles", {
        method: 'GET',
        headers: {
          'Accept': 'application/json',
          'Content-Type': 'application/json',
          'Authorization': token
        }
      });

      const jsonResult = await result.json();
      const accounts = jsonResult.payload.accounts;
      res.render('admin.ejs', { accounts: accounts });
      } catch (e) {
        console.error(e);
        res.redirect('/');
      }
    }
  })

  router.get('/all-users', await ensureAuthenticated, async (req, res) => {
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

      const jsonResult = await result.json();
      res.json(jsonResult);
    } else {
      res.redirect('/');
    }
  });

  router.post('/delete-user', await ensureAuthenticated, async (req, res) => {
    const access_token = getCookieByName("access_token", req.cookies);
    if (access_token !== null) {
      const token = "Bearer ".concat(access_token);
      const account_id = req.body.user_id;
      const result = await fetch(`http://localhost:8080/api/v1/profiles/${account_id}`, {
        method: 'DELETE',
        headers: {
          'Accept': 'application/json',
          'Content-Type': 'application/json',
          'Authorization': token
        }
      });

      const jsonResult = await result.json();

      res.json(jsonResult);
    } else {
      res.redirect('/');
    }
  });

  async function ensureAdmin(req, res, next) {
    const access_token = getCookieByName("access_token", req.cookies);
    let isAdmin = false;
    if (access_token !== null) {
      const result = await getUser(req, res, access_token);
      if (result.status === 200) {
        const jsonResult = await result.json();
        isAdmin = checkIfAdmin(jsonResult.payload.logged_account.roles);
        if (isAdmin === true) {
          return next();
        }
      }
    }
    return res.redirect('/');
  }

})();

module.exports = router;
