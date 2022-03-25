use crate::input::input_config::InputConfig;
use crate::input::user_action::UserAction;
use crate::input::user_action_state::UserActionState;
use pollster::FutureExt;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc, RwLock};
use winit::event::ElementState;

pub struct InputManager {
    config: InputConfig,
    internal_sender: mpsc::Sender<(UserAction, UserActionState)>,
    channel: broadcast::Sender<(UserAction, UserActionState)>,
    head_rotation_channel: broadcast::Sender<(f64, f64)>,
    active: Arc<RwLock<HashSet<UserAction>>>,
}

impl InputManager {
    pub fn new() -> Self {
        let config = InputConfig::default();
        let (channel, mut receiver) = broadcast::channel(1000);
        let (head_rotation_channel, mut receiver) = broadcast::channel(1000);
        let active = Arc::new(RwLock::new(HashSet::new()));

        let (internal_sender, internal_receiver) = mpsc::channel(1000);

        Self::spawn_update_task(internal_receiver, &active, channel.clone());

        Self {
            config,
            channel,
            active,
            internal_sender,
            head_rotation_channel,
        }
    }

    fn spawn_update_task(
        mut receiver: mpsc::Receiver<(UserAction, UserActionState)>,
        active: &Arc<RwLock<HashSet<UserAction>>>,
        sender: broadcast::Sender<(UserAction, UserActionState)>,
    ) {
        let active = Arc::clone(&active);
        tokio::spawn(async move {
            loop {
                let (action, state) = receiver.recv().await.unwrap();
                let mut lock = active.write().await;
                match state {
                    UserActionState::Started => {
                        if lock.insert(action.clone()) {
                            sender.send((action, state)).unwrap();
                        }
                    }
                    UserActionState::Stopped => {
                        lock.remove(&action);
                        sender.send((action, state)).unwrap();
                    }
                }
            }
        });
    }

    pub fn on_mouse_moved(&self, delta_x: f64, delta_y: f64) {
        // We don't need to track this internally in active actions since it's deactivated right away
        if let Err(err) = self.head_rotation_channel.send((
            delta_x * self.config.mouse_sensitivity(),
            delta_y * self.config.mouse_sensitivity(),
        )) {
            log::warn!("Failed to handle mouse input, this might happen when there are no listeners for these events, inner error was: {}", err)
        }
    }

    pub fn on_keyboard_input(&self, state: ElementState, scancode: u32) {
        if let Some(action) = self.config.get_action(scancode) {
            if let Err(err) = self
                .internal_sender
                .send((action.to_owned(), state.into()))
                .block_on()
            {
                log::warn!("Failed to handle keyboard input, this might happen when there are no listeners for these events, inner error was: {}", err)
            }
        }
    }

    pub fn listen_on_actions(&self) -> broadcast::Receiver<(UserAction, UserActionState)> {
        self.channel.subscribe()
    }

    pub fn listen_on_head_rotation(&self) -> broadcast::Receiver<(f64, f64)> {
        self.head_rotation_channel.subscribe()
    }

    pub async fn is_action_active(&self, action: &UserAction) -> bool {
        let lock = self.active.read().await;
        lock.contains(action)
    }
}
