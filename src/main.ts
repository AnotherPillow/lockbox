import "./styles.scss";
import App from "./App.svelte";
import { invoke } from "@tauri-apps/api/tauri";
import { startup_data } from "./stores";

const app = new App({
    target: document.getElementById("app"),
});

invoke("get_launch_data").then((result) => {
    if (typeof result === "string") {
        startup_data.set(JSON.parse(result));
    } else {
        startup_data.set(result);
    }

    

});

export default app;
