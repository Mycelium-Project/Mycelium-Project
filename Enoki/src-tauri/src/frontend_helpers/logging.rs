
use nu_ansi_term::{Style, Color};
use tracing::{Level, field::Visit, dispatcher, Dispatch};

thread_local! {
    static DISPATCH: Dispatch = Dispatch::new(
        tracing_subscriber::fmt()
            .event_format(FrontendFormatter)
            .finish());
}


#[tauri::command]
pub async fn tracing_frontend(level: String, msg: String, line: String, file: String) {
    if cfg!(debug_assertions) {
        std::thread::spawn(move || {
                DISPATCH.with(|dispatch| {
                    dispatcher::with_default(&dispatch,
                        move || {
                            match level.as_str() {
                                "trace" => tracing::trace!(message = msg, line = line, file = file),
                                "debug" => tracing::debug!(message = msg, line = line, file = file),
                                "info" => tracing::info!(message = msg, line = line, file = file),
                                "warn" => tracing::warn!(message = msg, line = line, file = file),
                                "error" => tracing::error!(message = msg, line = line, file = file),
                                _ => tracing::error!("Invalid log level: {}", level),
                            }
                        }
                    )
                });
            });
    } else {
        std::thread::Builder::new()
            .name("frontend".to_string())
            .spawn(move || {
                match level.as_str() {
                    "trace" => tracing::trace!(message = msg),
                    "debug" => tracing::debug!(message = msg),
                    "info" => tracing::info!(message = msg),
                    "warn" => tracing::warn!(message = msg),
                    "error" => tracing::error!(message = msg),
                    _ => tracing::error!("Invalid log level: {}", level),
                }
            }).ok();
    }
}

//make a custom tracing formatter
struct FrontendFormatter;

impl<S, N> tracing_subscriber::fmt::FormatEvent<S, N> for FrontendFormatter
where
    S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    N: for<'a> tracing_subscriber::fmt::FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        _ctx: &tracing_subscriber::fmt::FmtContext<'_, S, N>,
        mut writer: tracing_subscriber::fmt::format::Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> core::fmt::Result {
        let meta = event.metadata();

        write!(&mut writer, "  ")?;

        let style = match *meta.level() {
            Level::TRACE => Style::new().fg(Color::Purple),
            Level::DEBUG => Style::new().fg(Color::Blue),
            Level::INFO => Style::new().fg(Color::Green),
            Level::WARN => Style::new().fg(Color::Yellow),
            Level::ERROR => Style::new().fg(Color::Red),
        };

        write!(writer, "{} ", style.paint(meta.level().to_string()))?;

        let target_style = style.bold();

        write!(
            writer,
            "{}{}{}:",
            target_style.prefix(),
            "enoki::frontend",
            target_style.infix(style),
        )?;

        writer.write_char(' ')?;

        let mut visitor = MetaVisitor::default();
        event.record(&mut visitor);

        write!(writer, "{}", style.paint(visitor.message()))?;
        writer.write_char('\n')?;

        let dimmed = Style::new().dimmed().italic();

        let pseudo_thread = "frontend";

        write!(writer, "    {} ", dimmed.paint("at"))?;

        //write file and line number
        write!(
            writer,
            "{}:{}",
            visitor.file_path(),
            visitor.line_num()
        )?;

        write!(writer, " {} {}", dimmed.paint("on"), pseudo_thread)?;

        writer.write_char('\n')?;

        Ok(())
    }
}

#[derive(Default)]
struct MetaVisitor {
    pub line_num: Option<String>,
    pub file_path: Option<String>,
    pub message: String,
}
impl Visit for MetaVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        match field.name() {
            "line" => self.line_num = Some(format!("{:?}", value)),
            "file" => self.file_path = Some(format!("{:?}", value)),
            "message" => self.message = format!("{:?}", value),
            _ => (),
        }
    }
}
impl MetaVisitor {
    pub fn line_num(&self) -> String {
        self.line_num.clone().unwrap_or("unknown".into()).replace('"', "")
    }
    pub fn file_path(&self) -> String {
        self.file_path.clone().unwrap_or("unknown".into()).replace('"', "")
    }
    pub fn message(&self) -> String {
        self.message.clone().replace('"', "")
    }
}