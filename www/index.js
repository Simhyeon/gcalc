import * as gcalc from "gcalc";

class Calculator {
	constructor() {
		this.option = gcalc.default_option();
		this.csv = "";
	}

	calculate() {
		this.csv = gcalc.calculate("range", this.option);
	}
}

// Force export class to browser
window.Calculator = Calculator;
