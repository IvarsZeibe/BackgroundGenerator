import { writable, type Writable } from 'svelte/store';

type UserData = { id: number, email: string, isAdmin: boolean };

export class User {
	#role: "Loading" | "Guest" | "Authorised";
	#data: {id: number, email: string, isAdmin: boolean} | null;
	constructor() {
		this.#role = "Loading";
		this.#data = null;
	}
	isLoading() {
		return this.#role == "Loading";
	}
	isGuest() {
		return this.#role == "Guest";
	}
	isAuthorised() {
		return this.#role == "Authorised";
	}
	getAuthorisedUserData(): UserData {
		if (!this.isAuthorised()) {
			throw "can't get user when state is loading or guest";
		}
		return this.#data as UserData;
	}
	setToLoading() {
		this.#role = "Loading";
		this.#data = null;
	};
	setToGuest() {
		this.#role = "Guest";
		this.#data = null;
	};
	setToAuthorised(id: number, email: string, isAdmin: boolean) {
		this.#role = "Authorised";
		this.#data = { id, email, isAdmin };
	};
}
export const user: Writable<User> = writable(new User());
export enum ThemeMode {
	Light = 0,
	Dark = 1,
	UseDeviceTheme = 2
}
export let themeMode: Writable<ThemeMode | null> = writable(null);