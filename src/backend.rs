// use crate::platform::Platform;
use crate::tracing::{SocketImpl, Tracer, TracerChannel, TracerChannelConfig, TracerConfig};
use parking_lot::RwLock;
use std::fmt::Debug;
use std::sync::Arc;
use trace::Trace;
use tracing::instrument;

pub mod flows;
pub mod trace;

/// A tracing backend.
#[derive(Debug)]
pub struct Backend {
    tracer_config: TracerConfig,
    channel_config: TracerChannelConfig,
    trace: Arc<RwLock<Trace>>,
}

impl Backend {
    /// Create a tracing `Backend`.
    pub fn new(
        tracer_config: TracerConfig,
        channel_config: TracerChannelConfig,
        max_samples: usize,
    ) -> Self {
        Self {
            tracer_config,
            channel_config,
            trace: Arc::new(RwLock::new(Trace::new(max_samples))),
        }
    }

    pub fn trace(&self) -> Arc<RwLock<Trace>> {
        self.trace.clone()
    }

    /// Run the tracing backend.
    ///
    /// Note that this implementation blocks the tracer on the `RwLock` and so any delays in the the TUI
    /// will delay the next round of the trace.
    #[instrument(skip_all)]
    pub fn start(&self) -> anyhow::Result<()> {
        let td = self.trace.clone();
        let channel =
            TracerChannel::<SocketImpl>::connect(&self.channel_config).map_err(|err| {
                td.write().set_error(Some(err.to_string()));
                err
            })?;
        // Platform::drop_privileges()?; // This will disble multi-threading
        let tracer = Tracer::new(&self.tracer_config, move |round| {
            self.trace.write().update_from_round(round);
        });
        match tracer.trace(channel) {
            Ok(()) => {}
            Err(err) => {
                td.write().set_error(Some(err.to_string()));
            }
        };
        Ok(())
    }
}
