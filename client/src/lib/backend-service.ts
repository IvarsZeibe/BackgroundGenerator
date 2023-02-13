import { User, user } from "../stores";

class BackendService {
    async test() {
        return fetch("http://localhost:8000");
    }
    async login(email: string, password: string): Promise<{ isOk: boolean, message: string }> {
        let body: string = JSON.stringify({email: email, password: password});
        return fetch("http://localhost:8000/api/login", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: body
        }).then(async res => {
            if (res.ok) {
                this.setProfile();
                return { isOk: true, message: "" };
            } else {
                return { isOk: false, message: await res.text() };
            }
        });
        
    }
    
    async register(email: string, password: string): Promise<{ isOk: boolean, message: string }> {
        let body: string = JSON.stringify({email: email, password: password});
        return fetch("http://localhost:8000/api/register", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: body
        }).then(async res => {
            if (res.ok) {
                this.setProfile();
                return { isOk: true, message: "" };
            } else {
                return { isOk: false, message: await res.text() };
            }
        });
    }

    async logout() {

        fetch("http://localhost:8000/api/logout", {
            method: "POST"
        });
        user.update(u => {
            u.setToGuest();
            return u;
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
            user.update(u => {
                u.setToAuthorised(res.id, res.email, res.is_admin);
                return u;
            });
        })
        .catch(_ => {
            user.update(u => {
                u.setToGuest();
                return u;
            });
        });
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