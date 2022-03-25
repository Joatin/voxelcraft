use crate::gpu::{Gpu, RenderContext};
use crate::interface::message::Message;
use crate::interface::pages::get_default_pages;
use crate::interface::pages::MAIN_PAGE_ROUTE;
use crate::interface::router::Router;
use crate::interface::router_flags::RouterFlags;
use crate::primitives;
use crate::primitives::Point2D;
use crate::window::Clipboard;
use iced::mouse::Interaction;
use iced::{Application, Size};
use iced_native::user_interface::Cache;
use iced_native::{Debug, Event, UserInterface};
use iced_wgpu::Settings;
use iced_wgpu::{Backend, Renderer, Viewport};

use wgpu::util::StagingBelt;
use wgpu::CommandBuffer;

pub struct Interface {
    cache: Cache,
    renderer: Renderer,
    messages: Vec<Message>,
    events: Vec<Event>,
    router: Router,
    staging_belt: StagingBelt,
    clipboard: Clipboard,
    viewport: Viewport,
    cursor_position: Point2D,
    debug: Debug,
}

impl Interface {
    pub fn new(window: &winit::window::Window, gpu: &Gpu) -> Self {
        let cache = Cache::new();

        let backend = Backend::new(
            &gpu.device,
            Settings {
                default_font: Some(include_bytes!("Inconsolata-Regular.ttf")),
                ..Default::default()
            },
            gpu.render_format,
        );

        let renderer = Renderer::new(backend);
        let messages = vec![];
        let events = vec![];

        let pages = get_default_pages();
        let (router, _) = Router::new(RouterFlags {
            pages,
            initial_route: MAIN_PAGE_ROUTE.to_string(),
        });
        let staging_belt = StagingBelt::new(1024);
        let clipboard = Clipboard::connect(window);
        let viewport = Viewport::with_physical_size(
            Size {
                width: window.inner_size().width,
                height: window.inner_size().height,
            },
            window.scale_factor(),
        );
        let cursor_position = Point2D::default();
        let debug = Debug::new();

        Self {
            cache,
            renderer,
            messages,
            router,
            staging_belt,
            clipboard,
            viewport,
            cursor_position,
            events,
            debug,
        }
    }

    pub fn render<T: FnMut(&Message)>(
        &mut self,
        render_context: &RenderContext,
        game_messages: Vec<Message>,
        mut message_callback: T,
    ) -> (CommandBuffer, Interaction, bool) {
        let mut encoder =
            render_context
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Interface Render"),
                });

        for message in game_messages {
            self.router.update(message);
        }

        let mut user_interface = UserInterface::build(
            self.router.view(),
            self.viewport.logical_size(),
            self.cache.clone(),
            &mut self.renderer,
        );

        let _event_statuses = user_interface.update(
            &self.events,
            self.cursor_position.into(),
            &mut self.renderer,
            &mut self.clipboard,
            &mut self.messages,
        );

        self.events.clear();

        let mouse_cursor = user_interface.draw(&mut self.renderer, self.cursor_position.into());

        self.renderer.with_primitives(|backend, primitives| {
            backend.present::<String>(
                &render_context.device,
                &mut self.staging_belt,
                &mut encoder,
                &render_context.view,
                primitives,
                &self.viewport,
                &self.debug.overlay(),
            )
        });

        self.cache = user_interface.into_cache();

        self.staging_belt.finish();

        let mut should_quit = false;
        for message in self.messages.drain(..) {
            if let Message::QuitApplication = message {
                should_quit = true;
            }
            message_callback(&message);
            self.router.update(message);
        }

        (encoder.finish(), mouse_cursor, should_quit)
    }

    pub fn cleanup(&mut self) {
        tokio::spawn(self.staging_belt.recall());
    }

    pub fn set_cursor_position(&mut self, cursor_position: Point2D) {
        self.cursor_position = cursor_position;
    }

    pub fn resize(&mut self, size: primitives::Size, scale_factor: f64) {
        self.viewport = Viewport::with_physical_size(
            Size {
                width: size.width as u32,
                height: size.height as u32,
            },
            scale_factor,
        )
    }

    pub fn push_event(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn escape_pressed(&mut self) {
        self.messages.push(Message::EscapePressed)
    }

    pub fn should_grab_cursor(&self) -> bool {
        self.router.should_grab_cursor()
    }
}
