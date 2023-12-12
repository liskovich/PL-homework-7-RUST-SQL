// use std::sync::{Arc, Mutex};
// use rocket::futures::{StreamExt, FutureExt};
// use tokio::sync::mpsc;
// use warp::ws::{Message, WebSocket};

// // async fn send_info(ws: WebSocket, sender: Arc<Mutex<mpsc::Sender<Message>>>) {
// //     let (mut ws_tx, _) = ws.split();

// //     loop {
// //         tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

// //         // Get the message to send (you can change this string to anything you want to send)
// //         let message = Message::text("Hello from the server!");

// //         // Send the message to the WebSocket client
// //         if let Err(_) = ws_tx.send_message(&message).await {
// //             break;
// //         }
// //     }
// // }

// pub async fn start_websocket_service(ws: warp::ws::WebSocket) {
//     // let (ws_tx, ws_rx) = ws.split();
//     // let (sender, receiver) = mpsc::channel::<Message>(100);

//     // // Spawn a task to send messages periodically
//     // tokio::spawn(send_info(ws, Arc::new(Mutex::new(sender))));

//     // // Receive messages from the WebSocket client (if needed)
//     // let receiver_task = receiver
//     //     .for_each(|_| async {})
//     //     .map_err(|_| ());

//     // tokio::spawn(receiver_task);

//     // Just echo all messages back...
//     let (tx, rx) = ws.split();
//     rx.forward(tx).map(|result| {
//         if let Err(e) = result {
//             eprintln!("websocket error: {:?}", e);
//         }
//     });
// }
