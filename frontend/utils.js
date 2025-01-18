function getCookieByName(searchKey, cookies) {
    for (const [key, value] of Object.entries(cookies)) {
      if (key === searchKey) {
        return value;
      }
    }
    return null;
  }
  
  async function ensureAuthenticated(req, res, next) {
    let access_token = getCookieByName("access_token", req.cookies);
    if (access_token !== null) {
      return next();
    }
  
    let refresh = getCookieByName("refresh_token", req.cookies);
    if (refresh !== null) {
      return next();
    }
  
    res.redirect('/');
  }
  
  module.exports = { getCookieByName, ensureAuthenticated };  



      // const data = {
    //   refresh_token: refresh
    // };

    // const result = await fetch("http://localhost:8080/api/v1/auth/refresh", {
    //   method: 'POST',
    //   headers: {
    //     'Accept': 'application/json',
    //     'Content-Type': 'application/json'
    //   },
    //   body: JSON.stringify(data)  
    // });
    
    // console.log("RESULT");
    // console.log(result);
    // const jsonResult = await result.json();
    // console.log("JSONRESULT");
    // console.log(jsonResult);
    // createCookie("access_token", value, 1000 * 60 * 15);