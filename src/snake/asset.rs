use bevy::asset::AsyncReadExt;
use strum::IntoEnumIterator;

use crate::snake::rendering::{SnakeBody, SnakeFrame};

use super::*;

#[derive(Asset, TypePath)]
pub struct SnakeType {
    pub id: SnakeId,
    pub body: SnakeBody,
}

impl SnakeType {
    pub fn get_frame(&self, index: usize) -> &SnakeFrame {
        match &self.body {
            SnakeBody::Single(frame) => frame,
            SnakeBody::Animated(frames) => &frames[index % frames.len()],
        }
    }
}

#[derive(Default)]
pub struct SnakeLoader;

impl bevy::asset::AssetLoader for SnakeLoader {
    type Asset = SnakeType;
    type Settings = ();
    type Error = SnakeAssetError;

    fn extensions(&self) -> &[&str] {
        &["snake"]
    }

    fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &Self::Settings,
        load_context: &mut bevy::asset::LoadContext,
    ) -> impl bevy::tasks::ConditionalSendFuture<Output = std::result::Result<Self::Asset, Self::Error>>
    {
        async move {
            let mut data = String::new();
            reader.read_to_string(&mut data).await?;
            SnakeLoader::parse_snake_asset(&data, load_context)
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SnakeAssetError {
    #[error("Key:Value pair is malformed")]
    MalformedKeyValue,
    #[error("Snake asset is missing required fields: `{0}`")]
    MissingFields(&'static str),
    #[error("Invalid Value for field: {0}: {1}")]
    InvalidValue(&'static str, String),
    #[error("Unknown Key `{0}` in {1} block")]
    UnknownKey(String, &'static str),
    #[error("I/O Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Missing Trailing Comma")]
    MissingTrailingComma,
    #[error("Mismatched Brackets: found `{0}` expected `{1}`")]
    MismatchedBrackets(char, char),
    #[error("Unmatched closing bracket: `{0}`")]
    UnmatchedClosingBracket(char),
    #[error("Block Missing {} '{0}'", if *.1 { "leading" } else { "trailing" })]
    BlockNotWrappedIn(char, bool),
}

impl SnakeLoader {
    fn parse_snake_asset(
        data: &str,
        ctx: &mut bevy::asset::LoadContext,
    ) -> Result<SnakeType, SnakeAssetError> {
        let mut id = None;
        let mut body = None;
        for block in SnakeBlock::new(data) {
            let block = block?;
            let (key, value) = block
                .split_once(':')
                .ok_or(SnakeAssetError::MalformedKeyValue)?;
            match key.trim() {
                "id" => {
                    id = Some(SnakeLoader::parse_snake_id(value.trim())?);
                }
                "body" => {
                    let frame = SnakeLoader::parse_snake_frame(value.trim(), ctx)?;
                    match body {
                        None => {
                            body = Some(SnakeBody::Single(frame));
                        }
                        Some(SnakeBody::Single(first)) => {
                            let frames = vec![first, frame];
                            body = Some(SnakeBody::Animated(frames));
                        }
                        Some(SnakeBody::Animated(ref mut frames)) => {
                            frames.push(frame);
                        }
                    }
                }
                k => {
                    return Err(SnakeAssetError::UnknownKey(k.to_string(), "root"));
                }
            }
        }
        let Some(id) = id else {
            return Err(SnakeAssetError::MissingFields("id"));
        };
        let Some(body) = body else {
            return Err(SnakeAssetError::MissingFields("body"));
        };
        Ok(SnakeType { id, body })
    }

    fn parse_snake_id(data: &str) -> Result<SnakeId, SnakeAssetError> {
        for snake_id in SnakeId::iter() {
            let name: &'static str = snake_id.into();
            if name.eq_ignore_ascii_case(data.trim()) {
                return Ok(snake_id);
            }
        }
        Err(SnakeAssetError::MalformedKeyValue)
    }

    fn parse_snake_frame(
        data: &str,
        ctx: &mut bevy::asset::LoadContext,
    ) -> Result<SnakeFrame, SnakeAssetError> {
        let mut frame = SnakeFrame::EMPTY;

        let data = peel(data, '{')?;
        for block in SnakeBlock::new(data) {
            let block = block?;
            let (key, value) = block
                .split_once(':')
                .ok_or(SnakeAssetError::MalformedKeyValue)?;
            let value = peel(value.trim(), '"')?;
            match key.trim() {
                "head" => {
                    frame.head = ctx.load(value.trim().to_string());
                }
                "straight" => {
                    frame.body_straight = ctx.load(value.trim().to_string());
                }
                "curve" => {
                    frame.body_curve = ctx.load(value.trim().to_string());
                }
                "tail" => {
                    frame.tail = ctx.load(value.trim().to_string());
                }
                k => {
                    return Err(SnakeAssetError::UnknownKey(k.to_string(), "body"));
                }
            }
        }

        Ok(frame)
    }
}
struct SnakeBlock<'a> {
    haystack: &'a str,
    stack: Vec<char>,
}

impl<'a> SnakeBlock<'a> {
    fn new(haystack: &'a str) -> SnakeBlock<'a> {
        SnakeBlock {
            haystack: haystack.trim(),
            stack: Vec::new(),
        }
    }
    fn next_block(&mut self) -> Result<&'a str, SnakeAssetError> {
        let mut chars = self.haystack.char_indices();
        let mut in_string = false;
        while let Some((i, c)) = chars.next() {
            match c {
                '"' => in_string = !in_string,
                '\\' if in_string => {
                    // skip next char
                    chars.next();
                }
                '{' | '(' | '[' if !in_string => self.stack.push(c),
                ',' if !in_string && self.stack.is_empty() => {
                    let out = Ok(self.haystack[..i].trim());
                    self.haystack = &self.haystack[i + 1..];
                    return out;
                }
                '}' | ')' | ']' if !in_string => {
                    let open = match c {
                        '}' => '{',
                        ')' => '(',
                        ']' => '[',
                        _ => unreachable!(),
                    };
                    if let Some(last) = self.stack.pop() {
                        if last != open {
                            // mismatched closing
                            return Err(SnakeAssetError::MismatchedBrackets(c, brack_recp(last)));
                        }
                    } else {
                        // unmatched closing
                        return Err(SnakeAssetError::UnmatchedClosingBracket(c));
                    }
                }
                _ => {}
            }
        }
        let out = Ok(self.haystack.trim());
        self.haystack = "";
        out
    }
}

impl<'a> Iterator for SnakeBlock<'a> {
    type Item = Result<&'a str, SnakeAssetError>;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.next_block();
        if let Ok("") = r { None } else { Some(r) }
    }
}

fn brack_recp(char: char) -> char {
    match char {
        '{' => '}',
        '}' => '{',
        '(' => ')',
        ')' => '(',
        '[' => ']',
        ']' => '[',
        _ => char,
    }
}

fn peel(res: &str, wrapping: char) -> Result<&str, SnakeAssetError> {
    let recp = brack_recp(wrapping);
    res.trim()
        .strip_prefix(wrapping)
        .ok_or(SnakeAssetError::BlockNotWrappedIn(wrapping, true))?
        .strip_suffix(recp)
        .ok_or(SnakeAssetError::BlockNotWrappedIn(recp, false))
}
