import React, { useEffect, useState } from 'react'
import { invoke } from "@tauri-apps/api/core";
import { Link } from 'react-router-dom';


function Game() {
    const [players, setPlayers] = useState<Player[]>([]);
    useEffect(() => {
        (async () => {
            await invoke("create_room");
            await invoke<string>("get_player_info")
                .then(_players => {
                    setPlayers(JSON.parse(_players));
                });
        })()
    }, []);

    return (<>
        <div>Game</div>
        <ul>
            {players.map((player) => (
                <li key={player.id}>
                    {player.name}, ID: {player.id}, is_parent: {player.is_parent? "true": "false"}, point: {player.point}, wind: {player.wind}, hand: {player.hand.hoge}
                </li>
            ))}
        </ul>
        <Link to="/">Back</Link>
    </>)
}

export default Game
