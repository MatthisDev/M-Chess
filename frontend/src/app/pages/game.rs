use crate::app::{state::ServerState, ServerAction};
use game_lib::{
    messages::ClientMessage,
    position::Position,
    sharedenums::{GameMode, PlayerRole, RoomStatus},
};
use std::rc::Rc;
use uuid::Uuid;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GameProps {
    pub on_quit: Callback<()>,
}

#[function_component(Game)]
pub fn game(props: &GameProps) -> Html {
    let server_state =
        use_context::<UseReducerHandle<ServerState>>().expect("ServerState context is missing");
    let ctx = use_context::<crate::ws::WsContext>().expect("missing WsContext");
    let selected_piece = use_state(|| None as Option<String>);
    let selected_square = use_state(|| None as Option<Position>);
    let board_theme = use_state(|| "blue-theme".to_string());

    let on_click_pause = {
        let ctx = ctx.clone();
        Callback::from(move |_| {
            ctx.send(ClientMessage::PauseRequest);
        })
    };

    let set_theme = {
        let board_theme = board_theme.clone();
        Callback::from(move |theme: String| {
            board_theme.set(theme);
        })
    };

    // Callbacks
    let on_click_quit = {
        let cb = props.on_quit.clone();
        Callback::from(move |_| {
            cb.emit(());
        })
    };

    let on_ready = {
        let ctx = ctx.clone();
        let ready_state = server_state.ready;
        Callback::from(move |_| {
            ctx.send(ClientMessage::Ready { state: ready_state });
        })
    };

    let on_start_game = {
        let ctx = ctx.clone();
        let selected_piece = selected_piece.clone();
        let selected_square = selected_square.clone();
        Callback::from(move |_| {
            selected_piece.set(None);
            selected_square.set(None);
            ctx.send(ClientMessage::StartGame)
        })
    };

    // Display helpers
    let room_id_display = server_state
        .room_id
        .map_or("...".to_string(), |id| id.to_string());
    let room_status_display = server_state
        .room_status
        .as_ref()
        .map_or("...".to_string(), |s| format!("{:?}", s));
    let role_display = server_state
        .role
        .as_ref()
        .map_or("...".to_string(), |r| format!("{:?}", r));
    let gamemode_display = server_state
        .gamemod
        .as_ref()
        .map_or("...".to_string(), |r| format!("{:?}", r));
    let ready_display = if server_state.ready { "Yes" } else { "No" };
    let turn_display = format!("{:?}", server_state.counter);
    let game_over_display = server_state
        .game_over
        .as_ref()
        .map(|r| html! { <p>{ format!("Game Over: {}", r) }</p> });

    // Legal moves and piece selection
    let legal_moves = server_state.legals_moves.clone();
    let clicked = {
        let selected_square = selected_square.clone();
        let selected_piece = selected_piece.clone();
        let legal_moves = legal_moves.clone();
        let server_state = server_state.clone();
        Callback::from(move |pos: (u8, u8)| {
            let ctx = ctx.clone();
            let pos = Position {
                row: pos.0 as usize,
                col: pos.1 as usize,
            };

            let is_empty = server_state.board[pos.row][pos.col]
                .as_ref()
                .map_or(true, |s| s.is_empty());

            if is_empty && selected_square.is_none() && selected_piece.is_none() {
                return; // Ignore clicks on empty squares if nothing is selected
            }
            if is_empty && selected_piece.is_some() {
                if let Some(piece) = (*selected_piece).as_ref() {
                    ctx.send(ClientMessage::AddPiece {
                        piece: piece.clone(),
                        pos: pos.to_algebraic(),
                    });
                    selected_piece.set(None);
                }
            } else if Some(pos) == *selected_square {
                selected_square.set(None);
                server_state.dispatch(ServerAction::SetLegalMoves(Vec::new()));
            } else if legal_moves.contains(&pos.to_algebraic()) && selected_square.is_some() {
                let message = ClientMessage::Move {
                    mv: format!(
                        "{:?}->{:?}",
                        selected_square.unwrap().to_algebraic(),
                        pos.to_algebraic()
                    ),
                };
                ctx.send(message);
                selected_square.set(None);
            } else if selected_square.is_none() {
                selected_square.set(Some(pos));
                let message = ClientMessage::GetLegalMoves {
                    mv: format!("{:?}", pos.to_algebraic()),
                };
                ctx.send(message);
            } else {
                selected_square.set(None);
                server_state.dispatch(ServerAction::SetLegalMoves(Vec::new()));
            }
        })
    };

    html! {
        <div class="game-container">
            <h2 class="game-title">{ "Game Room" }</h2>

            {
                if let Some(game_over_message) = &server_state.game_over {
                    html! {
                        <div class="game-over-message">
                            <h3>{ "Game Over" }</h3>
                            <p>{ game_over_message }</p>
                            <button class="game-button" onclick={on_click_quit.clone()}>{ "Quit Game" }</button>
                        </div>
                    }
                } else {
                    html! {}
                }
            }

            <div class="game-layout">
                // Colonne gauche : Boutons et Informations
                <div class="game-controls">
                    <div class="theme-buttons">
                        <button class="theme-button" onclick={Callback::from({
                            let set_theme = set_theme.clone();
                            move |_| set_theme.emit("blue-theme".to_string())
                        })}>{ "Blue Theme" }</button>
                        <button class="theme-button" onclick={Callback::from({
                            let set_theme = set_theme.clone();
                            move |_| set_theme.emit("brown-theme".to_string())
                        })}>{ "Brown Theme" }</button>
                        <button class="theme-button" onclick={Callback::from({
                            let set_theme = set_theme.clone();
                            move |_| set_theme.emit("gray-theme".to_string())
                        })}>{ "Gray Theme" }</button>
                    </div>
                    {

                        if server_state.role != Some(PlayerRole::Spectator) || server_state.gamemod == Some(GameMode::AIvsAI)
                        {
                           html! (
                            <>
                                <button
                                    class="game-button"
                                    disabled={server_state.room_status == Some(RoomStatus::Running) || server_state.room_status == Some(RoomStatus::Finished)}
                                    onclick={on_ready}
                                >
                                    { "Set Ready" }
                                </button>
                                if server_state.host{

                                        <button
                                            class="game-button"
                                            disabled={!server_state.ready ||
                                                     !server_state.host ||
                                                     server_state.room_status != Some(RoomStatus::WaitingReady)}
                                            onclick={on_start_game}
                                        >
                                            { "Start Game" }
                                        </button>

                                }
                                else{

                                }
                            </>
                        )
                        }
                        else{
                            web_sys::console::log_1(
                            &format!(
                                "Error: role: {:?}, gammemod: {:?}",
                                server_state.role, server_state.gamemod
                            )
                            .into(),
                        );
                        html!()   }
                    }
                    { if server_state.gamemod == Some(GameMode::AIvsAI) && matches!(server_state.room_status, Some(RoomStatus::Running)|Some(RoomStatus::Paused)){

                        if server_state.paused
                        {
                            html!(<button class="game-button" onclick={on_click_pause}>{ "Resume Game" }</button>)
                        }
                    else{
                        html!(<button class="game-button" onclick={on_click_pause}>{ "Pause Game" }</button>)
                    }}
                        else{
                            html!()
                        }
                    }
                    <button class="game-button" onclick={on_click_quit}>{ "Quit Game" }</button>


                    // Informations de la salle
                    <div class="game-info">
                        <p><strong>{ "Room ID: " }</strong>{ room_id_display.clone() }</p>
                        <p><strong>{ "Room Status: " }</strong>{ room_status_display.clone() }</p>
                        <p><strong>{ "Role: " }</strong>{ role_display.clone() }</p>
                        {
                            if server_state.role != Some(PlayerRole::Spectator) && server_state.gamemod != Some(GameMode::AIvsAI){
                            html!(<p><strong>{ "Ready: " }</strong>{ ready_display }</p>)
                        }else{html!()}}
                        <p><strong>{ "Turn: " }</strong>{ turn_display }</p>
                    </div>
                </div>

                // Plateau d'échecs
                <div class={classes!("chess-board", (*board_theme).clone())}>
                    { for server_state.board.iter().enumerate().map(|(r, row)| html! {
                        { for row.iter().enumerate().map(|(c, cell)| {
                            let pos = Position { row: r, col: c };
                            let is_legal = legal_moves.contains(&pos.to_algebraic());
                            let is_selected = *selected_square == Some(pos);

                            let mut class = if (r + c) % 2 == 0 { "chess-cell light" } else { "chess-cell dark" }.to_string();
                            if is_legal {
                                class += " legal";
                            }
                            if is_selected {
                                class += " selected";
                            }

                            html! {
                                <div class={class}
                                    onclick={
                                        let onclick = clicked.clone();
                                        Callback::from(move |_: MouseEvent| onclick.emit((r as u8, c as u8)))
                                    }
                                >
                                    { cell.as_ref().map_or("".to_string(), |piece| get_piece_emoji(piece).to_string()) }
                                </div>
                            }
                        }) }
                    }) }
                </div>

                // Colonne droite : Palette Sandbox
                {
                     if server_state.gamemod == Some(GameMode::Sandbox)
                    {
                        html! {
                            <div  class="sandbox-container">
                                <h4>{ "Sandbox Pieces" }</h4>
                                <div class="sandbox-pieces">
                                    { for ["wp", "bp", "wr", "br", "wn", "bn", "wb", "bb", "wq", "bq", "wk", "bk"].iter().map(|&piece| {
                                        let selected = *selected_piece == Some(piece.to_string());
                                        let class = if selected { "sandbox-piece selected" } else { "sandbox-piece" };
                                        let sp = selected_piece.clone();
                                        html! {
                                            <div class={class}
                                                onclick={Callback::from(move |_| sp.set(Some(piece.to_string())))}
                                            >
                                                { get_piece_emoji(piece) }
                                            </div>
                                        }
                                    })}
                                </div>
                                <button class="game-button" onclick={Callback::from({
                                    let sp = selected_piece.clone();
                                    move |_| sp.set(Some("".to_string())) // Définit une pièce vide
                                })}>{ "Place Empty Piece" }</button>
                            </div>
                        }
                    }
                     else {html! {}}
                }
            </div>
        </div>
    }
}

// Helper function to get chess piece emojis
fn get_piece_emoji(piece: &str) -> &str {
    match piece {
        "wp" => "♙", // White Pawn
        "bp" => "♟", // Black Pawn
        "wr" => "♖", // White Rook
        "br" => "♜", // Black Rook
        "wn" => "♘", // White Knight
        "bn" => "♞", // Black Knight
        "wb" => "♗", // White Bishop
        "bb" => "♝", // Black Bishop
        "wq" => "♕", // White Queen
        "bq" => "♛", // Black Queen
        "wk" => "♔", // White King
        "bk" => "♚", // Black King
        _ => "",     // Empty cell or invalid piece
    }
}
