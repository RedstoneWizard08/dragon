use anyhow::Result;
use docker_api::{
    models::RestartPolicyNameInlineItem,
    opts::{ContainerCreateOpts, PublishPort},
};

use super::ContainerHelper;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum RestartPolicy {
    Never,
    Always,
    OnFailure,
    UnlessStopped,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct ContainerCreateOptions {
    /// The container name.
    pub name: Option<String>,

    /// The image.
    pub image: String,

    /// The container's ports.
    /// Spec: (source (in container), target (on host))
    pub ports: Vec<(u32, u32)>,

    /// The network to connect the container to.
    pub network: Option<String>,

    /// The container entrypoint.
    pub entrypoint: Option<Vec<String>>,

    /// The command to run.
    pub command: Option<Vec<String>>,

    /// Attach stdin, stdout, and stderr.
    pub attach: bool,

    /// Enable TTY in the container.
    pub tty: bool,

    /// Remove the container on command exit.
    pub rm: bool,

    /// The restart policy.
    pub restart: Option<RestartPolicy>,
}

impl ContainerHelper {
    pub async fn create(&self, opts: ContainerCreateOptions) -> Result<String> {
        let mut builder = ContainerCreateOpts::builder().image(opts.image.clone());

        if let Some(name) = opts.name.clone() {
            builder = builder.name(name);
        }

        if let Some(network) = opts.network.clone() {
            builder = builder.network_mode(network);
        }

        if let Some(entrypoint) = opts.entrypoint.clone() {
            builder = builder.entrypoint(entrypoint);
        }

        if let Some(command) = opts.command.clone() {
            builder = builder.command(command);
        }

        if opts.attach {
            builder = builder.attach_stdin(true);
            builder = builder.attach_stdout(true);
            builder = builder.attach_stderr(true);
        }

        if opts.tty {
            builder = builder.tty(true);
        }

        if opts.rm {
            builder = builder.auto_remove(true);
        }

        if let Some(restart) = opts.restart {
            let policy = match restart {
                RestartPolicy::Never => RestartPolicyNameInlineItem::No,
                RestartPolicy::Always => RestartPolicyNameInlineItem::Always,
                RestartPolicy::OnFailure => RestartPolicyNameInlineItem::OnFailure,
                RestartPolicy::UnlessStopped => RestartPolicyNameInlineItem::UnlessStopped,
            };

            builder = builder.restart_policy(policy.as_ref(), 5);
        }

        for (source, target) in opts.ports.clone() {
            builder = builder.expose(PublishPort::tcp(source), target);
            builder = builder.expose(PublishPort::udp(source), target);
        }

        let copts = builder.build();
        let containers = self.docker.containers();

        match containers.create(&copts).await {
            Ok(container) => Ok(container.id().to_string()),
            Err(err) => return Err(anyhow!(err)),
        }
    }
}
