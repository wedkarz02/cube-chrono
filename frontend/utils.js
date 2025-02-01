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

    if (!access_token && !req.cookies.refresh_token) {
        return res.status(401).json({
            error: 'Unauthorized',
            status: 401
        });
    }

    if (!access_token) {
        const refresh = getCookieByName('refresh_token', req.cookies);
        const result = await fetch(API_URL + '/auth/refresh', {
            method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ refresh_token: refresh })
        });

        if (!result.ok) {
            let error = new Error(`Refresh token error: ${result.status}`);
            console.error('Authentication error:', error);
            res.status(result.status).json({
                error: 'Invalid credentials',
                status: result.status
            });
        }

        const jsonResult = await result.json();
        createCookie('access_token', jsonResult.payload.access_token, ACCESS_TOKEN_EXPIRY_TIME, res);
        access_token = jsonResult.payload.access_token;
    }

    req.accessToken = access_token;
    next();
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

        if (result.ok) {
            const jsonResult = await result.json();
            createCookie('access_token', jsonResult.payload.access_token, ACCESS_TOKEN_EXPIRY_TIME, res);
        }
    }

    return res.redirect('/')

}

export async function getUser(req, res, access_token) {
    try {
        const response = await fetch(API_URL + '/profiles/logged', {
            method: 'GET',
            headers: {
                'Authorization': `Bearer ${access_token}`,
                'Accept': 'application/json'
            }
        });

        if (!response.ok) {
            const errorData = await response.json();
            return res.status(response.status).json({
                error: errorData.error || 'Wystąpił błąd podczas pobierania profilu',
                status: response.status
            });
        }
        return response
    } catch (error) {
        console.error('Get profile error:', error);
        res.status(500).json({ error: 'Internal server error' });
    }
}

export function checkIfAdmin(roles) {
    return roles.includes('Admin');
}
