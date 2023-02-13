import { User, user } from "../stores";

class BackendService {
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
        return fetch("http://localhost:8000/api/register", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: body
        });
    }

    async logout() {

        fetch("http://localhost:8000/api/logout", {
            method: "POST"
        });
        user.set(new User("Guest"));
    }
    
    async get_users() {
        return await fetch("http://localhost:8000/api/users", {
            method: "GET",
            headers: {
                "Content-type": "application/json"
            }
        }).then(response => response.json());
    }

    async getProfile() {
        return await fetch("http://localhost:8000/api/profile", {
            method: "GET",
            headers: {
                "Content-type": "application/json"
            }
        }).then(response => response.json());
    }

    async setProfile() {
        this.getProfile()
        .then(res => {
            user.set(new User({id: res.id, email: res.email, isAdmin: res.is_admin}));
        })
        .catch(err => user.set(new User("Guest")));
    }

    async generate(name: string, settings: Object) {
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