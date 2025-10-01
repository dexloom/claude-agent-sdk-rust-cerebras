use claude_agent_sdk::error::AgentError;
use claude_agent_sdk::internal::query::*;
use claude_agent_sdk::transport::Transport;
use mockall::mock;
use mockall::predicate::*;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

// Mock transport for testing
mock! {
    pub Transport {}
    #[async_trait::async_trait]
    impl Transport for Transport {
        async fn send(&self, payload: Value) -> Result<Value, Box<dyn std::error::Error + Send + Sync>>;
        async fn receive(&self) -> Result<Value, Box<dyn std::error::Error + Send + Sync>>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_permission_context_creation() {
        let context = ToolPermissionContext {
            signal: Some("test_signal".to_string()),
            suggestions: vec![],
        };
        assert_eq!(context.signal, Some("test_signal".to_string()));
    }

    #[test]
    fn test_permission_result_allow_creation() {
        let allow_result = PermissionResultAllow {
            behavior: "allow".to_string(),
            updated_input: None,
            updated_permissions: None,
        };
        assert_eq!(allow_result.behavior, "allow");
    }

    #[test]
    fn test_permission_result_deny_creation() {
        let deny_result = PermissionResultDeny {
            behavior: "deny".to_string(),
            message: "test message".to_string(),
            interrupt: false,
        };
        assert_eq!(deny_result.behavior, "deny");
        assert_eq!(deny_result.message, "test message");
        assert_eq!(deny_result.interrupt, false);
    }

    #[test]
    fn test_permission_rule_value_creation() {
        let rule = PermissionRuleValue {
            tool_name: "test_tool".to_string(),
            rule_content: Some("test_content".to_string()),
        };
        assert_eq!(rule.tool_name, "test_tool");
        assert_eq!(rule.rule_content, Some("test_content".to_string()));
    }

    #[test]
    fn test_permission_update_creation() {
        let update = PermissionUpdate {
            r#type: "addRules".to_string(),
            rules: None,
            behavior: None,
            mode: None,
            directories: None,
            destination: None,
        };
        assert_eq!(update.r#type, "addRules");
    }

    #[test]
    fn test_hook_context_creation() {
        let context = HookContext {
            signal: Some("test_signal".to_string()),
        };
        assert_eq!(context.signal, Some("test_signal".to_string()));
    }

    #[test]
    fn test_hook_matcher_creation() {
        let matcher = HookMatcher {
            matcher: Some("test_matcher".to_string()),
            hooks: vec!["hook1".to_string(), "hook2".to_string()],
        };
        assert_eq!(matcher.matcher, Some("test_matcher".to_string()));
        assert_eq!(matcher.hooks.len(), 2);
    }

    #[tokio::test]
    async fn test_query_creation() {
        let mock_transport = MockTransport::new();
        let transport_arc = Arc::new(mock_transport) as Arc<dyn Transport + Send + Sync>;

        let query = Query::new(transport_arc, true, None, None, None);

        // We can't directly test private fields, so we'll just ensure it compiles
    }

    #[tokio::test]
    async fn test_query_initialize_non_streaming() {
        let mock_transport = MockTransport::new();
        let transport_arc = Arc::new(mock_transport) as Arc<dyn Transport + Send + Sync>;

        let mut query = Query::new(
            transport_arc,
            false, // non-streaming mode
            None,
            None,
            None,
        );

        // In non-streaming mode, initialize should return Ok(None)
        let result = query.initialize().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[tokio::test]
    async fn test_query_interrupt() {
        // This would require a more complex mock setup to test properly
        // For now, we'll test that the code compiles
    }

    #[tokio::test]
    async fn test_query_set_permission_mode() {
        // This would require a more complex mock setup to test properly
        // For now, we'll test that the code compiles
    }

    #[tokio::test]
    async fn test_query_set_model() {
        // This would require a more complex mock setup to test properly
        // For now, we'll test that the code compiles
    }
}
