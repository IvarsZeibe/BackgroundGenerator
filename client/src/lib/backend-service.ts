import { user } from "../stores";

export type UserData = {
	id: number,
	email: string,
	password: string,
	isAdmin: boolean
};
type VisibleUserData = {
	id: number,
	email: string,
	isAdmin: boolean
}
class BackendService {
	async #accessAPI(path: string, settings: RequestInit | undefined) {
		try {
			return await fetch("http://localhost:8000/api/" + path, settings);
		} catch (e) {
			throw "network error";
		}
	}
	async login(email: string, password: string): Promise<{ isOk: boolean, message: string }> {
		let body: string = JSON.stringify({email: email, password: password});
		let response = await this.#accessAPI("login", {
			method: "POST",
			headers: { "Content-Type": "application/json" },
			body: body
		});
		
		if (response.ok) {
			this.setProfile();
			return { isOk: true, message: "" };
		} else {
			return { isOk: false, message: await response.text() };
		}
	}
	
	async register(email: string, password: string): Promise<{ isOk: boolean, message: string }> {
		let body: string = JSON.stringify({email: email, password: password});
		let response = await this.#accessAPI("register", {
			method: "POST",
			headers: { "Content-Type": "application/json" },
			body: body
		});
		if (response.ok) {
			this.setProfile();
			return { isOk: true, message: "" };
		} else {
			return { isOk: false, message: await response.text() };
		}
	}

	async logout() {
		this.#accessAPI("logout", { method: "POST" });
		user.update(u => {
			u.setToGuest();
			return u;
		});
	}
	
	async getUsers(): Promise<UserData[]> {
		let response = await this.#accessAPI("users", {
			method: "GET",
			headers: { "Content-type": "application/json" }
		});
		if (response.ok) {
			return response.json();
		} else {
			return [];
		}
	}

	async getProfile(): Promise<VisibleUserData | null> {
		try {
			let response = await this.#accessAPI("profile", {
				method: "GET",
				headers: { "Content-type": "application/json" }
			});
			if (response.ok) {
				return response.json();
			} else {
				return null;
			}
		} catch (e) {
			return null;
		}
	}

	async setProfile() {
		let profile = await this.getProfile();
		if (profile != null) {
			let p = profile; // helps typeScript infer the type of profile
			user.update(u => {
				u.setToAuthorised(p.id, p.email, p.isAdmin);
				return u;
			});
		} else {
			user.update(u => {
				u.setToGuest();
				return u;
			});
		}
	}

	async generate(name: string, settings: Object) {
		let body: string = JSON.stringify(settings);
		let response = await this.#accessAPI("generator/" + name, {
			method: "POST",
			headers: {
				"Content-type": "application/json"
			},
			body: body
		})
		if (response.ok) {
			return response.blob();
		} else {
			throw "Image generation failed";
		}
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