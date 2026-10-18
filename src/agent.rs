use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
enum ThinkingLevel {
    Low,
    Medium,
    High,
    None,
}

impl fmt::Display for ThinkingLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

struct ToolSchema{
    schema_type: Vec<String>,
    properties: HashMap<String, String>,
}

struct AgentTool {
    name: String,
    description: String,
    parameters: ToolSchema,
    execute: Box<dyn FnMut(String)>,
    model_config: HashMap<String, bool>,
}

impl AgentTool {
    pub fn new(
        name: String,
        description: String,
        parameters: ToolSchema,
    ) -> Self {
        AgentTool{
        name,
        description,
        parameters,
            execute: Box::new(|param1: String| -> (){}),
        model_config: HashMap::from([("arbitrary_types_allowed".to_string(), true)]),
        }
    }
}

trait AgentMessage {}

struct UserMessage {}
impl AgentMessage for UserMessage {}

struct AssistantMessage {}

struct ToolResultMessage {}

struct AgentState {
    messages: Vec<Box<dyn AgentMessage>>,
    tools: Vec<AgentTool>,
    model: String,
    thinking_level: ThinkingLevel,
    is_streaming: bool,
    custom: HashMap<String, String>,
    model_config: HashMap<String, bool>,
}
impl AgentState {
    pub fn new() -> Self {
        AgentState {
            messages: Vec::new(),
            tools: Vec::new(),
            model: "dolphin-llama3:latest".to_string(),
            thinking_level: ThinkingLevel::None,
            is_streaming: false,
            custom: HashMap::new(),
            model_config: HashMap::from([("arbitrary_types_allowed".to_string(), false)]),
        }
    }
}

struct AgentOptions {
    model: String,
    system_prompt: String,
    thinking_level: ThinkingLevel,
    ollama_base_url: String,
    logging_event_filter: Vec<String>,
    convert_to_llm: Box<dyn FnMut(Box<dyn AgentMessage>)>,
}

struct Agent {
    options: AgentOptions,
}

impl Agent {
    pub fn new(options: AgentOptions) -> Self {
        Agent {
            options,
        }
    }
}
