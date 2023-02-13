import { writable, type Writable } from 'svelte/store';

type UserData = { id: number, email: string, isAdmin: boolean };

export class User {
    #role: "Loading" | "Guest" | "Authorised";
    #data: {id: number, email: string, isAdmin: boolean} | null;
    constructor(arg: "Loading" | "Guest" | UserData) {
        if (typeof arg == "string") {
            this.#role = arg;
            this.#data = null;
        } else {
            this.#role = "Authorised";
            this.#data = arg;
        }
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
}
export const user: Writable<User> = writable(new User("Loading"));