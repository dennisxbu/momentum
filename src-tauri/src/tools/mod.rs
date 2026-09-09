#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolStatus {
    Enabled,
    Paused,
    Hidden,
}

pub trait OptionalTool {
    fn status(&self) -> ToolStatus;
    fn context_contribution(&self) -> Option<&'static str>;
}

pub fn active_contributions(tools: &[Box<dyn OptionalTool>]) -> Vec<&'static str> {
    tools
        .iter()
        .filter(|tool| tool.status() == ToolStatus::Enabled)
        .filter_map(|tool| tool.context_contribution())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    struct HiddenCalendar;
    impl OptionalTool for HiddenCalendar {
        fn status(&self) -> ToolStatus {
            ToolStatus::Hidden
        }
        fn context_contribution(&self) -> Option<&'static str> {
            Some("Darf nicht im Briefing landen")
        }
    }
    #[test]
    fn disabled_tool_has_no_effect() {
        assert!(active_contributions(&[Box::new(HiddenCalendar)]).is_empty());
    }
}
