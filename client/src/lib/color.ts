class Color {
	static getRandomHex() {
		return '#'+(Math.random() * 0xFFFFFF << 0).toString(16).padStart(6, '0');
	}
	static hexToRgb(hex: string) {
		var result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
		return result ? [
				parseInt(result[1], 16),
				parseInt(result[2], 16),
				parseInt(result[3], 16)
			] : null;
	}
}

export default Color;