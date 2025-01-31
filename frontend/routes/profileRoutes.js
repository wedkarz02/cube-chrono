import {API_URL} from "../server";

const express = require('express');
const router = express.Router();
const {getCookieByName, ensureAuthenticated} = require('../utils');

router.get('/myprofile', ensureAuthenticated, async (req, res) => {
    const access_token = getCookieByName("access_token", req.cookies);
    if (access_token !== null) {
        const token = "Bearer ".concat(access_token);
        const result = await fetch(API_URL + "/profiles/logged", {
            method: 'GET',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json',
                'Authorization': token
            }
        });

        if (result.status === 403) {
            res.redirect('/');
        } else {
            const jsonResult = await result.json();
            const username = jsonResult.payload.logged_account.username;
            res.render('profile.ejs', {username: username});
        }
    } else {
        res.redirect('/');
    }
});

router.put('/password', ensureAuthenticated, async (req, res) => {
    const access_token = getCookieByName("access_token", req.cookies);
    if (access_token !== null) {
        const token = "Bearer ".concat(access_token);

        try {
            const result = await fetch(API_URL + "/profiles/logged/change-password", {
                method: 'PUT',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json',
                    'Authorization': token
                },
                body: JSON.stringify(req.body)
            });

            if (result.ok) {
                res.clearCookie('access_token');
                res.clearCookie('refresh_token');
                const jsonResult = await result.json();
                res.json(jsonResult);
            } else {
                const errorResponse = await result.json();
                res.status(result.status).json(errorResponse);
            }
        } catch (error) {
            res.status(500).json({message: 'Wystąpił problem z przetwarzaniem zapytania.', error: error.message});
        }
    } else {
        res.redirect('/');
    }
});


router.put('/username', ensureAuthenticated, async (req, res) => {
    const access_token = getCookieByName("access_token", req.cookies);
    if (access_token !== null) {
        const token = "Bearer ".concat(access_token);

        try {
            const result = await fetch(API_URL + "/profiles/logged/change-username", {
                method: 'PUT',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json',
                    'Authorization': token
                },
                body: JSON.stringify(req.body)
            });

            if (result.ok) {
                const jsonResult = await result.json();
                res.json(jsonResult);
            } else {
                const errorResponse = await result.json();
                res.status(result.status).json(errorResponse);
            }
        } catch (error) {
            res.status(500).json({message: 'Wystąpił problem z przetwarzaniem zapytania.', error: error.message});
        }
    } else {
        res.redirect('/');
    }
});

router.get('/all-sessions', ensureAuthenticated, async (req, res) => {
    const access_token = getCookieByName("access_token", req.cookies);
    if (access_token !== null) {
        const token = "Bearer ".concat(access_token);
        const result = await fetch(API_URL + "/sessions", {
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

router.post('/session', ensureAuthenticated, async (req, res) => {
    const access_token = getCookieByName("access_token", req.cookies);
    if (access_token !== null) {
        const session_id = req.body.session_id;
        const token = "Bearer ".concat(access_token);
        const result = await fetch(API_URL + `/sessions/${session_id}`, {
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

module.exports = router;
