// msg type: text 文本

#[derive(Debug, Serialize, Deserialize)]
pub struct Text {
    #[serde(rename = "text")]
    text: String,
}

impl Text {
    pub fn new(msg: String) -> Self {
        Text { text: msg }
    }
}

// msg type: interactive 消息卡片

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    #[serde(rename = "config")]
    config: Config,

    #[serde(rename = "header")]
    header: Header,

    #[serde(rename = "elements")]
    card_elements: Vec<CardElement>,
}

impl Card {
    pub fn new(title: String, msgs: Vec<String>, url: Option<String>) -> Self {
        let mut card_elements = Vec::new();
        for msg in msgs {
            card_elements.push(CardElement::new_text(msg))
        }
        if url.is_some() {
            card_elements.push(CardElement::new_url_button(url.unwrap()))
        }
        Card {
            config: Config { enable_forward: true },
            header: Header {
                title: Context {
                    tag: "plain_text".to_string(),
                    content: title,
                },
                template: "wathet".to_string(),
            },
            card_elements,
        }
    }

    pub fn set_title_color(&mut self, color: &str) {
        self.header.template = color.to_string()
    }

    pub fn append_elements(&mut self, element: CardElement) {
        self.card_elements.push(element)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "enable_forward")]
    enable_forward: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    #[serde(rename = "title")]
    title: Context,

    #[serde(rename = "template")]
    template: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardElement {
    #[serde(rename = "tag")]
    tag: String,

    #[serde(rename = "text")]
    text: Option<Context>,

    #[serde(rename = "actions")]
    actions: Option<Vec<Action>>,

    #[serde(rename = "elements")]
    note: Option<Vec<Context>>,
}

impl CardElement {
    pub fn new_text(msg: String) -> Self {
        CardElement {
            tag: "div".to_string(),
            text: Some(Context {
                tag: "lark_md".to_string(),
                content: msg,
            }),
            actions: None,
            note: None,
        }
    }

    pub fn new_url_button(url: String) -> Self {
        CardElement {
            tag: "action".to_string(),
            text: None,
            actions: Some(vec![Action {
                tag: "button".to_string(),
                text: Context {
                    tag: "lark_md".to_string(),
                    content: "查看详情".to_string(),
                },
                url,
                action_type: "primary".to_string(),
            }]),
            note: None,
        }
    }

    pub fn new_note(note: String) -> Self {
        CardElement {
            tag: "note".to_string(),
            text: None,
            actions: None,
            note: Some(vec![Context {
                tag: "lark_md".to_string(),
                content: note,
            }]),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    #[serde(rename = "tag")]
    tag: String,

    #[serde(rename = "text")]
    text: Context,

    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "type")]
    action_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Context {
    #[serde(rename = "tag")]
    tag: String,

    #[serde(rename = "content")]
    content: String,
}
