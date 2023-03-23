class ValidationHelper {
	validateEmail(email: string): string | null {
		let validEmailPattern = /^[a-zA-Z0-9.!#$%&’*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,253}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,253}[a-zA-Z0-9])?)*$/;
		if (validEmailPattern.test(email)) {
			return null;
		} else {
			return "Must be a valid email";
		}
	}
	validatePassword(password: string): string | null {
		if (password.length < 8) {
			return "Must be atleast 8 characters";
		} else {
			return null;
		}
	}
}
export default new ValidationHelper();