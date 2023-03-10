import { get } from "svelte/store";
import { themeMode, ThemeMode, user } from "../stores";

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
export type GeneratorDescription = {
	id: string,
	name: string,
	description: string,
	dateCreated: Date,
	generatorType: string,
	generatorTypeCode: string,
	image: string
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
		themeMode.set(ThemeMode.UseDeviceTheme);
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
			themeMode.set(await this.getUserPreferredTheme());
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

	async saveGenerator(type: string, name: string, description: string, generatorSettings: { [key: string]: any}) {
		let body: string = JSON.stringify({ name, description, generatorSettings });
		return await this.#accessAPI("generator/" + type + "/save", {
			method: "POST",
			headers: {
				"Content-type": "application/json"
			},
			body: body
		});
	}

	async deleteGenerator(type: string, id: string) {
		return await this.#accessAPI("generator/" + type + "/" + id + "/" + "/delete", {
			method: "POST"
		});
	}
	
	async saveGeneratorChanges(type: string, id: string, name: string, description: string, generatorSettings: { [key: string]: any}) {
		let body: string = JSON.stringify({ name, description, generatorSettings });
		await this.#accessAPI("generator/" + type + "/" + id + "/save", {
			method: "POST",
			headers: {
				"Content-type": "application/json"
			},
			body: body
		});
	}

	async editGeneratorDescription(id: string, name: string, description: string) {
		let body: string = JSON.stringify({ name, description });
		await this.#accessAPI("generatorDescription/" + id, {
			method: "POST",
			headers: {
				"Content-type": "application/json"
			},
			body: body
		});
	}

	async getMyGenerators(): Promise<GeneratorDescription[]> {
		let response = await this.#accessAPI("myGenerators", {
			method: "GET"
		});
		let res: any[] = await response.json();
		
		const base64_arraybuffer = async (data: any) => {
			const base64url: any = await new Promise((r) => {
				const reader = new FileReader()
				reader.onload = () => r(reader.result)
				reader.readAsDataURL(new Blob([data]))
			})
			return base64url.split(",", 2)[1]
		}
		
		let generators = Promise.all(res.map(async generator => {
			return {
				...generator,
				dateCreated: new Date(generator.dateCreated),
				image: await base64_arraybuffer(new Uint8Array(generator.image))
			};
		}));
		return generators;
	}
	async getMyGenerator(name: string, id: string) {
		let response = await this.#accessAPI("generator/" + name + "/" + id, {
			method: "GET"
		});
		return await response.json();
	}
	async getGenerators() {
		let response = await this.#accessAPI("generators", {
			method: "GET"
		});
		return await response.json();
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
	async setUserPreferredTheme(preferredTheme: ThemeMode) {
		themeMode.set(preferredTheme);
		let u = get(user);
		if (u.isAuthorised()) {
			let response = await this.#accessAPI("profile/theme", {
				method: "POST",
				body: preferredTheme.toString()
			});
			if (!response.ok) {
				throw "Failed to save preferred theme";
			}
		}
	}
	async getUserPreferredTheme() {
		let response = await this.#accessAPI("profile/theme", { method: "GET" });
		if (response.ok) {
			return parseInt(await response.text()) as ThemeMode;
		} else {
			throw "Failed to get preferrred theme";
		}
	}
}

export default new BackendService();