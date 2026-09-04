//! Finite token, delimiter, attachment, and span admission.

const ITEM_LIMIT: u32 = 4_096;

/// Half-open byte span.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

/// Supported delimiter kinds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Delimiter {
    Parenthesis,
    Bracket,
    Brace,
}

/// Supported prefix attachment kinds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrefixKind {
    Quote,
    Quasiquote,
    Unquote,
    Splice,
    SyntaxQuote,
    SyntaxQuasiquote,
    SyntaxUnquote,
    SyntaxSplice,
    DatumComment,
}

/// Token classes in the bounded reader subset.
// r[impl scheme-logic.reader-token-span]
// r[impl scheme-logic.reader-datum-structure]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Atom,
    Open(Delimiter),
    Close(Delimiter),
    VectorOpen,
    BytevectorOpen,
    Prefix(PrefixKind),
}

/// One token and its source span.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

/// Explicit matching-delimiter metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DelimiterPair {
    pub open_index: u32,
    pub close_index: u32,
}

/// Explicit prefix-to-datum attachment metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Attachment {
    pub prefix_index: u32,
    pub datum_index: u32,
}

/// Explicit parent-child span relation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpanRelation {
    pub parent: Span,
    pub child: Span,
}

/// Complete finite syntax envelope.
// r[impl scheme-logic.reader-token-span]
// r[impl scheme-logic.reader-datum-structure]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReaderEnvelope {
    pub source_length: u32,
    pub tokens: Vec<Token>,
    pub delimiter_pairs: Vec<DelimiterPair>,
    pub attachments: Vec<Attachment>,
    pub child_spans: Vec<SpanRelation>,
}

/// Stable reader rejection classes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReaderFault {
    TooManyItems,
    InvalidSpan,
    TokenOrder,
    DelimiterIndex,
    DelimiterOrder,
    DelimiterKind,
    DuplicateDelimiter,
    UnmatchedDelimiter,
    CrossingDelimiter,
    AttachmentIndex,
    AttachmentOrder,
    AttachmentKind,
    DuplicateAttachment,
    MissingAttachment,
    ChildOutsideParent,
}

/// Deterministic reader rejection metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReaderFaultSite {
    pub class: ReaderFault,
    pub index: Option<u32>,
}

fn count<T>(items: &[T]) -> Option<u32> {
    u32::try_from(items.len()).ok()
}

fn position(index: usize) -> Option<u32> {
    u32::try_from(index).ok()
}

fn reject(class: ReaderFault, index: Option<u32>) -> Result<(), ReaderFaultSite> {
    Err(ReaderFaultSite { class, index })
}

fn span_admitted(span: Span, bound: u32) -> bool {
    span.start < span.end && span.end <= bound
}

fn expected_delimiter(kind: TokenKind) -> Option<Delimiter> {
    match kind {
        TokenKind::Open(delimiter) => Some(delimiter),
        TokenKind::VectorOpen | TokenKind::BytevectorOpen => Some(Delimiter::Parenthesis),
        TokenKind::Atom | TokenKind::Close(_) | TokenKind::Prefix(_) => None,
    }
}

fn is_datum_start(kind: TokenKind) -> bool {
    !matches!(kind, TokenKind::Close(_))
}

fn relation_contains(relation: SpanRelation, source_length: u32) -> bool {
    span_admitted(relation.parent, source_length)
        && span_admitted(relation.child, source_length)
        && relation.parent.start <= relation.child.start
        && relation.child.end <= relation.parent.end
}

fn validate_tokens(envelope: &ReaderEnvelope) -> Result<(), ReaderFaultSite> {
    for (index, token) in envelope.tokens.iter().enumerate() {
        if !span_admitted(token.span, envelope.source_length) {
            return reject(ReaderFault::InvalidSpan, position(index));
        }
        if index > 0 && envelope.tokens[index - 1].span.end > token.span.start {
            return reject(ReaderFault::TokenOrder, position(index));
        }
    }
    Ok(())
}

fn validate_pairs(envelope: &ReaderEnvelope) -> Result<(), ReaderFaultSite> {
    let token_count = count(&envelope.tokens).ok_or(ReaderFaultSite {
        class: ReaderFault::TooManyItems,
        index: None,
    })?;
    for (index, pair) in envelope.delimiter_pairs.iter().enumerate() {
        let at = position(index);
        if pair.open_index >= token_count || pair.close_index >= token_count {
            return reject(ReaderFault::DelimiterIndex, at);
        }
        if pair.open_index >= pair.close_index {
            return reject(ReaderFault::DelimiterOrder, at);
        }
        let open_index = usize::try_from(pair.open_index).map_err(|_| ReaderFaultSite {
            class: ReaderFault::DelimiterIndex,
            index: at,
        })?;
        let close_index = usize::try_from(pair.close_index).map_err(|_| ReaderFaultSite {
            class: ReaderFault::DelimiterIndex,
            index: at,
        })?;
        let open = envelope.tokens[open_index].kind;
        let close = envelope.tokens[close_index].kind;
        if expected_delimiter(open).is_none_or(|expected| close != TokenKind::Close(expected)) {
            return reject(ReaderFault::DelimiterKind, at);
        }
        let is_duplicate = envelope.delimiter_pairs[..index]
            .iter()
            .any(|prior| prior.open_index == pair.open_index || prior.close_index == pair.close_index);
        if is_duplicate {
            return reject(ReaderFault::DuplicateDelimiter, at);
        }
        let is_crossing = envelope.delimiter_pairs[..index].iter().any(|prior| {
            prior.open_index < pair.open_index
                && pair.open_index < prior.close_index
                && prior.close_index < pair.close_index
                || pair.open_index < prior.open_index
                    && prior.open_index < pair.close_index
                    && pair.close_index < prior.close_index
        });
        if is_crossing {
            return reject(ReaderFault::CrossingDelimiter, at);
        }
    }
    for (index, token) in envelope.tokens.iter().enumerate() {
        let token_index = u32::try_from(index).map_err(|_| ReaderFaultSite {
            class: ReaderFault::TooManyItems,
            index: None,
        })?;
        let is_matched = match token.kind {
            TokenKind::Open(_) | TokenKind::VectorOpen | TokenKind::BytevectorOpen => {
                envelope.delimiter_pairs.iter().any(|pair| pair.open_index == token_index)
            }
            TokenKind::Close(_) => envelope.delimiter_pairs.iter().any(|pair| pair.close_index == token_index),
            TokenKind::Atom | TokenKind::Prefix(_) => true,
        };
        if !is_matched {
            return reject(ReaderFault::UnmatchedDelimiter, Some(token_index));
        }
    }
    Ok(())
}

fn validate_attachments(envelope: &ReaderEnvelope) -> Result<(), ReaderFaultSite> {
    let token_count = count(&envelope.tokens).ok_or(ReaderFaultSite {
        class: ReaderFault::TooManyItems,
        index: None,
    })?;
    for (index, attachment) in envelope.attachments.iter().enumerate() {
        let at = position(index);
        if attachment.prefix_index >= token_count || attachment.datum_index >= token_count {
            return reject(ReaderFault::AttachmentIndex, at);
        }
        if attachment.prefix_index.checked_add(1) != Some(attachment.datum_index) {
            return reject(ReaderFault::AttachmentOrder, at);
        }
        let prefix_index = usize::try_from(attachment.prefix_index).map_err(|_| ReaderFaultSite {
            class: ReaderFault::AttachmentIndex,
            index: at,
        })?;
        let datum_index = usize::try_from(attachment.datum_index).map_err(|_| ReaderFaultSite {
            class: ReaderFault::AttachmentIndex,
            index: at,
        })?;
        let prefix = envelope.tokens[prefix_index];
        let datum = envelope.tokens[datum_index];
        if !matches!(prefix.kind, TokenKind::Prefix(_)) || !is_datum_start(datum.kind) {
            return reject(ReaderFault::AttachmentKind, at);
        }
        if prefix.span.end > datum.span.start {
            return reject(ReaderFault::AttachmentOrder, at);
        }
        if envelope.attachments[..index].iter().any(|prior| prior.prefix_index == attachment.prefix_index) {
            return reject(ReaderFault::DuplicateAttachment, at);
        }
    }
    for (index, token) in envelope.tokens.iter().enumerate() {
        if matches!(token.kind, TokenKind::Prefix(_)) {
            let token_index = u32::try_from(index).map_err(|_| ReaderFaultSite {
                class: ReaderFault::TooManyItems,
                index: None,
            })?;
            if !envelope.attachments.iter().any(|attachment| attachment.prefix_index == token_index) {
                return reject(ReaderFault::MissingAttachment, Some(token_index));
            }
        }
    }
    Ok(())
}

/// Admits the bounded reader structure without parsing source text.
// r[impl scheme-logic.reader-token-span]
// r[impl scheme-logic.reader-datum-structure]
pub fn admit_reader(envelope: &ReaderEnvelope) -> Result<(), ReaderFaultSite> {
    let counts = [
        count(&envelope.tokens),
        count(&envelope.delimiter_pairs),
        count(&envelope.attachments),
        count(&envelope.child_spans),
    ];
    if counts.iter().any(|item_count| item_count.is_none_or(|value| value > ITEM_LIMIT)) {
        return reject(ReaderFault::TooManyItems, None);
    }
    validate_tokens(envelope)?;
    validate_pairs(envelope)?;
    validate_attachments(envelope)?;
    if let Some(index) = envelope.child_spans.iter().position(|relation| !relation_contains(*relation, envelope.source_length)) {
        return reject(ReaderFault::ChildOutsideParent, position(index));
    }
    Ok(())
}
