const express = require('express');
const router = express.Router();
const { getCookieByName, ensureAuthenticated } = require('../utils'); // Przenieś pomocnicze funkcje do osobnego pliku

// Zmieniamy require na dynamiczny import
let fetch;

(async () => {
  fetch = (await import('node-fetch')).default; // Dynamiczny import fetch

  router.get('/admin', (req, res) => {
    res.render('admin.ejs');
  })

})();

module.exports = router;
