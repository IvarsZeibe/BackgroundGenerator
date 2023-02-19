import { user } from "../stores";

export type UserData = {
	id: number,
	email: string,
	password: string,
	isAdmin: boolean
};
class BackendService {
	async login(email: string, password: string): Promise<{ isOk: boolean, message: string }> {
		let body: string = JSON.stringify({email: email, password: password});
		let respone = await fetch("http://localhost:8000/api/login", {
			method: "POST",
			headers: {
				"Content-Type": "application/json"
			},
			body: body
		});
		if (respone.ok) {
			this.setProfile();
			return { isOk: true, message: "" };
		} else {
			return { isOk: false, message: await respone.text() };
		}
		
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
	
	async getUsers(): Promise<UserData[]> {
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
				u.setToAuthorised(res.id, res.email, res.isAdmin);
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
	}

	async setUserData(id: number, newId: number, email: string, password: string, isAdmin: boolean) {
		let body = JSON.stringify({ id: newId, email, password, isAdmin });
		return await fetch("http://localhost:8000/api/users/" + id.toString(), {
			method: "POST",
			headers: {
				"Content-type": "application/json"
			},
			body
		})
		.then(response => {
			if (response.ok) {
				return {};
			} else {
				return response.json();
			}
		});
	}
}

export default new BackendService();