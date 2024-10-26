use logging::initialize_logging;
use state::state_store::StateStore;
use termination::create_termination;

mod api;
mod state;
mod termination;
mod ui;

use tokio;

use termination::{Interrupted, Terminator};
use ui::ui_loop::UILoop;
mod logging;

#[tokio::main(worker_threads = 1)]
async fn main() -> anyhow::Result<()> {
    // console_subscriber::init();

    let _ = initialize_logging();

    let (terminator, mut interrupt_rx) = create_termination();
    let (state_store, state_rx) = StateStore::new();
    let (ui_manager, action_rx) = UILoop::new();

    tokio::try_join!(
        state_store.main_loop(terminator, action_rx, interrupt_rx.resubscribe()),
        ui_manager.main_loop(state_rx, interrupt_rx.resubscribe()),
    )?;

    if let Ok(reason) = interrupt_rx.recv().await {
        match reason {
            Interrupted::UserInt => println!("exited per user request"),
            Interrupted::OsSigInt => println!("exited because of an os sig int"),
        }
    } else {
        println!("exited because of an unexpected error");
    }

    Ok(())
}
