class BackendService {
    // async requestFromAPI(method, to, data = null, withAuth = true, returnType = "none") {
    //     if (withAuth && !AuthenticationService.isSignedIn()) {
    //         AuthenticationService.signOut();
    //         return;
    //     }
    //     let bearer = 'Bearer ' + sessionStorage.getItem('jwtToken');
    //     let headers = {};
    //     let body = null;
    //     if (withAuth) {
    //         headers["Authorization"] = bearer;
    //     }
    //     if (data !== null) {
    //         headers["Content-Type"] = "application/json";
    //         body = JSON.stringify(data); 
    //     }
    //     return await fetch(url + to, {
    //         method: method,
    //         mode: 'cors',
    //         cache: 'no-cache',
    //         credentials: 'same-origin',
    //         headers: headers,
    //         body: body
    //     })
    //     .then(response => {
    //         if (response.ok) {
    //             if (method === "GET") {
    //                 return response.json();
    //             } else {
    //                 return response.text();
    //             }
    //         } else {
    //             if (returnType == "json") {
    //                 return response.json().then(res => {throw res});
    //             } else if (returnType == "text") {
    //                 return response.text().then(res => {throw res});
    //             } else {
    //                 throw Error(response.statusText);
    //             }
    //         }
    //     });
    // }
    // async login(usename: String, password: String) {
    //     fetch
    // }
    async test() {
        return fetch("http://localhost:8000");
    }
    async login(email: string, password: string) {
        let body: string = JSON.stringify({email: email, password: password});
        return await fetch("http://localhost:8000/api/login", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: body
        });
    }
    
    async register(email: string, password: string) {
        let body: string = JSON.stringify({email: email, password: password});
        console.log(body);
        return fetch("http://localhost:8000/register", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: body
        });
    }
    
    async get_users() {
        return await fetch("http://localhost:8000/api/users", {
            method: "GET",
            headers: {
                "Content-type": "application/json"
            }
        }).then(response => response.json());
    }

    async generate(name: string, settings) {
        let body: string = JSON.stringify(settings);
        return await fetch("http://localhost:8000/api/generator/" + name, {
            method: "POST",
            headers: {
                "Content-type": "application/json"
            },
            body: body
        })
        .then(response => response.blob());
        // return image;
    }
}

export default new BackendService();