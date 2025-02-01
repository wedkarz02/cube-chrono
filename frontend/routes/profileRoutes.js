import {API_URL} from '../server.js';
import {getCookieByName, ensureAuthenticated} from '../utils.js';
import express from 'express';

const router = express.Router();

router.get('/myprofile', ensureAuthenticated, async (req, res) => {
    const access_token = getCookieByName("access_token", req.cookies);
    if (access_token === null) {
        res.redirect('/');
    }
    const result = await fetch(API_URL + "/profiles/logged", {
        method: 'GET',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${access_token}`
        }
    });

    if (!result.ok) {
        res.redirect('/');
    } else {
        const jsonResult = await result.json();
        const username = jsonResult.payload.logged_account.username;
        res.render('profile.ejs', {username: username});
    }
});

router.put('/password', ensureAuthenticated, async (req, res) => {
    const access_token = getCookieByName("access_token", req.cookies);
    if (access_token === null) {
        res.redirect('/');
    }
    try {
        const result = await fetch(API_URL + "/profiles/logged/change-password", {
            method: 'PUT',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${access_token}`
            },
            body: JSON.stringify(req.body)
        });

        if (!result.ok) {
            const errorData = await result.json();
            return res.status(result.status).json({
                error: errorData.error || 'Wystąpił błąd podczas zmiany hasła',
                status: result.status
            });
        }

        const jsonResult = await result.json();
        res.clearCookie('access_token');
        res.clearCookie('refresh_token');
        res.json(jsonResult);
    } catch (error) {
        console.error('Error in password change route:', error);
        res.status(500).json({
            error: 'Wystąpił wewnętrzny błąd serwera',
            status: 500
        });
    }
});


router.put('/username', ensureAuthenticated, async (req, res) => {
    const access_token = getCookieByName("access_token", req.cookies);
    if (access_token === null) {
        res.redirect('/');
    }
    try {
        const result = await fetch(API_URL + "/profiles/logged/change-username", {
            method: 'PUT',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${access_token}`
            },
            body: JSON.stringify(req.body)
        });

        if (!result.ok) {
            const errorData = await result.json();
            return res.status(result.status).json({
                error: errorData.error || 'Wystąpił błąd podczas zmiany nazwy użytkownika',
                status: result.status
            });
        }

        const jsonResult = await result.json();
        res.json(jsonResult);
    } catch (error) {
        console.error('Error in username change route:', error);
        res.status(500).json({
            error: 'Wystąpił wewnętrzny błąd serwera',
            status: 500
        });
    }
});

router.get('/all-sessions', ensureAuthenticated, async (req, res) => {
    const access_token = getCookieByName("access_token", req.cookies);
    if (access_token === null) {
        res.redirect('/');
    }
    const result = await fetch(API_URL + "/sessions", {
        method: 'GET',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${access_token}`
        }
    });

    const jsonResult = await result.json();
    res.json(jsonResult);
});

router.post('/session', ensureAuthenticated, async (req, res) => {
    const access_token = getCookieByName("access_token", req.cookies);
    if (access_token === null) {
        res.redirect('/');
    }
    const session_id = req.body.session_id;
    const result = await fetch(API_URL + `/sessions/${session_id}`, {
        method: 'GET',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${access_token}`
        }
    });

    const jsonResult = await result.json();
    res.json(jsonResult);
});

export default router;
