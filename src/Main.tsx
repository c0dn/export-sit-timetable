import {render} from "solid-js/web";
import {Route, Router, RouteSectionProps} from "@solidjs/router";
import Home from "./Home.tsx";
import "./App.css"

const App = (props: RouteSectionProps) => {
    return (
        <>
            <div class="bg-slate-700 min-h-screen">
                {props.children}
            </div>
        </>
    )
}


render(() => {
    if (window.location.hostname === 'tauri.localhost') {
        document.addEventListener('contextmenu', e => {
            e.preventDefault();
            return false;
        }, {capture: true})

        document.addEventListener('selectstart', e => {
            e.preventDefault();
            return false;
        }, {capture: true})
    }

    return (
        <>
            <Router root={App}>
                <Route path="/" component={Home}/>
            </Router>
        </>)
}, document.getElementById("root") as HTMLElement);