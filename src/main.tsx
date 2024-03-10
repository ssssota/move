import "@unocss/reset/tailwind.css";
import { render } from "preact";
import "virtual:uno.css";
import App from "./App";
import "./index.css";

const root = document.getElementById("root");
if (!root) {
	alert("Unexpected error");
	throw new Error("Root element not found");
}
render(<App />, root);
