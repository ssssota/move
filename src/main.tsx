import { render } from "preact";
import App from "./App";

const root = document.getElementById("root");
if (!root) {
	alert("Unexpected error");
	throw new Error("Root element not found");
}
render(<App />, root);
