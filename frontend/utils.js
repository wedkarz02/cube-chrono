import {API_URL, ACCESS_TOKEN_EXPIRY_TIME} from './server.js';

export function getCookieByName(searchKey, cookies) {
    for (const [key, value] of Object.entries(cookies)) {
        if (key === searchKey) {
            return value;
        }
    }
    return null;
}

export function getCookieByValue(searchValue, cookies) {
    for (const [, value] of Object.entries(cookies)) {
        if (value === searchValue) {
            return value;
        }
    }
    return null;
}

function createCookie(name, value, age, res) {
    res.cookie(name, value, {
        httpOnly: true,
        secure: true,
        maxAge: age,
        sameSite: 'Strict'
    });
}

export async function ensureAuthenticated(req, res, next) {
    let access_token = getCookieByName('access_token', req.cookies);
    if (access_token !== null) {
        return next();
    }

    let refresh = getCookieByName('refresh_token', req.cookies);
    if (refresh !== null) {
        const data = {
            refresh_token: refresh
        };

        const result = await fetch(API_URL + '/auth/refresh', {
            method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(data)
        });

        if (result.status === 200) {
            const jsonResult = await result.json();
            createCookie('access_token', jsonResult.payload.access_token, ACCESS_TOKEN_EXPIRY_TIME, res);
        }
    }

    res.redirect('/');
}

export async function ensureNotAuthenticated(req, res, next) {
    let access_token = getCookieByName('access_token', req.cookies);
    let refresh = getCookieByName('refresh_token', req.cookies);

    if (access_token === null && refresh === null) {
        return next();
    } else if (refresh !== null && access_token === null) {
        const data = {
            refresh_token: refresh
        };

        const result = await fetch(API_URL + '/auth/refresh', {
            method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(data)
        });

        if (result.status === 200) {
            const jsonResult = await result.json();
            createCookie('access_token', jsonResult.payload.access_token, ACCESS_TOKEN_EXPIRY_TIME, res);
        }
    }

    return res.redirect('/')

}

export async function getUser(req, res, access_token) {
    const token = 'Bearer '.concat(access_token);
    return await fetch(API_URL + '/profiles/logged', {
        method: 'GET',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json',
            'Authorization': token
        }
    });
}

export function checkIfAdmin(roles) {
    return roles.includes('Admin');
}
