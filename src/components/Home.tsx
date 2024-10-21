import { useEffect } from "react";
import { Link } from "react-router-dom";
import { invoke } from "@tauri-apps/api/core";


function Home() {
    useEffect(() => {
        (async () => {
            await invoke("discard_room");
        })();
    })
    return (
        <main className="container">
            <h1>Mahjong Game</h1>
            <Link to="/game">Game</Link>
        </main>
    )
}

export default Home
