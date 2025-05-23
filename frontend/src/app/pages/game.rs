use crate::{
    app::{state::ServerState, ServerAction},
    messages::ClientMessage,
    sharedenums::RoomStatus,
};
use game_lib::position::Position;
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

    let on_click_quit = {
        let cb = props.on_quit.clone();
        Callback::from(move |_| {
            cb.emit(());
        })
    };
    //Ready callback to send the ready message to Server
    let on_ready = {
        let ctx = ctx.clone();
        let server_state = server_state.clone();
        Callback::from(move |_| {
            ctx.send(ClientMessage::Ready {
                state: server_state.ready,
            });
        })
    };
    let on_start_game = {
        let ctx = ctx.clone();
        let server_state = server_state.clone();
        let selected_piece = selected_piece.clone();
        let selected_square = selected_square.clone();
        Callback::from(move |_| {
            selected_piece.set(None);
            selected_square.set(None);
            ctx.send(ClientMessage::StartGame)
        })
    };
    // Display the game id
    let room_id_display = server_state
        .room_id
        .map_or("...".to_string(), |id| id.to_string());

    // Display the room status
    let room_status_display = server_state
        .room_status
        .as_ref()
        .map_or("...".to_string(), |s| format!("{:?}", s));

    // Display the player role
    let role_display = server_state
        .role
        .as_ref()
        .map_or("...".to_string(), |r| format!("{:?}", r));

    // Display the ready status
    let ready_display = if server_state.ready { "Yes" } else { "No" };

    // Display the turn
    let turn_display = server_state
        .turn
        .as_ref()
        .map_or("...".to_string(), |t| format!("{:?}", t));

    // Display the game over status
    let game_over_display = server_state
        .game_over
        .as_ref()
        .map(|r| html! { <p>{ format!("Game Over: {}", r) }</p> });

    //------------------
    //  Move Piece Part
    //------------------
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
                return; // Ignore le clic sur une case vide si rien de selectionné
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
                ctx.send(message)
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
        <div>
            <h2>{ "Game Room" }</h2>
            <p><strong>{ "Room ID: " }</strong>{ room_id_display.clone() }</p>
            <p><strong>{ "Room Status: " }</strong>{ room_status_display.clone() }</p>
            <p><strong>{ "Role: " }</strong>{ role_display.clone() }</p>
            <p><strong>{ "Ready: " }</strong>{ ready_display }</p>
            <p><strong>{ "Turn: " }</strong>{ turn_display }</p>

            { for game_over_display }
            <h3>{ "Chess Board" }</h3>
            <table style="border-collapse: collapse;">
        { for server_state.board.iter().enumerate().map(|(r, row)| html! {
            <tr>
                { for row.iter().enumerate().map(|(c, cell)| {
                    let pos = Position { row: r, col: c };
                    let is_legal = legal_moves.contains(&pos.to_algebraic());
                    let is_selected = *selected_square == Some(pos);

                    // Style de base
                    let mut style = "border: 1px solid black; width: 40px; height: 40px; text-align: center; cursor: pointer;".to_string();

                    if is_legal {
                        style += " background-color: lightgreen;";
                    }
                    if is_selected {
                        style += " outline: 3px solid blue;";
                    }



                    html! {
                        <td {style}
                        onclick={
                            let onclick = clicked.clone();
                            Callback::from(move |_: MouseEvent| onclick.emit((r as u8, c as u8)))
                        }
                        >
                            { cell.clone().unwrap_or("".to_string()) }
                        </td>
                    }
                }) }
            </tr>
        }) }

        <div style="border-left: 1px solid #ccc; padding-left: 10px;">
        <h4>{"Sandbox Pieces"}</h4>
        <div style="display: grid; grid-template-columns: repeat(2, 40px); gap: 10px;">
            { for ["wp", "bp", "wr", "br", "wn", "bn", "wb", "bb", "wq", "bq", "wk", "bk"].iter().map(|&piece| {
                let selected = *selected_piece == Some(piece.to_string());
                let style = if selected {
                    "border: 2px solid red; font-size: 24px; cursor: pointer;"
                } else {
                    "font-size: 24px; cursor: pointer;"
                };
                let sp = selected_piece.clone();
                html! {
                    <div style={style}
                        onclick={Callback::from(move |_| sp.set(Some(piece.to_string())))}
                    >
                        { get_piece_emoji(piece) }
                    </div>
                }
            })}
        </div>
        <button onclick={Callback::from({
            let sp = selected_piece.clone();
            move |_| sp.set(None)
        })}>{"Clear Selection"}</button>
    </div>
    </table>
            <button
                disabled = {server_state.room_status == Some(RoomStatus::Running) || server_state.room_status == Some(RoomStatus::Finished)}
                onclick={on_ready}
            >
                { "Set Ready" }
            </button>

            <button
                disabled={!server_state.ready||!server_state.host || server_state.room_status != Some(RoomStatus::WaitingReady)}
                onclick={on_start_game}
            >{"Start Game"}</button>
            <button onclick={on_click_quit}>{ "Quit Game" }</button>
        </div>
    }
}

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
