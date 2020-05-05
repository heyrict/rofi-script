use std::borrow::Cow;
use std::fmt;

pub struct ModeOption<'a> {
    /// Update the prompt text.
    prompt: Option<Cow<'a, str>>,
    /// Update the message text.
    message: Option<Cow<'a, str>>,
    /// If `true` renders markup in the row.
    markup_rows: Option<bool>,
    /// Only accept listed entries, ignore custom input.
    no_custom: Option<bool>,
    /// Mark rows as urgent.
    urgent: Option<Cow<'a, str>>,
    /// Mark rows as active.
    active: Option<Cow<'a, str>>,
    ///  Set the delimiter for the next rows.
    delim: Option<Cow<'a, str>>,
}

impl<'a> Default for ModeOption<'a> {
    fn default() -> Self {
        ModeOption {
            prompt: None,
            message: None,
            markup_rows: None,
            no_custom: None,
            urgent: None,
            active: None,
            delim: None,
        }
    }
}

impl<'a> fmt::Display for ModeOption<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();

        if let Some(prompt) = &self.prompt {
            output.push_str(&option_line(None, "prompt", prompt.as_ref()));
        };

        if let Some(message) = &self.message {
            output.push_str(&option_line(None, "message", message.as_ref()));
        };

        if let Some(markup_rows) = &self.markup_rows {
            output.push_str(&option_line(None, "markup-rows", &markup_rows.to_string()));
        };

        if let Some(no_custom) = &self.no_custom {
            output.push_str(&option_line(None, "no-custom", &no_custom.to_string()));
        };

        if let Some(urgent) = &self.urgent {
            output.push_str(&option_line(None, "urgent", urgent.as_ref()));
        };

        if let Some(active) = &self.active {
            output.push_str(&option_line(None, "active", active.as_ref()));
        };

        if let Some(delim) = &self.delim {
            output.push_str(&option_line(None, "delim", delim.as_ref()));
        };

        output.push('\n');
        write!(f, "{}", output)
    }
}

impl<'a> ModeOption<'a> {
    pub fn is_empty(&self) -> bool {
        self.prompt.is_none()
            && self.message.is_none()
            && self.markup_rows.is_none()
            && self.no_custom.is_none()
            && self.urgent.is_none()
            && self.active.is_none()
            && self.delim.is_none()
    }

    pub fn set_prompt<T: Into<Cow<'a, str>>>(&mut self, prompt: T) -> &Self {
        self.prompt = Some(prompt.into());
        self
    }

    pub fn set_message<T: Into<Cow<'a, str>>>(&mut self, message: T) -> &Self {
        self.message = Some(message.into());
        self
    }

    pub fn set_markup_rows(&mut self, markup_rows: bool) -> &Self {
        self.markup_rows = Some(markup_rows);
        self
    }

    pub fn set_no_custom(&mut self, no_custom: bool) -> &Self {
        self.no_custom = Some(no_custom);
        self
    }

    pub fn set_urgent<T: Into<Cow<'a, str>>>(&mut self, urgent: T) -> &Self {
        self.urgent = Some(urgent.into());
        self
    }

    pub fn set_active<T: Into<Cow<'a, str>>>(&mut self, active: T) -> &Self {
        self.active = Some(active.into());
        self
    }

    pub fn set_delim<T: Into<Cow<'a, str>>>(&mut self, delim: T) -> &Self {
        self.delim = Some(delim.into());
        self
    }
}

pub struct RowOption<'a> {
    /// The text of the row
    text: Cow<'a, str>,
    /// Set the icon for that row.
    icon: Option<Cow<'a, str>>,
    /// Specify invisible search terms.
    meta: Option<Cow<'a, str>>,
    /// If `true` the row cannot be activated.
    non_selectable: Option<bool>,
}

impl<'a> fmt::Display for RowOption<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Due to limitations of rofi script mode, each row can have only one option.
        // This may change in future APIs

        let output = if let Some(icon) = &self.icon {
            option_line(Some(self.text.as_ref()), "icon", icon)
        } else if let Some(meta) = &self.meta {
            option_line(Some(self.text.as_ref()), "meta", meta)
        } else if let Some(non_selectable) = &self.non_selectable {
            option_line(
                Some(self.text.as_ref()),
                "nonselectable",
                &non_selectable.to_string(),
            )
        } else {
            format!("{}\n", self.text.as_ref())
        };

        write!(f, "{}", output)
    }
}

impl<'a> RowOption<'a> {
    pub fn new<T: Into<Cow<'a, str>>>(text: T) -> Self {
        Self {
            text: text.into(),
            icon: None,
            meta: None,
            non_selectable: None,
        }
    }

    pub fn set_icon<T: Into<Cow<'a, str>>>(&mut self, icon: T) -> &Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn set_meta<T: Into<Cow<'a, str>>>(&mut self, meta: T) -> &Self {
        self.meta = Some(meta.into());
        self
    }

    pub fn set_non_selectable(&mut self, non_selectable: bool) -> &Self {
        self.non_selectable = Some(non_selectable);
        self
    }
}

fn option_line(text: Option<&str>, key: &str, value: &str) -> String {
    match &text {
        Some(text) => format!("{}\0{}\x1f{}\n", &text, key, value),
        None => format!("\0{}\x1f{}\n", key, value),
    }
}
