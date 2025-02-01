import {API_URL} from '../server.js';
import {getCookieByName, ensureAuthenticated, getUser, checkIfAdmin} from '../utils.js';
import express from 'express';

const router = express.Router();

router.get('/admin', ensureAdmin, async (req, res) => {
    const access_token = getCookieByName("access_token", req.cookies);
    if (access_token === null) {
        res.redirect('/');
    }
    try {
        const result = await fetch(API_URL + "/profiles", {
            method: 'GET',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${access_token}`
            }
        });

        if (!result.ok) {
            const errorData = await result.json();
            return res.status(result.status).json({
                error: errorData.error || 'Wystąpił błąd podczas pobierania profili',
                status: result.status
            });
        }

        const jsonResult = await result.json();
        const accounts = jsonResult.payload.accounts;
        res.render('admin.ejs', {accounts: accounts});
    } catch (error) {
        console.error('Error in admin route:', error);
        res.status(500).json({
            error: 'Wystąpił wewnętrzny błąd serwera',
            status: 500
        });
    }
});

router.get('/all-users', ensureAuthenticated, async (req, res) => {
    const access_token = getCookieByName("access_token", req.cookies);
    if (access_token === null) {
        res.redirect('/');
    }
    const result = await fetch(API_URL + "/profiles", {
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

router.post('/delete-user', ensureAuthenticated, async (req, res) => {
    const access_token = getCookieByName("access_token", req.cookies);
    if (access_token === null) {
        res.redirect('/');
    }
    try {
        const account_id = req.body.user_id;
        const result = await fetch(API_URL + `/profiles/${account_id}`, {
            method: 'DELETE',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${access_token}`
            }
        });

        if (!result.ok) {
            const errorData = await result.json();
            return res.status(result.status).json({
                error: errorData.error || 'Wystąpił błąd podczas usuwania użytkownika',
                status: result.status
            });
        }

        const jsonResult = await result.json();
        res.json(jsonResult);
    } catch (error) {
        console.error('Error in delete user route:', error);
        res.status(500).json({
            error: 'Wystąpił wewnętrzny błąd serwera',
            status: 500
        });
    }
});

async function ensureAdmin(req, res, next) {
    const access_token = getCookieByName("access_token", req.cookies);
    let isAdmin = false;
    if (access_token === null) {
        return res.redirect('/');
    }

    const result = await getUser(req, res, access_token);
    if (result.ok) {
        const jsonResult = await result.json();
        isAdmin = checkIfAdmin(jsonResult.payload.logged_account.roles);
        if (isAdmin === true) {
            return next();
        }
    }

}

export default router;
