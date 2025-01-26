const express = require('express');
const router = express.Router();
const { getCookieByName, ensureAuthenticated } = require('../utils'); // Przenieś pomocnicze funkcje do osobnego pliku

// Zmieniamy require na dynamiczny import
let fetch;

(async () => {
  fetch = (await import('node-fetch')).default; // Dynamiczny import fetch

  router.get('/events', async (req, res) => {
    const data = {
      page: 1,
      limit: 5
    };
  
    const result = await fetch("http://localhost:8080/api/v1/events", {
      method: 'GET',
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json'
      }
      //body: JSON.stringify(data)
    });
  
    if (result.status === 200) {
      const jsonResult = await result.json();
      res.render('events.ejs', { eventsList: events });
    } else {
      res.render('events.ejs', { eventsList: null });
    }
  
  })

})();

module.exports = router;
