#![allow(unused_attributes)]
#![rustfmt::skip]
use web_sys as dom;

html_element!(
    /// The [HTML `<a>` element (or *anchor* element)][mdn], along with its href attribute, creates
    /// a hyperlink to other web pages, files, locations within the same page, email addresses, or
    /// any other URL.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a
    a {
        /// Prompts the user to save the linked URL instead of navigating to it. Can be used with or
        /// without a value:
        /// 
        /// * Without a value, the browser will suggest a filename/extension, generated from various
        /// sources:
        /// * The Content-Disposition HTTP header
        /// * The final segment in the URL path
        /// * The media type (from the (Content-Type header, the start of a data: URL, or
        /// Blob.type for a blob: URL)
        /// * Defining a value suggests it as the filename. `/` and `\` characters are converted to
        /// underscores (_). Filesystems may forbid other characters in filenames, so browsers
        /// will adjust the suggested name if necessary.
        /// 
        /// > Notes:
        /// > * download only works for same-origin URLs, or the blob: and data: schemes.
        /// > * If Content-Disposition has a different filename than download, the header takes
        /// >   priority. (If `Content-Disposition: inline`, Firefox prefers the header while Chrome
        /// >   prefers download.)
        download: String,

        /// The URL that the hyperlink points to. Links are not restricted to HTTP-based URLs —
        /// they can use any URL scheme supported by browsers:
        /// 
        /// * Sections of a page with fragment URLs
        /// * Pieces of media files with media fragments
        /// * Telephone numbers with tel: URLs
        /// * Email addresses with mailto: URLs
        /// * While web browsers may not support other URL schemes, web sites can with
        /// registerProtocolHandler()
        href: String,

        /// Hints at the human language of the linked URL. No built-in functionality. Allowed values
        /// are the same as the global lang attribute.
        hreflang: String,

        /// A space-separated list of URLs. When the link is followed, the browser will send POST
        /// requests with the body PING to the URLs. Typically for tracking.
        ping: String,

        /// The relationship of the linked URL as space-separated link types.
        rel: String,

        /// Where to display the linked URL, as the name for a browsing context (a tab, window, or
        /// `<iframe>`). The following keywords have special meanings for where to load the URL:
        /// 
        /// * `_self`: the current browsing context. (Default)
        /// * `_blank`: usually a new tab, but users can configure browsers to open a new window
        /// instead.
        /// * `_parent`: the parent browsing context of the current one. If no parent, behaves as
        /// _self.
        /// * `_top`: the topmost browsing context (the "highest" context that’s an ancestor of the
        /// current one). If no ancestors, behaves as _self.
        /// 
        /// > Note: When using target, add rel="noreferrer noopener" to avoid exploitation of the
        /// window.opener API;
        /// 
        /// > Note: Linking to another page with target="_blank" will run the new page in the same
        /// process as your page. If the new page executes JavaScript, your page's performance may
        /// suffer. This can also be avoided by using rel="noreferrer noopener".
        target: String,

        /// Hints at the linked URL’s format with a MIME type. No built-in functionality.
        type_: String,

    }
);

dom_type!(a <dom::HtmlAnchorElement>);
text_parent!(a);

html_element!(
    /// The [HTML Abbreviation element (`<abbr>`)][mdn] represents an abbreviation or acronym; the
    /// optional [`title`][title] attribute can provide an expansion or description for the
    /// abbreviation.
    /// 
    /// The title attribute has a specific semantic meaning when used with the `<abbr>` element; it
    /// must contain a full human-readable description or expansion of the abbreviation. This text
    /// is often presented by browsers as a tooltip when the mouse cursor is hovered over the
    /// element.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/abbr
    /// [title]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes#attr-title
    abbr {
    }
);

dom_type!(abbr <dom::HtmlElement>);
text_parent!(abbr);

html_element!(
    /// The [HTML Bring Attention To element (`<b>`)][mdn] is used to draw the reader's attention to
    /// the element's contents, which are not otherwise granted special importance.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/b
    b {
    }
);

dom_type!(b <dom::HtmlElement>);
text_parent!(b);

html_element!(
    /// The [HTML Bidirectional Isolate element (`<bdi>`)][mdn] tells the browser's bidirectional
    /// algorithm to treat the text it contains in isolation from its surrounding text.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/bdi
    bdi {
    }
);

dom_type!(bdi <dom::HtmlElement>);
text_parent!(bdi);

html_element!(
    /// The [HTML Bidirectional Text Override element (`<bdo>`)][mdn] overrides the current
    /// directionality of text, so that the text within is rendered in a different direction.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/bdo
    bdo {
        /// The direction in which text should be rendered in this element's contents. Possible
        /// values are:
        /// 
        /// * `ltr`: Indicates that the text should go in a left-to-right direction.
        /// * `rtl`: Indicates that the text should go in a right-to-left direction.
        dir: String,

    }
);

dom_type!(bdo <dom::HtmlElement>);
text_parent!(bdo);

html_element!(
    /// The [HTML `<br>` element][mdn] produces a line break in text (carriage-return). It is useful
    /// for writing a poem or an address, where the division of lines is significant.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/br
    br {
    }
);

dom_type!(br <dom::HtmlBrElement>);

html_element!(
    /// The [HTML Citation element (`<cite>`)][mdn] is used to describe a reference to a cited
    /// creative work, and must include the title of that work.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/cite
    cite {
    }
);

dom_type!(cite <dom::HtmlElement>);
text_parent!(cite);

html_element!(
    /// The [HTML `<code>` element][mdn] displays its contents styled in a fashion intended to
    /// indicate that the text is a short fragment of computer code.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/code
    code {
    }
);

dom_type!(code <dom::HtmlElement>);
text_parent!(code);

html_element!(
    /// The [HTML `<data>` element][mdn] links a given content with a machine-readable translation.
    /// If the content is time- or date-related, the [`<time>`][time] element must be used.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/data
    /// [time]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/time
    data {
        /// This attribute specifies the machine-readable translation of the content of the element.
        value: String,

    }
);

dom_type!(data <dom::HtmlDataElement>);
text_parent!(data);

html_element!(
    /// The [HTML Definition element (`<dfn>`)][mdn] is used to indicate the term being defined
    /// within the context of a definition phrase or sentence.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dfn
    dfn {
    }
);

dom_type!(dfn <dom::HtmlElement>);
text_parent!(dfn);

html_element!(
    /// The [HTML `<em>` element][mdn] marks text that has stress emphasis. The `<em>` element can
    /// be nested, with each level of nesting indicating a greater degree of emphasis.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/em
    em {
    }
);

dom_type!(em <dom::HtmlElement>);
text_parent!(em);

html_element!(
    /// The [HTML `<i>` element][mdn] represents a range of text that is set off from the normal
    /// text for some reason. Some examples include technical terms, foreign language phrases, or
    /// fictional character thoughts. It is typically displayed in italic type.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/i
    i {
    }
);

dom_type!(i <dom::HtmlElement>);
text_parent!(i);

html_element!(
    /// The [HTML Keyboard Input element (`<kbd>`)][mdn] represents a span of inline text denoting
    /// textual user input from a keyboard, voice input, or any other text entry device.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/kbd
    kbd {
    }
);

dom_type!(kbd <dom::HtmlElement>);
text_parent!(kbd);

html_element!(
    /// The [HTML Mark Text element (`<mark>`)][mdn] represents text which is marked or highlighted
    /// for reference or notation purposes, due to the marked passage's relevance or importance in
    /// the enclosing context.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/mark
    mark {
    }
);

dom_type!(mark <dom::HtmlElement>);
text_parent!(mark);

html_element!(
    /// The [HTML `<q>` element][mdn]  indicates that the enclosed text is a short inline quotation.
    /// Most modern browsers implement this by surrounding the text in quotation marks.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/q
    q {
        /// The value of this attribute is a URL that designates a source document or message for
        /// the information quoted. This attribute is intended to point to information explaining
        /// the context or the reference for the quote.
        cite: String,

    }
);

dom_type!(q <dom::HtmlQuoteElement>);
text_parent!(q);

html_element!(
    /// The [HTML Ruby Base (`<rb>`) element][mdn] is used to delimit the base text component of
    /// a [`<ruby>`][ruby] annotation, i.e. the text that is being annotated.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/rb
    /// [ruby]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ruby
    rb {
    }
);

dom_type!(rb <dom::HtmlElement>);
text_parent!(rb);

html_element!(
    /// The [HTML Ruby Fallback Parenthesis (`<rp>`) element][mdn] is used to provide fall-back
    /// parentheses for browsers that do not support display of ruby annotations using the
    /// [`<ruby>`][ruby] element.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/rp
    /// [ruby]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ruby
    rp {
    }
);

dom_type!(rp <dom::HtmlElement>);
text_parent!(rp);

html_element!(
    /// The [HTML Ruby Text (`<rt>`) element][mdn] specifies the ruby text component of a ruby
    /// annotation, which is used to provide pronunciation, translation, or transliteration
    /// information for East Asian typography. The `<rt>` element must always be contained within a
    /// [`<ruby>`][ruby] element.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/rt
    /// [ruby]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ruby
    rt {
    }
);

dom_type!(rt <dom::HtmlElement>);
text_parent!(rt);

html_element!(
    /// The [HTML Ruby Text Container (`<rtc>`) element][mdn] embraces semantic annotations of
    /// characters presented in a ruby of [`<rb>`][rb] elements used inside of [`<ruby>`][ruby]
    /// element. [`<rb>`][rb] elements can have both pronunciation ([`<rt>`][rt]) and semantic
    /// ([`<rtc>`][rtc]) annotations.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/rtc
    /// [rb]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/rb
    /// [ruby]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ruby
    /// [rt]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/rt
    /// [rtc]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/rtc
    rtc {
    }
);

dom_type!(rtc <dom::HtmlElement>);
text_parent!(rtc);

html_element!(
    /// The [HTML `<ruby>` element][mdn] represents a ruby annotation. Ruby annotations are for
    /// showing pronunciation of East Asian characters.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ruby
    ruby {
    }
);

dom_type!(ruby <dom::HtmlElement>);
text_parent!(ruby);

html_element!(
    /// The [HTML `<s>` element][mdn] renders text with a strikethrough, or a line through it. Use
    /// the `<s>` element to represent things that are no longer relevant or no longer accurate.
    /// However, `<s>` is not appropriate when indicating document edits; for that, use the
    /// [`<del>`][del] and [`<ins>`][ins] elements, as appropriate.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/s
    /// [del]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/del
    /// [ins]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ins
    s {
    }
);

dom_type!(s <dom::HtmlElement>);
text_parent!(s);

html_element!(
    /// The [HTML Sample Element (`<samp>`)][mdn] is used to enclose inline text which represents
    /// sample (or quoted) output from a computer program.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/samp
    samp {
    }
);

dom_type!(samp <dom::HtmlElement>);
text_parent!(samp);

html_element!(
    /// The [HTML `<small>` element][mdn] represents side-comments and small print, like copyright
    /// and legal text, independent of its styled presentation. By default, it renders text within
    /// it one font-size small, such as from `small` to `x-small`.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/small
    small {
    }
);

dom_type!(small <dom::HtmlElement>);
text_parent!(small);

html_element!(
    /// The [HTML `<span>` element][mdn] is a generic inline container for phrasing content, which
    /// does not inherently represent anything. It can be used to group elements for styling
    /// purposes (using the [`class`][class] or [`id`][id] attributes), or because they share
    /// attribute values, such as [`lang`][lang].
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/span
    /// [class]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes#attr-class
    /// [id]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes#attr-id
    /// [lang]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes#attr-lang
    span {
    }
);

dom_type!(span <dom::HtmlSpanElement>);
text_parent!(span);

html_element!(
    /// The [HTML Strong Importance Element (`<strong>`)][mdn] indicates that its contents have
    /// strong importance, seriousness, or urgency. Browsers typically render the contents in bold
    /// type.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/strong
    strong {
    }
);

dom_type!(strong <dom::HtmlElement>);
text_parent!(strong);

html_element!(
    /// The [HTML Subscript element (`<sub>`)][mdn] specifies inline text which should be displayed
    /// as subscript for solely typographical reasons.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/sub
    sub {
    }
);

dom_type!(sub <dom::HtmlElement>);
text_parent!(sub);

html_element!(
    /// The [HTML Superscript element (`<sup>`)][mdn] specifies inline text which is to be displayed
    /// as superscript for solely typographical reasons.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/sup
    sup {
    }
);

dom_type!(sup <dom::HtmlElement>);
text_parent!(sup);

html_element!(
    /// The [HTML `<time>` element][mdn] represents a specific period in time.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/time
    time {
        /// This attribute indicates the time and/or date of the element and must be in one of the
        /// formats described below.
        datetime: String,

    }
);

dom_type!(time <dom::HtmlTimeElement>);
text_parent!(time);

html_element!(
    /// The [HTML Unarticulated Annotation Element (`<u>`)][mdn] represents a span of inline text
    /// which should be rendered in a way that indicates that it has a non-textual annotation.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/u
    u {
    }
);

dom_type!(u <dom::HtmlElement>);
text_parent!(u);

html_element!(
    /// The [HTML Variable element (`<var>`)][mdn] represents the name of a variable in a
    /// mathematical expression or a programming context.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/var
    var {
    }
);

dom_type!(var <dom::HtmlElement>);
text_parent!(var);

html_element!(
    /// The [HTML `<wbr>` element][mdn] represents a word break opportunity—a position within text
    /// where the browser may optionally break a line, though its line-breaking rules would not
    /// otherwise create a break at that location.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/wbr
    wbr {
    }
);

dom_type!(wbr <dom::HtmlElement>);

html_element!(
    /// The [HTML `<del>` element][mdn] represents a range of text that has been deleted from a
    /// document.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/del
    del {
        /// A URI for a resource that explains the change (for example, meeting minutes).
        cite: String,

        /// This attribute indicates the time and date of the change and must be a valid date string
        /// with an optional time. If the value cannot be parsed as a date with an optional time
        /// string, the element does not have an associated time stamp. For the format of the string
        /// without a time, see Date strings. The format of the string if it includes both date and
        /// time is covered in Local date and time strings.
        datetime: String,

    }
);

dom_type!(del <dom::HtmlModElement>);
text_parent!(del);

html_element!(
    /// The [HTML `<ins>` element][mdn] represents a range of text that has been added to a
    /// document.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ins
    ins {
        /// A URI for a resource that explains the change (for example, meeting minutes).
        cite: String,

        /// This attribute indicates the time and date of the change and must be a valid date string
        /// with an optional time. If the value cannot be parsed as a date with an optional time
        /// string, the element does not have an associated time stamp. For the format of the string
        /// without a time, see Date strings. The format of the string if it includes both date and
        /// time is covered in Local date and time strings.
        datetime: String,

    }
);

dom_type!(ins <dom::HtmlModElement>);
text_parent!(ins);

html_element!(
    /// The [HTML `<address>` element][mdn] indicates that the enclosed HTML provides contact
    /// information for a person or people, or for an organization.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/address
    address {
    }
);

dom_type!(address <dom::HtmlElement>);
text_parent!(address);

html_element!(
    /// The [HTML `<article>` element][mdn] represents a self-contained composition in a document,
    /// page, application, or site, which is intended to be independently distributable or reusable
    /// (e.g., in syndication).
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/article
    article {
    }
);

dom_type!(article <dom::HtmlElement>);
text_parent!(article);

html_element!(
    /// The [HTML `<aside>` element][mdn] represents a portion of a document whose content is only
    /// indirectly related to the document's main content.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/aside
    aside {
    }
);

dom_type!(aside <dom::HtmlElement>);
text_parent!(aside);

html_element!(
    /// The [HTML `<footer>` element][mdn] represents a footer for its nearest [sectioning content]
    /// or [sectioning root] element. A footer typically contains information about the author of
    /// the section, copyright data or links to related documents.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/footer
    /// [sectioning content]: https://developer.mozilla.org/en-US/docs/Web/Guide/HTML/Content_categories#Sectioning_content
    /// [sectioning root]: https://developer.mozilla.org/en-US/docs/Web/Guide/HTML/Sections_and_Outlines_of_an_HTML5_document#Sectioning_roots
    footer {
    }
);

dom_type!(footer <dom::HtmlElement>);
text_parent!(footer);

html_element!(
    /// The [HTML `<header>` element][mdn] represents introductory content, typically a group of
    /// introductory or navigational aids. It may contain some heading elements but also a logo, a
    /// search form, an author name, and other elements.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/header
    header {
    }
);

dom_type!(header <dom::HtmlElement>);
text_parent!(header);

html_element!(
    /// The [HTML `<h1>`–`<h6>` elements][mdn] represent six levels of section headings. `<h1>` is
    /// the highest section level and `<h6>` is the lowest.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h1
    h1 {
    }
);

dom_type!(h1 <dom::HtmlHeadingElement>);
text_parent!(h1);

html_element!(
    /// The [HTML `<h1>`–`<h6>` elements][mdn] represent six levels of section headings. `<h1>` is
    /// the highest section level and `<h6>` is the lowest.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h2
    h2 {
    }
);

dom_type!(h2 <dom::HtmlHeadingElement>);
text_parent!(h2);

html_element!(
    /// The [HTML `<h1>`–`<h6>` elements][mdn] represent six levels of section headings. `<h1>` is
    /// the highest section level and `<h6>` is the lowest.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h3
    h3 {
    }
);

dom_type!(h3 <dom::HtmlHeadingElement>);
text_parent!(h3);

html_element!(
    /// The [HTML `<h1>`–`<h6>` elements][mdn] represent six levels of section headings. `<h1>` is
    /// the highest section level and `<h6>` is the lowest.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h4
    h4 {
    }
);

dom_type!(h4 <dom::HtmlHeadingElement>);
text_parent!(h4);

html_element!(
    /// The [HTML `<h1>`–`<h6>` elements][mdn] represent six levels of section headings. `<h1>` is
    /// the highest section level and `<h6>` is the lowest.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h5
    h5 {
    }
);

dom_type!(h5 <dom::HtmlHeadingElement>);
text_parent!(h5);

html_element!(
    /// The [HTML `<h1>`–`<h6>` elements][mdn] represent six levels of section headings. `<h1>` is
    /// the highest section level and `<h6>` is the lowest.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h6
    h6 {
    }
);

dom_type!(h6 <dom::HtmlHeadingElement>);
text_parent!(h6);

html_element!(
    /// The [HTML `<hgroup>` element][mdn] represents a multi-level heading for a section of a
    /// document. It groups a set of [`<h1>–<h6>`][heading] elements.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/hgroup
    /// [heading]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/Heading_Elements
    hgroup {
    }
);

dom_type!(hgroup <dom::HtmlElement>);
text_parent!(hgroup);

html_element!(
    /// The [HTML `<main>` element][mdn] represents the dominant content of the [`<body>`][body] of
    /// a document. The main content area consists of content that is directly related to or expands
    /// upon the central topic of a document, or the central functionality of an application.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/main
    /// [body]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/body
    main {
    }
);

dom_type!(main <dom::HtmlElement>);
text_parent!(main);

html_element!(
    /// The [HTML `<nav>` element][mdn] represents a section of a page whose purpose is to provide
    /// navigation links, either within the current document or to other documents. Common examples
    /// of navigation sections are menus, tables of contents, and indexes.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/nav
    nav {
    }
);

dom_type!(nav <dom::HtmlElement>);
text_parent!(nav);

html_element!(
    /// The [HTML `<section>` element][mdn] represents a standalone section — which doesn't have a
    /// more specific semantic element to represent it — contained within an HTML document.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/section
    section {
    }
);

dom_type!(section <dom::HtmlElement>);
text_parent!(section);

html_element!(
    /// The [HTML `<embed>` element][mdn] embeds external content at the specified point in the
    /// document. This content is provided by an external application or other source of interactive
    /// content such as a browser plug-in.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/embed
    embed {
        /// The displayed height of the resource, in [CSS pixels]. This must be an absolute value;
        /// percentages are not allowed.
        /// 
        /// [CSS pixels]: https://drafts.csswg.org/css-values/#px
        height: String,

        /// The URL of the resource being embedded.
        src: String,

        /// The [MIME type] to use to select the plug-in to instantiate.
        /// 
        /// [MIME type]: https://developer.mozilla.org/en-US/docs/Glossary/MIME_type
        type_: String,

        /// The displayed width of the resource, in [CSS pixels]. This must be an absolute value;
        /// percentages are not allowed.
        /// 
        /// [CSS pixels]: https://drafts.csswg.org/css-values/#px
        width: String,

    }
);

dom_type!(embed <dom::HtmlEmbedElement>);

html_element!(
    /// The [HTML Inline Frame element (`<iframe>`)][mdn] represents a nested [browsing context],
    /// embedding another HTML page into the current one.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/iframe
    /// [browsing context]: https://developer.mozilla.org/en-US/docs/Glossary/browsing_context
    iframe {
        /// Specifies a feature policy for the `<iframe>`.
        allow: String,

        /// The height of the frame in CSS pixels. Default is 150.
        height: String,

        /// A targetable name for the embedded browsing context. This can be used in the target
        /// attribute of the `<a>`, `<form>`, or `<base>` elements; the formtarget attribute of the
        /// `<input>` or `<button>` elements; or the windowName parameter in the window.open() method.
        name: String,

        /// Indicates which referrer to send when fetching the frame's resource.
        referrerpolicy: String,

        /// Applies extra restrictions to the content in the frame. The value of the attribute can
        /// either be empty to apply all restrictions, or space-separated tokens to lift particular
        /// restrictions:
        /// 
        /// * allow-downloads-without-user-activation: Allows for downloads to occur without a
        /// gesture from the user.
        /// * allow-forms: Allows the resource to submit forms. If this keyword is not used, form
        /// submission is blocked.
        /// * allow-modals: Lets the resource open modal windows.
        /// * allow-orientation-lock: Lets the resource lock the screen orientation.
        /// * allow-pointer-lock: Lets the resource use the Pointer Lock API.
        /// * allow-popups: Allows popups (such as window.open(), target="_blank", or
        /// showModalDialog()). If this keyword is not used, the popup will silently fail to open.
        /// * allow-popups-to-escape-sandbox: Lets the sandboxed document open new windows without
        /// those windows inheriting the sandboxing. For example, this can safely sandbox an
        /// advertisement without forcing the same restrictions upon the page the ad links to.
        /// * allow-presentation: Lets the resource start a presentation session.
        /// * allow-same-origin: If this token is not used, the resource is treated as being from a
        /// special origin that always fails the same-origin policy.
        /// * allow-scripts: Lets the resource run scripts (but not create popup windows).
        /// * allow-storage-access-by-user-activation : Lets the resource request access to the
        /// parent's storage capabilities with the Storage Access API.
        /// * allow-top-navigation: Lets the resource navigate the top-level browsing context (the
        /// one named _top).
        /// * allow-top-navigation-by-user-activation: Lets the resource navigate the top-level
        /// browsing context, but only if initiated by a user gesture.
        /// 
        /// Notes about sandboxing:
        /// 
        /// When the embedded document has the same origin as the embedding page, it is strongly
        /// discouraged to use both allow-scripts and allow-same-origin, as that lets the embedded
        /// document remove the sandbox attribute — making it no more secure than not using the
        /// sandbox attribute at all.
        /// 
        /// Sandboxing is useless if the attacker can display content outside a sandboxed iframe —
        /// such as if the viewer opens the frame in a new tab. Such content should be also served
        /// from a separate origin to limit potential damage.
        sandbox: String,

        /// The URL of the page to embed. Use a value of about:blank to embed an empty page that
        /// conforms to the same-origin policy. Also note that programatically removing an
        /// `<iframe>`'s src attribute (e.g. via Element.removeAttribute()) causes about:blank to be
        /// loaded in the frame in Firefox (from version 65), Chromium-based browsers, and
        /// Safari/iOS.
        src: String,

        /// Inline HTML to embed, overriding the src attribute. If a browser does not support the
        /// srcdoc attribute, it will fall back to the URL in the src attribute.
        srcdoc: String,

        /// The width of the frame in CSS pixels. Default is 300.
        width: String,

    }
);

dom_type!(iframe <dom::HtmlIFrameElement>);
text_parent!(iframe);

html_element!(
    /// The [HTML `<object>` element][mdn] represents an external resource, which can be treated as
    /// an image, a nested browsing context, or a resource to be handled by a plugin.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/object
    object {
        /// Specifies the URL of the resource.
        data: String,

        /// The form element, if any, that the object element is associated with (its form owner).
        /// The value of the attribute must be an ID of a `<form>` element in the same document.
        form: String,

        /// The height of the displayed resource, in CSS pixels. No percentages.
        height: String,

        /// The name of valid browsing context.
        name: String,

        /// The content type of the resource specified by data. At least one of data and type must
        /// be defined.
        type_: String,

        /// Indicates if the type attribute and the actual content type of the resource must match
        /// to be used.
        typemustmatch: bool,

        /// A hash-name reference to a `<map>` element; that is a '#' followed by the value of a name
        /// of a map element.
        usemap: String,

        /// The width of the display resource, in CSS pixels. No percentages.
        width: String,

    }
);

dom_type!(object <dom::HtmlObjectElement>);
text_parent!(object);

html_element!(
    /// The [HTML `<param>` element][param] defines parameters for an [`<object>`][object] element.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/param
    /// [object]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/object
    param {
        /// Name of the parameter.
        name: String,

        /// Specifies the value of the parameter.
        value: String,

    }
);

dom_type!(param <dom::HtmlParamElement>);

html_element!(
    /// The [HTML `<picture>` element][mdn] contains zero or more [`<source>`][source] elements and
    /// one [`<img>`][img] element to provide versions of an image for different display/device
    /// scenarios.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/picture
    /// [source]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/source
    /// [img]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/img
    picture {
    }
);

dom_type!(picture <dom::HtmlPictureElement>);
text_parent!(picture);

html_element!(
    /// The [HTML `<source>` element][source] specifies multiple media resources for the
    /// [`<picture>`][picture], the [`<audio>`][audio] element, or the [`<video>`][video] element.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/source
    /// [picture]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/picture
    /// [audio]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/audio
    /// [video]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/video
    source {
        /// Media query of the resource's intended media; this should be used only in a `<picture>`
        /// element.
        media: String,

        /// Is a list of source sizes that describes the final rendered width of the image
        /// represented by the source. Each source size consists of a comma-separated list of media
        /// condition-length pairs. This information is used by the browser to determine, before
        /// laying the page out, which image defined in srcset to use. Please note that sizes will
        /// have its effect only if width dimension descriptors are provided with srcset instead of
        /// pixel ratio values (200w instead of 2x for example).
        /// 
        /// The sizes attribute has an effect only when the `<source>` element is the direct child of
        /// a `<picture>` element.
        sizes: String,

        /// Required for `<audio>` and `<video>`, address of the media resource. The value of this
        /// attribute is ignored when the `<source>` element is placed inside a `<picture>` element.
        src: String,

        /// A list of one or more strings separated by commas indicating a set of possible images
        /// represented by the source for the browser to use. Each string is composed of:
        /// 
        /// 1. One URL specifying an image.
        /// 2. A width descriptor, which consists of a string containing a positive integer directly
        /// followed by "w", such as 300w. The default value, if missing, is the infinity.
        /// 3. A pixel density descriptor, that is a positive floating number directly followed by
        /// "x". The default value, if missing, is 1x.
        /// 
        /// Each string in the list must have at least a width descriptor or a pixel density
        /// descriptor to be valid. Among the list, there must be only one string containing the
        /// same tuple of width descriptor and pixel density descriptor. The browser chooses the
        /// most adequate image to display at a given point of time.
        /// 
        /// The srcset attribute has an effect only when the `<source>` element is the direct child of
        /// a `<picture>` element.
        srcset: String,

        /// The MIME media type of the resource, optionally with a codecs parameter.
        type_: String,

    }
);

dom_type!(source <dom::HtmlSourceElement>);

html_element!(
    /// Use the [HTML `<canvas>` element][mdn] with either the [canvas scripting API][api] or the
    /// [WebGL API][gl] to draw graphics and animations.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/canvas
    /// [api]: https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API
    /// [gl]: https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API
    canvas {
        /// The height of the coordinate space in CSS pixels. Defaults to 150.
        height: String,

        /// The width of the coordinate space in CSS pixels. Defaults to 300.
        width: String,

    }
);

dom_type!(canvas <dom::HtmlCanvasElement>);
text_parent!(canvas);

html_element!(
    /// The [HTML `<noscript>` element][mdn] defines a section of HTML to be inserted if a script
    /// type on the page is unsupported or if scripting is currently turned off in the browser.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/noscript
    noscript {
    }
);

dom_type!(noscript <dom::HtmlElement>);
text_parent!(noscript);

html_element!(
    /// The [HTML `<script>` element][mdn] is used to embed or reference executable code; this is
    /// typically used to embed or refer to JavaScript code.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/script
    script {
        /// For classic scripts, if the async attribute is present, then the classic script will be
        /// fetched in parallel to parsing and evaluated as soon as it is available.
        /// 
        /// For module scripts, if the async attribute is present then the scripts and all their
        /// dependencies will be executed in the defer queue, therefore they will get fetched in
        /// parallel to parsing and evaluated as soon as they are available.
        /// 
        /// This attribute allows the elimination of parser-blocking JavaScript where the browser
        /// would have to load and evaluate scripts before continuing to parse. defer has a similar
        /// effect in this case.
        async_: bool,

        /// Normal script elements pass minimal information to the window.onerror for scripts which
        /// do not pass the standard CORS checks. To allow error logging for sites which use a
        /// separate domain for static media, use this attribute.
        crossorigin: String,

        /// Indicates to a browser that the script is meant to be executed after the document has
        /// been parsed, but before firing DOMContentLoaded.
        /// 
        /// Scripts with the defer attribute will prevent the DOMContentLoaded event from firing
        /// until the script has loaded and finished evaluating.
        /// 
        /// This attribute must not be used if the src attribute is absent (i.e. for inline
        /// scripts), in this case it would have no effect.
        /// 
        /// The defer attribute has no effect on module scripts — they defer by default.
        /// 
        /// Scripts with the defer attribute will execute in the order in which they appear in the
        /// document.
        /// 
        /// This attribute allows the elimination of parser-blocking JavaScript where the browser
        /// would have to load and evaluate scripts before continuing to parse. async has a similar
        /// effect in this case.
        defer: bool,

        /// This attribute contains inline metadata that a user agent can use to verify that a
        /// fetched resource has been delivered free of unexpected manipulation.
        integrity: String,

        /// Indicates that the script should not be executed in browsers that support ES2015 modules
        /// — in effect, this can be used to serve fallback scripts to older browsers that do not
        /// support modular JavaScript code.
        nomodule: bool,

        /// A cryptographic nonce (number used once) to whitelist scripts in a script-src
        /// Content-Security-Policy. The server must generate a unique nonce value each time it
        /// transmits a policy. It is critical to provide a nonce that cannot be guessed as
        /// bypassing a resource's policy is otherwise trivial.
        nonce: String,

        /// Indicates which referrer to send when fetching the script, or resources fetched by the
        /// script.
        referrerpolicy: String,

        /// This attribute specifies the URI of an external script; this can be used as an
        /// alternative to embedding a script directly within a document.
        src: String,

        /// This attribute indicates the type of script represented. The value of this attribute
        /// will be in one of the following categories:
        /// 
        /// * Omitted or a JavaScript MIME type: This indicates the script is JavaScript. The HTML5
        /// specification urges authors to omit the attribute rather than provide a redundant MIME
        /// type.
        /// * `module`: Causes the code to be treated as a JavaScript module. The processing of the
        /// script contents is not affected by the charset and defer attributes. Unlike classic
        /// scripts, module scripts require the use of the CORS protocol for cross-origin
        /// fetching.
        /// * Any other value: The embedded content is treated as a data block which won't be
        /// processed by the browser. Developers must use a valid MIME type that is not a
        /// JavaScript MIME type to denote data blocks. The src attribute will be ignored.
        type_: String,

    }
);

dom_type!(script <dom::HtmlScriptElement>);
text_parent!(script);

html_element!(
    /// The [HTML `<area>` element][mdn] defines a hot-spot region on an image, and optionally
    /// associates it with a [hypertext link]. This element is used only within a [`<map>`][map]
    /// element.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/area
    /// [hypertext link]: https://developer.mozilla.org/en-US/docs/Glossary/Hyperlink
    /// [map]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/map
    area {
        /// A text string alternative to display on browsers that do not display images. The text
        /// should be phrased so that it presents the user with the same kind of choice as the image
        /// would offer when displayed without the alternative text. This attribute is required only
        /// if the href attribute is used.
        alt: String,

        /// A set of values specifying the coordinates of the hot-spot region. The number and
        /// meaning of the values depend upon the value specified for the shape attribute.
        /// 
        /// * rect or rectangle: the coords value is two x,y pairs: left, top, right, bottom.
        /// * circle: the value is x,y,r where x,y is a pair specifying the center of the circle and
        /// r is a value for the radius.
        /// * poly or polygon: the value is a set of x,y pairs for each point in the polygon:
        /// x1,y1,x2,y2,x3,y3, and so on.
        /// 
        /// The values are numbers of CSS pixels.
        coords: String,

        /// This attribute, if present, indicates that the author intends the hyperlink to be used
        /// for downloading a resource. See `<a>` for a full description of the download attribute.
        download: bool,

        /// The hyperlink target for the area. Its value is a valid URL. This attribute may be
        /// omitted; if so, the area element does not represent a hyperlink.
        href: String,

        /// Indicates the language of the linked resource. Allowed values are determined by BCP47.
        /// Use this attribute only if the href attribute is present.
        hreflang: String,

        /// Contains a space-separated list of URLs to which, when the hyperlink is followed, POST
        /// requests with the body PING will be sent by the browser (in the background). Typically
        /// used for tracking.
        ping: String,

        /// For anchors containing the href attribute, this attribute specifies the relationship of
        /// the target object to the link object. The value is a space-separated list of link types
        /// values. The values and their semantics will be registered by some authority that might
        /// have meaning to the document author. The default relationship, if no other is given, is
        /// void. Use this attribute only if the href attribute is present.
        rel: String,

        /// This attribute specifies where to display the linked resource. It is a name of, or
        /// keyword for, a browsing context (for example, tab, window, or inline frame). The
        /// following keywords have special meanings:
        /// 
        /// * _self: Load the response into the same browsing context as the current one. This value
        /// is the default if the attribute is not specified.
        /// * _blank: Load the response into a new unnamed browsing context.
        /// * _parent: Load the response into the parent browsing context of the current one. If
        /// there is no parent, this option behaves the same way as _self.
        /// * _top: Load the response into the top-level browsing context (that is, the browsing
        /// context that is an ancestor of the current one, and has no parent). If there is no
        /// parent, this option behaves the same way as _self.
        /// 
        /// Use this attribute only if the `href` attribute is present.
        target: String,

    }
);

dom_type!(area <dom::HtmlAreaElement>);

html_element!(
    /// The [HTML `<audio>` element][mdn] is used to embed sound content in documents. It may
    /// contain one or more audio sources, represented using the `src` attribute or the
    /// [`<source>`][source] element: the browser will choose the most suitable one. It can also be
    /// the destination for streamed media, using a [`MediaStream`][stream].
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/audio
    /// [source]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/source
    /// [stream]: https://developer.mozilla.org/en-US/docs/Web/API/MediaStream
    audio {
        /// If specified, the audio will automatically begin playback as soon as it can do so,
        /// without waiting for the entire audio file to finish downloading.
        /// 
        /// Note: Sites that automatically play audio (or videos with an audio track) can be an
        /// unpleasant experience for users, so should be avoided when possible. If you must offer
        /// autoplay functionality, you should make it opt-in (requiring a user to specifically
        /// enable it). However, this can be useful when creating media elements whose source will
        /// be set at a later time, under user control. See our autoplay guide for additional
        /// information about how to properly use autoplay.
        autoplay: bool,

        /// If this attribute is present, the browser will offer controls to allow the user to
        /// control audio playback, including volume, seeking, and pause/resume playback.
        controls: bool,

        /// This enumerated attribute indicates whether to use CORS to fetch the related audio file.
        /// CORS-enabled resources can be reused in the `<canvas>` element without being tainted.
        /// 
        /// When not present, the resource is fetched without a CORS request (i.e. without sending
        /// the Origin: HTTP header), preventing its non-tainted used in `<canvas>` elements. If
        /// invalid, it is handled as if the enumerated keyword anonymous was used.
        /// 
        /// The allowed values are:
        /// 
        /// # `anonymous`
        /// 
        /// Sends a cross-origin request without a credential. In other words, it sends the
        /// `Origin: HTTP` header without a cookie, X.509 certificate, or performing HTTP Basic
        /// authentication. If the server does not give credentials to the origin site (by not
        /// setting the `Access-Control-Allow-Origin: HTTP` header), the image will be tainted, and
        /// its usage restricted.
        /// 
        /// # `use-credentials`
        /// 
        /// Sends a cross-origin request with a credential. In other words, it sends the
        /// `Origin: HTTP` header with a cookie, a certificate, or performing HTTP Basic
        /// authentication. If the server does not give credentials to the origin site (through
        /// `Access-Control-Allow-Credentials: HTTP` header), the image will be tainted and its
        /// usage restricted.
        crossorigin: String,

        /// Reading currentTime returns a double-precision floating-point value indicating the
        /// current playback position, in seconds, of the audio. If the audio's metadata isn't
        /// available yet—thereby preventing you from knowing the media's start time or
        /// duration—currentTime instead indicates, and can be used to change, the time at which
        /// playback will begin. Otherwise, setting currentTime sets the current playback position
        /// to the given time and seeks the media to that position if the media is currently loaded.
        /// 
        /// If the audio is being streamed, it's possible that the user agent may not be able to
        /// obtain some parts of the resource if that data has expired from the media buffer. Other
        /// audio may have a media timeline that doesn't start at 0 seconds, so setting currentTime
        /// to a time before that would fail. For example, if the audio's media timeline starts at
        /// 12 hours, setting currentTime to 3600 would be an attempt to set the current playback
        /// position well before the beginning of the media, and would fail. The getStartDate()
        /// method can be used to determine the beginning point of the media timeline's reference
        /// frame.
        current_time: String,

        /// If specified, the audio player will automatically seek back to the start upon reaching
        /// the end of the audio.
        loop_: bool,

        /// Indicates whether the audio will be initially silenced. Its default value is false.
        muted: bool,

        /// This enumerated attribute is intended to provide a hint to the browser about what the
        /// author thinks will lead to the best user experience. It may have one of the following
        /// values:
        /// 
        /// * `none`: Indicates that the audio should not be preloaded.
        /// * `metadata`: Indicates that only audio metadata (e.g. length) is fetched.
        /// * `auto`: Indicates that the whole audio file can be downloaded, even if the user is not
        /// expected to use it.
        /// * empty string: A synonym of the auto value.
        /// 
        /// The default value is different for each browser. The spec advises it to be set to
        /// metadata.
        /// 
        /// Usage notes:
        /// 
        /// The autoplay attribute has precedence over preload. If autoplay is specified, the
        /// browser would obviously need to start downloading the audio for playback.
        /// 
        /// The browser is not forced by the specification to follow the value of this attribute; it
        /// is a mere hint.
        preload: String,

        /// The URL of the audio to embed. This is subject to HTTP access controls. This is
        /// optional; you may instead use the `<source>` element within the audio block to specify
        /// the audio to embed.
        src: String,

    }
);

dom_type!(audio <dom::HtmlAudioElement>);
text_parent!(audio);

html_element!(
    /// The [HTML `<img>` element][mdn] embeds an image into the document.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/img
    img {
        /// Defines an alternative text description of the image.
        /// 
        /// > Note: Browsers do not always display images. For example:
        /// >
        /// > * Non-visual browsers (such as those used by people with visual impairments)
        /// > * The user chooses not to display images (saving bandwidth, privacy reasons)
        /// > * The image is invalid or an unsupported type
        /// > * In these cases, the browser may replace the image with the text in the element's alt
        /// attribute. For these reasons and others, provide a useful value for alt whenever
        /// possible.
        /// 
        /// Omitting alt altogether indicates that the image is a key part of the content and no
        /// textual equivalent is available. Setting this attribute to an empty string (alt="")
        /// indicates that this image is not a key part of the content (it’s decoration or a
        /// tracking pixel), and that non-visual browsers may omit it from rendering. Visual
        /// browsers will also hide the broken image icon if the alt is empty and the image failed
        /// to display.
        /// 
        /// This attribute is also used when copying and pasting the image to text, or saving a
        /// linked image to a bookmark.
        alt: String,

        /// Indicates if the fetching of the image must be done using a CORS request. Image data
        /// from a CORS-enabled image returned from a CORS request can be reused in the `<canvas>`
        /// element without being marked "tainted".
        /// 
        /// If the crossorigin attribute is not specified, then a non-CORS request is sent (without
        /// the Origin request header), and the browser marks the image as tainted and restricts
        /// access to its image data, preventing its usage in `<canvas>` elements.
        /// 
        /// If the crossorigin attribute is specified, then a CORS request is sent (with the Origin
        /// request header); but if the server does not opt into allowing cross-origin access to the
        /// image data by the origin site (by not sending any Access-Control-Allow-Origin response
        /// header, or by not including the site's origin in any Access-Control-Allow-Origin
        /// response header it does send), then the browser marks the image as tainted and restricts
        /// access to its image data, preventing its usage in `<canvas>` elements.
        /// 
        /// Allowed values:
        /// 
        /// * `anonymous`: A CORS request is sent with credentials omitted (that is, no cookies,
        /// X.509 certificates, or Authorization request header).
        /// * `use-credentials`: The CORS request is sent with any credentials included (that is,
        /// cookies, X.509 certificates, and the `Authorization` request header). If the server
        /// does not opt into sharing credentials with the origin site (by sending back the
        /// `Access-Control-Allow-Credentials: true` response header), then the browser marks the
        /// image as tainted and restricts access to its image data.
        /// 
        /// If the attribute has an invalid value, browsers handle it as if the anonymous value was
        /// used.
        crossorigin: String,

        /// Provides an image decoding hint to the browser. Allowed values:
        /// 
        /// * `sync`: Decode the image synchronously, for atomic presentation with other content.
        /// * `async`: Decode the image asynchronously, to reduce delay in presenting other content.
        /// * `auto`: Default: no preference for the decoding mode. The browser decides what is best
        /// for the user.
        decoding: String,

        /// The intrinsic height of the image, in pixels. Must be an integer without a unit.
        height: String,

        /// Indicates that the image is part of a server-side map. If so, the coordinates where the
        /// user clicked on the image are sent to the server.
        /// 
        /// Note: This attribute is allowed only if the `<img>` element is a descendant of an `<a>`
        /// element with a valid href attribute. This gives users without pointing devices a
        /// fallback destination.
        ismap: bool,

        /// Indicates how the browser should load the image:
        /// 
        /// * `eager`: Loads the image immediately, regardless of whether or not the image is
        /// currently within the visible viewport (this is the default value).
        /// * `lazy`: Defers loading the image until it reaches a calculated distance from the
        /// viewport, as defined by the browser. The intent is to avoid the network and storage
        /// bandwidth needed to handle the image until it's reasonably certain that it will be
        /// needed. This generally improves the performance of the content in most typical use
        /// cases.
        /// 
        /// > Note: Loading is only deferred when JavaScript is enabled. This is an anti-tracking
        /// measure, because if a user agent supported lazy loading when scripting is disabled, it
        /// would still be possible for a site to track a user's approximate scroll position
        /// throughout a session, by strategically placing images in a page's markup such that a
        /// server can track how many images are requested and when.
        loading: String,

        /// One or more strings separated by commas, indicating a set of source sizes. Each source
        /// size consists of:
        /// 
        /// * A media condition. This must be omitted for the last item in the list.
        /// * A source size value.
        /// 
        /// Media Conditions describe properties of the viewport, not of the image. For example,
        /// (max-height: 500px) 1000px proposes to use a source of 1000px width, if the viewport is
        /// not higher than 500px.
        /// 
        /// Source size values specify the intended display size of the image. User agents use the
        /// current source size to select one of the sources supplied by the srcset attribute, when
        /// those sources are described using width (w) descriptors. The selected source size
        /// affects the intrinsic size of the image (the image’s display size if no CSS styling is
        /// applied). If the srcset attribute is absent, or contains no values with a width
        /// descriptor, then the sizes attribute has no effect.
        sizes: String,

        /// The image URL. Mandatory for the `<img>` element. On browsers supporting srcset, src is
        /// treated like a candidate image with a pixel density descriptor 1x, unless an image with
        /// this pixel density descriptor is already defined in srcset, or unless srcset contains w
        /// descriptors.
        src: String,

        /// One or more strings separated by commas, indicating possible image sources for the user
        /// agent to use. Each string is composed of:
        /// 
        /// * A URL to an image
        /// * Optionally, whitespace followed by one of:
        /// * A width descriptor (a positive integer directly followed by w). The width descriptor
        /// is divided by the source size given in the sizes attribute to calculate the
        /// effective pixel density.
        /// * A pixel density descriptor (a positive floating point number directly followed by
        /// x).
        /// * If no descriptor is specified, the source is assigned the default descriptor of 1x.
        /// 
        /// It is incorrect to mix width descriptors and pixel density descriptors in the same
        /// srcset attribute. Duplicate descriptors (for instance, two sources in the same srcset
        /// which are both described with 2x) are also invalid.
        /// 
        /// The user agent selects any of the available sources at its discretion. This provides
        /// them with significant leeway to tailor their selection based on things like user
        /// preferences or bandwidth conditions. See our Responsive images tutorial for an example.
        srcset: String,

        /// The intrinsic width of the image in pixels. Must be an integer without a unit.
        width: String,

        /// The partial URL (starting with #) of an image map associated with the element.
        /// 
        /// Note: You cannot use this attribute if the `<img>` element is inside an `<a>` or
        /// `<button>` element.
        usemap: String,

    }
);

dom_type!(img <dom::HtmlImageElement>);

html_element!(
    /// The [HTML `<map>` element][mdn] is used with [`<area>`][area] elements to define an image
    /// map (a clickable link area).
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/map
    /// [area]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/area
    map {
        /// The name attribute gives the map a name so that it can be referenced. The attribute must
        /// be present and must have a non-empty value with no space characters. The value of the
        /// name attribute must not be a compatibility-caseless match for the value of the name
        /// attribute of another `<map>` element in the same document. If the id attribute is also
        /// specified, both attributes must have the same value.
        name: String,

    }
);

dom_type!(map <dom::HtmlMapElement>);
text_parent!(map);

html_element!(
    /// The [HTML `<track>` element][mdn] is used as a child of the media elements
    /// [`<audio>`][audio] and [`<video>`][video]. It lets you specify timed text tracks (or
    /// time-based data), for example to automatically handle subtitles. The tracks are formatted in
    /// [WebVTT format][vtt] (`.vtt` files) — Web Video Text Tracks or [Timed Text Markup Language
    /// (TTML)][ttml].
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/track
    /// [audio]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/audio
    /// [video]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/video
    /// [vtt]: https://developer.mozilla.org/en-US/docs/Web/API/Web_Video_Text_Tracks_Format
    /// [ttml]: https://w3c.github.io/ttml2/index.html
    track {
        /// This attribute indicates that the track should be enabled unless the user's preferences
        /// indicate that another track is more appropriate. This may only be used on one track
        /// element per media element.
        default: bool,

        /// How the text track is meant to be used. If omitted the default kind is subtitles. If the
        /// attribute is not present, it will use the subtitles. If the attribute contains an
        /// invalid value, it will use metadata. The following keywords are allowed:
        /// Subtitles provide translation of content that cannot be understood by the viewer. For
        /// example dialogue or text that is not English in an English language film.
        /// 
        /// Subtitles may contain additional content, usually extra background information. For
        /// example the text at the beginning of the Star Wars films, or the date, time, and
        /// location of a scene.
        subtitles: String,

        /// Closed captions provide a transcription and possibly a translation of audio.
        /// 
        /// It may include important non-verbal information such as music cues or sound effects. It
        /// may indicate the cue's source (e.g. music, text, character).
        /// 
        /// Suitable for users who are deaf or when the sound is muted.
        captions: String,

        /// Textual description of the video content.
        /// 
        /// * `descriptions`: Suitable for users who are blind or where the video cannot be seen.
        /// * `chapters`: Chapter titles are intended to be used when the user is navigating the
        /// media resource.
        /// * `metadata`: Tracks used by scripts. Not visible to the user.
        /// * `label`: A user-readable title of the text track which is used by the browser when
        /// listing available text tracks.
        kind: String,

        /// Address of the track (.vtt file). Must be a valid URL. This attribute must be specified
        /// and its URL value must have the same origin as the document — unless the `<audio>` or
        /// `<video>` parent element of the track element has a crossorigin attribute.
        src: String,

        /// Language of the track text data. It must be a valid BCP 47 language tag. If the kind
        /// attribute is set to subtitles, then srclang must be defined.
        srclang: String,

    }
);

dom_type!(track <dom::HtmlTrackElement>);

html_element!(
    /// The [HTML Video element (`<video>`)][mdn] embeds a media player which supports video
    /// playback into the document.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/video
    video {
        /// If specified, the video automatically begins to play back as soon as it can do so
        /// without stopping to finish loading the data.
        /// 
        /// Note: Sites that automatically play audio (or videos with an audio track) can be an
        /// unpleasant experience for users, so should be avoided when possible. If you must offer
        /// autoplay functionality, you should make it opt-in (requiring a user to specifically
        /// enable it). However, this can be useful when creating media elements whose source will
        /// be set at a later time, under user control. See our autoplay guide for additional
        /// information about how to properly use autoplay.
        /// 
        /// To disable video autoplay, autoplay="false" will not work; the video will autoplay if
        /// the attribute is there in the `<video>` tag at all. To remove autoplay, the attribute
        /// needs to be removed altogether.
        autoplay: bool,

        /// An attribute you can read to determine the time ranges of the buffered media. This
        /// attribute contains a TimeRanges object.
        buffered: String,

        /// If this attribute is present, the browser will offer controls to allow the user to
        /// control video playback, including volume, seeking, and pause/resume playback.
        controls: bool,

        /// This enumerated attribute indicates whether to use CORS to fetch the related image.
        /// CORS-enabled resources can be reused in the `<canvas>` element without being tainted.
        /// The allowed values are:
        /// 
        /// * `anonymous`: Sends a cross-origin request without a credential. In other words, it
        /// sends the `Origin: HTTP` header without a cookie, X.509 certificate, or performing
        /// HTTP Basic authentication. If the server does not give credentials to the origin site
        /// (by not setting the `Access-Control-Allow-Origin: HTTP` header), the image will be
        /// tainted, and its usage restricted.
        /// * `use-credentials`: Sends a cross-origin request with a credential. In other words, it
        /// sends the Origin: HTTP header with a cookie, a certificate, or performing HTTP Basic
        /// authentication. If the server does not give credentials to the origin site (through
        /// `Access-Control-Allow-Credentials: HTTP` header), the image will be tainted and its
        /// usage restricted.
        /// 
        /// When not present, the resource is fetched without a CORS request (i.e. without sending
        /// the `Origin: HTTP` header), preventing its non-tainted used in `<canvas>` elements. If
        /// invalid, it is handled as if the enumerated keyword anonymous was used.
        crossorigin: String,

        /// Reading currentTime returns a double-precision floating-point value indicating the
        /// current playback position of the media specified in seconds. If the media has not
        /// started playing yet, the time offset at which it will begin is returned. Setting
        /// currentTime sets the current playback position to the given time and seeks the media to
        /// that position if the media is currently loaded.
        /// 
        /// If the media is being streamed, it's possible that the user agent may not be able to
        /// obtain some parts of the resource if that data has expired from the media buffer. Other
        /// media may have a media timeline that doesn't start at 0 seconds, so setting currentTime
        /// to a time before that would fail. The getStartDate() method can be used to determine the
        /// beginning point of the media timeline's reference frame.
        current_time: String,

        /// The height of the video's display area, in CSS pixels (absolute values only; no
        /// percentages.)
        height: String,

        /// If specified, the browser will automatically seek back to the start upon reaching the
        /// end of the video.
        loop_: bool,

        /// Indicates the default setting of the audio contained in the video. If set, the audio
        /// will be initially silenced. Its default value is false, meaning that the audio will be
        /// played when the video is played.
        muted: bool,

        /// Indicating that the video is to be played "inline", that is within the element's
        /// playback area. Note that the absence of this attribute does not imply that the video
        /// will always be played in fullscreen.
        playsinline: bool,

        /// A URL for an image to be shown while the video is downloading. If this attribute isn't
        /// specified, nothing is displayed until the first frame is available, then the first frame
        /// is shown as the poster frame.
        poster: String,

        /// This enumerated attribute is intended to provide a hint to the browser about what the
        /// author thinks will lead to the best user experience with regards to what content is
        /// loaded before the video is played. It may have one of the following values:
        /// 
        /// * `none`: Indicates that the video should not be preloaded.
        /// * `metadata`: Indicates that only video metadata (e.g. length) is fetched.
        /// * `auto`: Indicates that the whole video file can be downloaded, even if the user is not
        /// expected to use it.
        /// * empty string: Synonym of the auto value.
        /// 
        /// The default value is different for each browser. The spec advises it to be set to
        /// metadata.
        /// 
        /// > Note:
        /// >
        /// > The autoplay attribute has precedence over preload. If autoplay is specified, the
        /// browser would obviously need to start downloading the video for playback.
        /// >
        /// > The specification does not force the browser to follow the value of this attribute; it
        /// is a mere hint.
        preload: String,

        /// The URL of the video to embed. This is optional; you may instead use the `<source>`
        /// element within the video block to specify the video to embed.
        src: String,

        /// The width of the video's display area, in CSS pixels (absolute values only; no
        /// percentages).
        width: String,

    }
);

dom_type!(video <dom::HtmlVideoElement>);
text_parent!(video);

html_element!(
    /// The [HTML Details Element (`<details>`)][mdn] creates a disclosure widget in which
    /// information is visible only when the widget is toggled into an "open" state.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/details
    details {
        /// Indicates whether the details will be shown on page load.
        open: bool,

    }
);

dom_type!(details <dom::HtmlDetailsElement>);
text_parent!(details);

html_element!(
    /// The [HTML `<dialog>` element][mdn] represents a dialog box or other interactive component,
    /// such as an inspector or window.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dialog
    dialog {
        /// Indicates that the dialog is active and can be interacted with. When the open attribute
        /// is not set, the dialog shouldn't be shown to the user.
        open: bool,

    }
);

dom_type!(dialog <dom::HtmlDialogElement>);
text_parent!(dialog);

html_element!(
    /// The [HTML `<menu>` element][mdn] represents a group of commands that a user can perform or
    /// activate. This includes both list menus, which might appear across the top of a screen, as
    /// well as context menus, such as those that might appear underneath a button after it has been
    /// clicked.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/menu
    menu {
    }
);

dom_type!(menu <dom::HtmlMenuElement>);
text_parent!(menu);

html_element!(
    /// The [HTML Disclosure Summary element (`<summary>`)][mdn] element specifies a summary,
    /// caption, or legend for a [`<details>`][details] element's disclosure box.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/summary
    /// [details]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/details
    summary {
    }
);

dom_type!(summary <dom::HtmlElement>);
text_parent!(summary);

html_element!(
    /// The [HTML `<blockquote>` element][mdn] (or *HTML Block Quotation Element*) indicates that
    /// the enclosed text is an extended quotation. Usually, this is rendered visually by
    /// indentation. A URL for the source of the quotation may be given using the `cite` attribute,
    /// while a text representation of the source can be given using the [`<cite>`][cite] element.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/blockquote
    /// [cite]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/cite
    blockquote {
        /// A URL that designates a source document or message for the information quoted. This
        /// attribute is intended to point to information explaining the context or the reference
        /// for the quote.
        cite: String,

    }
);

dom_type!(blockquote <dom::HtmlQuoteElement>);
text_parent!(blockquote);

html_element!(
    /// The [HTML `<dd>` element][mdn] provides the description, definition, or value for the
    /// preceding term ([`<dt>`][dt]) in a description list ([`<dl>`][dl]).
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dd
    /// [dt]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dt
    /// [dl]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dl
    dd {
    }
);

dom_type!(dd <dom::HtmlElement>);
text_parent!(dd);

html_element!(
    /// The [HTML Content Division element (`<div>`)][mdn] is the generic container for flow
    /// content. It has no effect on the content or layout until styled using [CSS].
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/div
    /// [CSS]: https://developer.mozilla.org/en-US/docs/Glossary/CSS
    div {
    }
);

dom_type!(div <dom::HtmlDivElement>);
text_parent!(div);

html_element!(
    /// The [HTML `<dl>` element][mdn] represents a description list. The element encloses a list of
    /// groups of terms (specified using the [`<dt>`][dt] element) and descriptions (provided by
    /// [`<dd>`][dd] elements). Common uses for this element are to implement a glossary or to
    /// display metadata (a list of key-value pairs).
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dl
    /// [dt]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dt
    /// [dd]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dd
    dl {
    }
);

dom_type!(dl <dom::HtmlDListElement>);
text_parent!(dl);

html_element!(
    /// The [HTML `<dt>` element][mdn] specifies a term in a description or definition list, and as
    /// such must be used inside a [`<dl>`][dl] element.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dt
    /// [dl]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dl
    dt {
    }
);

dom_type!(dt <dom::HtmlElement>);
text_parent!(dt);

html_element!(
    /// The [HTML `<figcaption>` or Figure Caption element][mdn] represents a caption or legend
    /// describing the rest of the contents of its parent [`<figure>`][figure] element.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/figcaption
    /// [figure]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/figure
    figcaption {
    }
);

dom_type!(figcaption <dom::HtmlElement>);
text_parent!(figcaption);

html_element!(
    /// The [HTML `<figure>` (Figure With Optional Caption) element][mdn] represents self-contained
    /// content, potentially with an optional caption, which is specified using the
    /// ([`<figcaption>`][figcaption]) element.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/figure
    /// [figcaption]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/figcaption
    figure {
    }
);

dom_type!(figure <dom::HtmlElement>);
text_parent!(figure);

html_element!(
    /// The [HTML `<hr>` element][mdn] represents a thematic break between paragraph-level elements:
    /// for example, a change of scene in a story, or a shift of topic within a section.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/hr
    hr {
    }
);

dom_type!(hr <dom::HtmlHrElement>);

html_element!(
    /// The [HTML `<li>` element][mdn] is used to represent an item in a list.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/li
    li {
    }
);

dom_type!(li <dom::HtmlLiElement>);
text_parent!(li);

html_element!(
    /// The [HTML `<ol>` element][mdn] represents an ordered list of items, typically rendered as a
    /// numbered list.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ol
    ol {
        /// Specifies that the list’s items are in reverse order. Items will be numbered from high
        /// to low.
        reversed: bool,

        /// An integer to start counting from for the list items. Always an Arabic numeral (1, 2, 3,
        /// etc.), even when the numbering type is letters or Roman numerals. For example, to start
        /// numbering elements from the letter "d" or the Roman numeral "iv," use start="4".
        start: u32,

        /// Sets the numbering type:
        /// 
        /// * `a` for lowercase letters
        /// * `A` for uppercase letters
        /// * `i` for lowercase Roman numerals
        /// * `I` for uppercase Roman numerals
        /// * `1` for numbers (default)
        /// 
        /// The specified type is used for the entire list unless a different type attribute is used
        /// on an enclosed `<li>` element.
        /// 
        /// > Note: Unless the type of the list number matters (like legal or technical documents
        /// where items are referenced by their number/letter), use the CSS list-style-type property
        /// instead.
        type_: String,

    }
);

dom_type!(ol <dom::HtmlOListElement>);
text_parent!(ol);

html_element!(
    /// The [HTML `<p>` element][mdn] represents a paragraph.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/p
    p {
    }
);

dom_type!(p <dom::HtmlParagraphElement>);
text_parent!(p);

html_element!(
    /// The [HTML `<pre>` element][mdn] represents preformatted text which is to be presented
    /// exactly as written in the HTML file.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/pre
    pre {
    }
);

dom_type!(pre <dom::HtmlPreElement>);
text_parent!(pre);

html_element!(
    /// The [HTML `<ul>` element][mdn] represents an unordered list of items, typically rendered as
    /// a bulleted list.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ul
    ul {
    }
);

dom_type!(ul <dom::HtmlUListElement>);
text_parent!(ul);

html_element!(
    /// The [HTML Table Caption element (`<caption>`)][mdn] specifies the caption (or title) of a
    /// table, and if used is *always* the first child of a [`<table>`][table].
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/caption
    /// [table]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/table
    caption {
    }
);

dom_type!(caption <dom::HtmlTableCaptionElement>);
text_parent!(caption);

html_element!(
    /// The [HTML `<col>` element][mdn] defines a column within a table and is used for defining
    /// common semantics on all common cells. It is generally found within a [`<colgroup>`][cg]
    /// element.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/col
    /// [cg]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/colgroup
    col {
        /// This attribute contains a positive integer indicating the number of consecutive columns
        /// the `<col>` element spans. If not present, its default value is 1.
        span: String,

    }
);

dom_type!(col <dom::HtmlTableColElement>);

html_element!(
    /// The [HTML `<colgroup>` element][mdn] defines a group of columns within a table.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/colgroup
    colgroup {
        /// This attribute contains a positive integer indicating the number of consecutive columns
        /// the `<colgroup>` element spans. If not present, its default value is 1.
        /// 
        /// > Note: This attribute is applied on the attributes of the column group, it has no
        /// > effect on the CSS styling rules associated with it or, even more, to the cells of the
        /// > column's members of the group.
        /// >
        /// > The span attribute is not permitted if there are one or more `<col>` elements within
        /// > the `<colgroup>`.
        span: String,

    }
);

dom_type!(colgroup <dom::HtmlTableColElement>);
text_parent!(colgroup);

html_element!(
    /// The [HTML `<table>` element][mdn] represents tabular data — that is, information presented
    /// in a two-dimensional table comprised of rows and columns of cells containing data.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/table
    table {
    }
);

dom_type!(table <dom::HtmlTableElement>);
text_parent!(table);

html_element!(
    /// The [HTML Table Body element (`<tbody>`)][mdn] encapsulates a set of table rows
    /// ([`<tr>`][tr] elements), indicating that they comprise the body of the table
    /// ([`<table>`][table]).
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/tbody
    /// [tr]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/tr
    /// [table]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/table
    tbody {
    }
);

dom_type!(tbody <dom::HtmlTableSectionElement>);
text_parent!(tbody);

html_element!(
    /// The [HTML `<td>` element][mdn] defines a cell of a table that contains data. It participates
    /// in the *table model*.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/td
    td {
        /// This attribute contains a non-negative integer value that indicates for how many columns
        /// the cell extends. Its default value is 1. Values higher than 1000 will be considered as
        /// incorrect and will be set to the default value (1).
        colspan: String,

        /// This attribute contains a list of space-separated strings, each corresponding to the id
        /// attribute of the `<th>` elements that apply to this element.
        headers: String,

        /// This attribute contains a non-negative integer value that indicates for how many rows
        /// the cell extends. Its default value is 1; if its value is set to 0, it extends until the
        /// end of the table section (`<thead>`, `<tbody>`, `<tfoot>`, even if implicitly defined),
        /// that the cell belongs to. Values higher than 65534 are clipped down to 65534.
        rowspan: String,

    }
);

dom_type!(td <dom::HtmlTableCellElement>);
text_parent!(td);

html_element!(
    /// The [HTML `<tfoot>` element][mdn] defines a set of rows summarizing the columns of the
    /// table.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/tfoot
    tfoot {
    }
);

dom_type!(tfoot <dom::HtmlTableSectionElement>);
text_parent!(tfoot);

html_element!(
    /// The [HTML `<th>` element][mdn] defines a cell as header of a group of table cells. The exact
    /// nature of this group is defined by the [`scope`][scope] and [`headers`][headers] attributes.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/th
    /// [scope]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/th#attr-scope
    /// [headers]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/th#attr-headers
    th {
        /// This attribute contains a short abbreviated description of the cell's content. Some
        /// user-agents, such as speech readers, may present this description before the content
        /// itself.
        abbr: String,

        /// This attribute contains a non-negative integer value that indicates for how many columns
        /// the cell extends. Its default value is 1. Values higher than 1000 will be considered as
        /// incorrect and will be set to the default value (1).
        colspan: String,

        /// This attribute contains a list of space-separated strings, each corresponding to the id
        /// attribute of the `<th>` elements that apply to this element.
        headers: String,

        /// This attribute contains a non-negative integer value that indicates for how many rows
        /// the cell extends. Its default value is 1; if its value is set to 0, it extends until the
        /// end of the table section (`<thead>`, `<tbody>`, `<tfoot>`, even if implicitly defined),
        /// that the cell belongs to. Values higher than 65534 are clipped down to 65534.
        rowspan: String,

        /// This enumerated attribute defines the cells that the header (defined in the `<th>`)
        /// element relates to. It may have the following values:
        /// 
        /// * `row`: The header relates to all cells of the row it belongs to.
        /// * `col`: The header relates to all cells of the column it belongs to.
        /// * `rowgroup`: The header belongs to a rowgroup and relates to all of its cells. These
        /// cells can be placed to the right or the left of the header, depending on the value of
        /// the dir attribute in the `<table>` element.
        /// * `colgroup`: The header belongs to a colgroup and relates to all of its cells.
        /// * `auto`
        /// 
        /// The default value when this attribute is not specified is auto.
        scope: String,

    }
);

dom_type!(th <dom::HtmlTableCellElement>);
text_parent!(th);

html_element!(
    /// The [HTML `<thead>` element][mdn] defines a set of rows defining the head of the columns of
    /// the table.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/thead
    thead {
    }
);

dom_type!(thead <dom::HtmlTableSectionElement>);
text_parent!(thead);

html_element!(
    /// The [HTML `<tr>` element][mdn] defines a row of cells in a table. The row's cells can then
    /// be established using a mix of [`<td>`][td] (data cell) and [`<th>`][th] (header cell)
    /// elements.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/tr
    /// [td]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/td
    /// [th]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/th
    tr {
    }
);

dom_type!(tr <dom::HtmlTableRowElement>);
text_parent!(tr);

html_element!(
    /// The [HTML `<base> element`][mdn] specifies the base URL to use for all relative URLs
    /// contained within a document. There can be only one `<base>` element in a document.
    /// 
    /// If either of its inherent attributes are specified, this element must come before other
    /// elements with attributes whose values are URLs, such as <link>’s href attribute.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/base
    base {
        /// The base URL to be used throughout the document for relative URLs. Absolute and relative
        /// URLs are allowed.
        href: String,

        /// A keyword or author-defined name of the default browsing context to display the result
        /// when links or forms cause navigation, for `<a>` or `<form>` elements without an explicit
        /// target attribute. The attribute value targets a browsing context (such as a tab, window,
        /// or `<iframe>`).
        /// 
        /// The following keywords have special meanings:
        /// 
        /// * `_self`: Load the result into the same browsing context as the current one. (This is
        /// the default.)
        /// * `_blank`: Load the result into a new, unnamed browsing context.
        /// * `_parent`: Load the result into the parent browsing context of the current one. (If
        /// the current page is inside a frame.) If there is no parent, behaves the same way as
        /// _self.
        /// * `_top`: Load the result into the topmost browsing context (that is, the browsing
        /// context that is an ancestor of the current one, and has no parent). If there is no
        /// parent, behaves the same way as _self.
        target: String,

    }
);

dom_type!(base <dom::HtmlBaseElement>);

html_element!(
    /// The [HTML `<head>` element][mdn] contains machine-readable information ([metadata]) about
    /// the document, like its [title], [scripts], and [style sheets].
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/head
    /// [metadata]: https://developer.mozilla.org/en-US/docs/Glossary/metadata
    /// [title]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/title
    /// [scripts]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/script
    /// [style sheets]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/style
    head {
    }
);

dom_type!(head <dom::HtmlHeadElement>);
text_parent!(head);

html_element!(
    /// The [HTML External Resource Link element (`<link>`)][mdn] specifies relationships between
    /// the current document and an external resource. This element is most commonly used to link to
    /// [stylesheets], but is also used to establish site icons (both "favicon" style icons and
    /// icons for the home screen and apps on mobile devices) among other things.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/link
    /// [stylesheets]: https://developer.mozilla.org/en-US/docs/Glossary/CSS
    link {
        /// This attribute is only used when rel="preload" or rel="prefetch" has been set on the
        /// `<link>` element. It specifies the type of content being loaded by the `<link>`, which is
        /// necessary for request matching, application of correct content security policy, and
        /// setting of correct Accept request header. Furthermore, rel="preload" uses this as a
        /// signal for request prioritization. The table below lists the valid values for this
        /// attribute and the elements or resources they apply to.
        /// 
        /// | Value    | Applies To                                                                |
        /// | -------- | ------------------------------------------------------------------------- |
        /// | audio    | `<audio>` elements                                                        |
        /// | document | `<iframe>` and `<frame>` elements                                         |
        /// | embed    | `<embed>` elements                                                        |
        /// | fetch    | fetch, XHR (also requires `<link>` to contain the crossorigin attribute.) |
        /// | font     | CSS @font-face                                                            |
        /// | image    | `<img>` and `<picture>` elements with srcset or imageset attributes,      |
        /// |          | SVG `<image>` elements, CSS *-image rules                                 |
        /// | object   | `<object>` elements                                                       |
        /// | script   | `<script>` elements, Worker importScripts                                 |
        /// | style    | `<link rel=stylesheet>` elements, CSS @import                             |
        /// | track    | `<track>` elements                                                        |
        /// | video    | `<video>` elements                                                        |
        /// | worker   | Worker, SharedWorker                                                      |
        as_: String,

        /// This enumerated attribute indicates whether CORS must be used when fetching the
        /// resource. CORS-enabled images can be reused in the `<canvas>` element without being
        /// tainted. The allowed values are:
        /// 
        /// * `anonymous`: A cross-origin request (i.e. with an Origin HTTP header) is performed,
        /// but no credential is sent (i.e. no cookie, X.509 certificate, or HTTP Basic
        /// authentication). If the server does not give credentials to the origin site (by not
        /// setting the Access-Control-Allow-Origin HTTP header) the resource will be tainted and
        /// its usage restricted.
        /// * `use-credentials`: A cross-origin request (i.e. with an Origin HTTP header) is
        /// performed along with a credential sent (i.e. a cookie, certificate, and/or HTTP Basic
        /// authentication is performed). If the server does not give credentials to the origin
        /// site (through Access-Control-Allow-Credentials HTTP header), the resource will be
        /// tainted and its usage restricted.
        /// 
        /// If the attribute is not present, the resource is fetched without a CORS request (i.e.
        /// without sending the Origin HTTP header), preventing its non-tainted usage. If invalid,
        /// it is handled as if the enumerated keyword anonymous was used.
        crossorigin: String,

        /// For rel="stylesheet" only, the disabled Boolean attribute indicates whether or not the
        /// described stylesheet should be loaded and applied to the document. If disabled is
        /// specified in the HTML when it is loaded, the stylesheet will not be loaded during page
        /// load. Instead, the stylesheet will be loaded on-demand, if and when the disabled
        /// attribute is changed to false or removed.
        /// 
        /// Once the stylesheet has been loaded, however, changes made to the value of the disabled
        /// property no longer have any relationship to the value of the StyleSheet.disabled
        /// property. Changing the value of this property instead simply enables and disables the
        /// stylesheet form being applied to the document.
        /// 
        /// This differs from StyleSheet's disabled property; changing it to true removes the
        /// stylesheet from the document's document.styleSheets list, and doesn't automatically
        /// reload the stylesheet when it's toggled back to false.
        disabled: String,

        /// This attribute specifies the URL of the linked resource. A URL can be absolute or
        /// relative.
        href: String,

        /// This attribute indicates the language of the linked resource. It is purely advisory.
        /// Allowed values are determined by BCP47. Use this attribute only if the href attribute is
        /// present.
        hreflang: String,

        /// This attribute specifies the media that the linked resource applies to. Its value must
        /// be a media type / media query. This attribute is mainly useful when linking to external
        /// stylesheets — it allows the user agent to pick the best adapted one for the device it
        /// runs on.
        media: String,

        /// This attribute names a relationship of the linked document to the current document. The
        /// attribute must be a space-separated list of link type values.
        rel: String,

        /// This attribute defines the sizes of the icons for visual media contained in the
        /// resource. It must be present only if the rel contains a value of icon or a non-standard
        /// type such as Apple's apple-touch-icon. It may have the following values:
        /// 
        /// * `any`, meaning that the icon can be scaled to any size as it is in a vector format,
        /// like image/svg+xml.
        /// * a white-space separated list of sizes, each in the format <width in pixels>x<height in
        /// pixels> or <width in pixels>X<height in pixels>. Each of these sizes must be contained
        /// in the resource.
        /// 
        /// Note: Most icon formats are only able to store one single icon; therefore most of the
        /// time the sizes attribute contains only one entry. MS's ICO format does, as well as
        /// Apple's ICNS. ICO is more ubiquitous, so you should use this format if cross-browser
        /// support is a concern (especially for old IE versions).
        sizes: String,

        /// The title attribute has special semantics on the `<link>` element. When used on a
        /// `<link rel="stylesheet">` it defines a preferred or an alternate stylesheet. Incorrectly
        /// using it may cause the stylesheet to be ignored.
        title: String,

        /// This attribute is used to define the type of the content linked to. The value of the
        /// attribute should be a MIME type such as text/html, text/css, and so on. The common use
        /// of this attribute is to define the type of stylesheet being referenced (such as
        /// text/css), but given that CSS is the only stylesheet language used on the web, not only
        /// is it possible to omit the type attribute, but is actually now recommended practice. It
        /// is also used on rel="preload" link types, to make sure the browser only downloads file
        /// types that it supports.
        type_: String,

    }
);

dom_type!(link <dom::HtmlLinkElement>);

html_element!(
    /// The [HTML `<meta>` element][mdn] represents [metadata] that cannot be represented by other
    /// HTML meta-related elements, like [`<base>`], [`<link>`], [`<script>`], [`<style>`] or
    /// [`<title>`].
    /// 
    /// Note: the attribute `name` has a specific meaning for the `<meta>` element, and the
    /// `itemprop` attribute must not be set on the same `<meta>` element that has any existing
    /// name, `http-equiv` or `charset` attributes.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meta
    /// [metadata]: https://developer.mozilla.org/en-US/docs/Glossary/Metadata
    /// [`<base>`]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/base
    /// [`<link>`]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/link
    /// [`<script>`]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/script
    /// [`<style>`]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/style
    /// [`<title>`]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/title
    meta {
        /// This attribute declares the document's character encoding. If the attribute is present,
        /// its value must be an ASCII case-insensitive match for the string "utf-8".
        charset: String,

        /// This attribute contains the value for the http-equiv or name attribute, depending on
        /// which is used.
        content: String,

        /// Defines a pragma directive. The attribute is named http-equiv(alent) because all the
        /// allowed values are names of particular HTTP headers:
        /// 
        /// * `content-security-policy`: Allows page authors to define a content policy for the
        /// current page. Content policies mostly specify allowed server origins and script
        /// endpoints which help guard against cross-site scripting attacks.
        /// * `content-type`: If specified, the content attribute must have the value
        /// `text/html; charset=utf-8`. Note: Can only be used in documents served with a
        /// text/html MIME type — not in documents served with an XML MIME type.
        /// * `default-style`: Sets the name of the default CSS style sheet set.
        /// * `x-ua-compatible`: If specified, the content attribute must have the value "IE=edge".
        /// User agents are required to ignore this pragma.
        /// * `refresh`: This instruction specifies:
        /// * The number of seconds until the page should be reloaded - only if the content
        /// attribute contains a positive integer.
        /// * The number of seconds until the page should redirect to another - only if the
        /// content attribute contains a positive integer followed by the string ';url=', and a
        /// valid URL.
        /// * Accessibility concerns: Pages set with a refresh value run the risk of having the
        /// time interval being too short. People navigating with the aid of assistive
        /// technology such as a screen reader may be unable to read through and understand the
        /// page's content before being automatically redirected. The abrupt, unannounced
        /// updating of the page content may also be disorienting for people experiencing low
        /// vision conditions.
        http_equiv: String,

        /// The name and content attributes can be used together to provide document metadata in
        /// terms of name-value pairs, with the name attribute giving the metadata name, and the
        /// content attribute giving the value.
        /// 
        /// See [standard metadata names] for details about the set of standard metadata names
        /// defined in the HTML specification.
        /// 
        /// [standard metadata names]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meta/name
        name: String,

    }
);

dom_type!(meta <dom::HtmlMetaElement>);

html_element!(
    /// The [HTML `<style>` element][mdn] contains style information for a document, or part of a
    /// document.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/style
    style {
        /// This attribute defines which media the style should be applied to. Its value is a media
        /// query, which defaults to all if the attribute is missing.
        media: String,

        /// A cryptographic nonce (number used once) used to whitelist inline styles in a style-src
        /// Content-Security-Policy. The server must generate a unique nonce value each time it
        /// transmits a policy. It is critical to provide a nonce that cannot be guessed as
        /// bypassing a resource’s policy is otherwise trivial.
        nonce: String,

        /// This attribute specifies alternative style sheet sets.
        title: String,

    }
);

dom_type!(style <dom::HtmlStyleElement>);
text_parent!(style);

html_element!(
    /// The [HTML Title element (`<title>`)][mdn] defines the document's title that is shown in a
    /// [browser]'s title bar or a page's tab.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/title
    /// [browser]: https://developer.mozilla.org/en-US/docs/Glossary/Browser
    title {
    }
);

dom_type!(title <dom::HtmlTitleElement>);
text_parent!(title);

html_element!(
    /// The [HTML `<button>` element][mdn] represents a clickable button, which can be used in
    /// [forms] or anywhere in a document that needs simple, standard button functionality.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button
    /// [forms]: https://developer.mozilla.org/en-US/docs/Learn/HTML/Forms
    button {
        /// Specifies that the button should have input focus when the page loads. Only one element
        /// in a document can have this attribute.
        autofocus: bool,

        /// Prevents the user from interacting with the button: it cannot be pressed or focused.
        disabled: bool,

        /// The `<form>` element to associate the button with (its form owner). The value of this
        /// attribute must be the id of a `<form>` in the same document. (If this attribute is not
        /// set, the `<button>` is associated with its ancestor `<form>` element, if any.)
        /// 
        /// This attribute lets you associate `<button>` elements to `<form>`s anywhere in the
        /// document, not just inside a `<form>`. It can also override an ancestor `<form>` element.
        form: String,

        /// The URL that processes the information submitted by the button. Overrides the action
        /// attribute of the button's form owner. Does nothing if there is no form owner.
        formaction: String,

        /// If the button is a submit button (it's inside/associated with a `<form>` and doesn't
        /// have type="button"), specifies how to encode the form data that is submitted. Possible
        /// values:
        /// 
        /// * application/x-www-form-urlencoded: The default if the attribute is not used.
        /// * multipart/form-data: Use to submit `<input>` elements with their type attributes set
        /// to file.
        /// * text/plain: Specified as a debugging aid; shouldn’t be used for real form submission.
        /// 
        /// If this attribute is specified, it overrides the enctype attribute of the button's form
        /// owner.
        formenctype: String,

        /// If the button is a submit button (it's inside/associated with a `<form>` and doesn't
        /// have type="button"), this attribute specifies the HTTP method used to submit the form.
        /// Possible values:
        /// 
        /// * post: The data from the form are included in the body of the HTTP request when sent to
        /// the server. Use when the form contains information that shouldn’t be public, like
        /// login credentials.
        /// * get: The form data are appended to the form's action URL, with a ? as a separator, and
        /// the resulting URL is sent to the server. Use this method when the form has no side
        /// effects, like search forms.
        /// 
        /// If specified, this attribute overrides the method attribute of the button's form owner.
        formmethod: String,

        /// If the button is a submit button, specifies that the form is not to be validated when it
        /// is submitted. If this attribute is specified, it overrides the novalidate attribute of
        /// the button's form owner.
        /// 
        /// This attribute is also available on `<input type="image">` and `<input type="submit">`
        /// elements.
        formnovalidate: bool,

        /// If the button is a submit button, this attribute is a author-defined name or
        /// standardized, underscore-prefixed keyword indicating where to display the response from
        /// submitting the form. This is the name of, or keyword for, a browsing context (a tab,
        /// window, or `<iframe>`). If this attribute is specified, it overrides the target
        /// attribute of the button's form owner. The following keywords have special meanings:
        /// 
        /// * _self: Load the response into the same browsing context as the current one.
        /// This is the default if the attribute is not specified.
        /// * _blank: Load the response into a new unnamed browsing context — usually a new tab or
        /// window, depending on the user’s browser settings.
        /// * _parent: Load the response into the parent browsing context of the current one. If
        /// there is no parent, this option behaves the same way as _self.
        /// * _top: Load the response into the top-level browsing context (that is, the browsing
        /// context that is an ancestor of the current one, and has no parent). If there is no
        /// parent, this option behaves the same way as _self.
        formtarget: String,

        /// The name of the button, submitted as a pair with the button’s value as part of the form
        /// data.
        name: String,

        /// The default behavior of the button. Possible values are:
        /// 
        /// * submit: The button submits the form data to the server. This is the default if the
        /// attribute is not specified for buttons associated with a `<form>`, or if the attribute
        /// is an empty or invalid value.
        /// * reset: The button resets all the controls to their initial values, like
        /// `<input type="reset">`. (This behavior tends to annoy users.)
        /// * button: The button has no default behavior, and does nothing when pressed by default.
        /// It can have client-side scripts listen to the element's events, which are triggered
        /// when the events occur.
        type_: String,

        /// Defines the value associated with the button’s name when it’s submitted with the form
        /// data. This value is passed to the server in params when the form is submitted.
        value: String,

    }
);

dom_type!(button <dom::HtmlButtonElement>);
text_parent!(button);

html_element!(
    /// The [HTML `<datalist>` element][mdn] contains a set of [`<option>`][option] elements that
    /// represent the values available for other controls.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/datalist
    /// [option]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/option
    datalist {
    }
);

dom_type!(datalist <dom::HtmlDataListElement>);
text_parent!(datalist);

html_element!(
    /// The [HTML `<fieldset>` element][mdn] is used to group several controls as well as labels
    /// ([`<label>`][label]) within a web form.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/fieldset
    /// [label]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/label
    fieldset {
        /// If this Boolean attribute is set, all form controls that are descendants of the
        /// `<fieldset>` are disabled, meaning they are not editable and won't be submitted along
        /// with the `<form>`. They won't receive any browsing events, like mouse clicks or
        /// focus-related events. By default browsers display such controls grayed out. Note that
        /// form elements inside the `<legend>` element won't be disabled.
        disabled: String,

        /// This attribute takes the value of the id attribute of a `<form>` element you want the
        /// `<fieldset>` to be part of, even if it is not inside the form.
        form: String,

        /// The name associated with the group.
        /// 
        /// Note: The caption for the fieldset is given by the first `<legend>` element inside it.
        name: String,

    }
);

dom_type!(fieldset <dom::HtmlFieldSetElement>);
text_parent!(fieldset);

html_element!(
    /// The [HTML `<form>` element][mdn] represents a document section that contains interactive
    /// controls for submitting information to a web server.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/form
    form {
        /// Space-separated [character encodings] the server accepts. The browser uses
        /// them in the order in which they are listed. The default value means
        /// the same encoding as the page.
        /// 
        /// [character encodings]: https://developer.mozilla.org/en-US/docs/Web/Guide/Localizations_and_character_encodings
        accept_charset: String,

        /// The URI of a program that processes the information submitted via the form.
        action: String,

        /// Indicates whether input elements can by default have their values automatically
        /// completed by the browser. autocomplete attributes on form elements override it on
        /// `<form>`. Possible values:
        /// 
        /// * off: The browser may not automatically complete entries. (Browsers tend to ignore this
        /// for suspected login forms; see The autocomplete attribute and login fields.)
        /// * on: The browser may automatically complete entries.
        autocomplete: String,

        /// If the value of the method attribute is post, enctype is the MIME type of the form
        /// submission. Possible values:
        /// 
        /// * application/x-www-form-urlencoded: The default value.
        /// * multipart/form-data: Use this if the form contains `<input>` elements with type=file.
        /// * text/plain: Introduced by HTML5 for debugging purposes.
        /// 
        /// This value can be overridden by formenctype attributes on `<button>`,
        /// `<input type="submit">`, or `<input type="image">` elements.
        enctype: String,

        /// The HTTP method to submit the form with. Possible values:
        /// 
        /// * post: The POST method; form data sent as the request body.
        /// * get: The GET method; form data appended to the action URL with a ? separator. Use this
        /// method when the form has no side-effects.
        /// * dialog: When the form is inside a `<dialog>`, closes the dialog on submission.
        /// 
        /// This value is overridden by formmethod attributes on `<button>`,
        /// `<input type="submit">`, or `<input type="image">` elements.
        method: String,

        /// Indicates that the form shouldn't be validated when submitted. If this attribute is not
        /// set (and therefore the form is validated), it can be overridden by a formnovalidate
        /// attribute on a `<button>`, `<input type="submit">`, or `<input type="image">` element
        /// belonging to the form.
        novalidate: bool,

        /// Creates a hyperlink or annotation depending on the value.
        rel: String,

        /// Indicates where to display the response after submitting the form. It is a name/keyword
        /// for a browsing context (for example, tab, window, or iframe). The following keywords
        /// have special meanings:
        /// 
        /// * _self (default): Load into the same browsing context as the current one.
        /// * _blank: Load into a new unnamed browsing context.
        /// * _parent: Load into the parent browsing context of the current one. If no parent,
        /// behaves the same as _self.
        /// * _top: Load into the top-level browsing context (i.e., the browsing context that is an
        /// ancestor of the current one and has no parent). If no parent, behaves the same as
        /// _self.
        /// 
        /// This value can be overridden by a formtarget attribute on a `<button>`,
        /// `<input type="submit">`, or `<input type="image">` element.
        target: String,

    }
);

dom_type!(form <dom::HtmlFormElement>);
text_parent!(form);

html_element!(
    /// The [HTML `<input>` element][mdn] is used to create interactive controls for web-based forms
    /// in order to accept data from the user; a wide variety of types of input data and control
    /// widgets are available, depending on the device and [user agent].
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input
    /// [user agent]: https://developer.mozilla.org/en-US/docs/Glossary/user_agent
    input {
        /// Valid for the file input type only, the accept property defines which file types are
        /// selectable in a file upload control. See the file input type.
        accept: String,

        /// Valid for the image button only, the alt attribute provides alternative text for the
        /// image, displaying the value of the attribute if the image src is missing or otherwise
        /// fails to load. See the image input type.
        alt: String,

        /// The autocomplete attribute takes as its value a space-separated string that describes
        /// what, if any, type of autocomplete functionality the input should provide. A typical
        /// implementation of autocomplete simply recalls previous values entered in the same input
        /// field, but more complex forms of autocomplete can exist. For instance, a browser could
        /// integrate with a device's contacts list to autocomplete email addresses in an email
        /// input field. See Values in The HTML autocomplete attribute for permitted values.
        /// 
        /// The autocomplete attribute is valid on hidden, text, search, url, tel, email, date,
        /// month, week, time, datetime-local, number, range, color, and password. This attribute
        /// has no effect on input types that do not return numeric or text data, being valid for
        /// all input types except checkbox, radio, file, or any of the button types.
        /// 
        /// See The HTML autocomplete attribute for additional information, including information on
        /// password security and how autocomplete is slightly different for hidden than for other
        /// input types.
        autocomplete: String,

        /// Indicates if present that the input should automatically have focus when the page has
        /// finished loading (or when the `<dialog>` containing the element has been displayed).
        /// 
        /// Note: An element with the autofocus attribute may gain focus before the DOMContentLoaded
        /// event is fired.
        /// 
        /// No more than one element in the document may have the autofocus attribute. The autofocus
        /// attribute cannot be used on inputs of type hidden, since hidden inputs cannot be
        /// focused.
        /// 
        /// If put on more than one element, the first one with the attribute receives focus.
        /// 
        /// Warning: Automatically focusing a form control can confuse visually-impaired people
        /// using screen-reading technology and people with cognitive impairments. When autofocus is
        /// assigned, screen-readers "teleport" their user to the form control without warning them
        /// beforehand.
        /// 
        /// For better usability, avoid using autofocus. Automatically focusing on a form control
        /// can cause the page to scroll on load. The focus can also cause dynamic keyboards to
        /// display on some touch devices. While a screen reader will announce the label of the form
        /// control receiving focus, the screen reader  will not announce anything before the label,
        /// and the sighted user on a small device will equally miss the context created by the
        /// preceding content.
        autofocus: bool,

        /// Introduced in the HTML Media Capture specification and valid for the file input type
        /// only, the capture attribute defines which media—microphone, video, or camera—should be
        /// used to capture a new file for upload with file upload control in supporting scenarios.
        /// See the file input type.
        capture: String,

        /// Valid for both radio and checkbox types, checked is a Boolean attribute. If present on a
        /// radio type, it indicates that that radio button is the currently selected one in the
        /// group of same-named radio buttons. If present on a checkbox type, it indicates that the
        /// checkbox is checked by default (when the page loads). It does not indicate whether this
        /// checkbox is currently checked: if the checkbox’s state is changed, this content
        /// attribute does not reflect the change. (Only the HTMLInputElement’s checked IDL
        /// attribute is updated.)
        /// 
        /// Note: Unlike other input controls, a checkboxes and radio buttons value are only
        /// included in the submitted data if they are currently checked. If they are, the name and
        /// the value(s) of the checked controls are submitted.
        /// 
        /// For example, if a checkbox whose name is fruit has a value of cherry, and the checkbox
        /// is checked, the form data submitted will include fruit=cherry. If the checkbox isn't
        /// active, it isn't listed in the form data at all. The default value for checkboxes and
        /// radio buttons is on.
        checked: bool,

        /// Valid for text and search input types only, the dirname attribute enables the submission
        /// of the directionality of the element. When included, the form control will submit with
        /// two name/value pairs: the first being the name and value, the second being the value of
        /// the dirname as the name with the value of ltr or rtl being set by the browser.
        dirname: String,

        /// If present indicates that the user should not be able to interact with the input.
        /// Disabled inputs are typically rendered with a dimmer color or using some other form of
        /// indication that the field is not available for use.
        /// 
        /// Specifically, disabled inputs do not receive the click event, and disabled inputs are
        /// not submitted with the form.
        disabled: bool,

        /// A string specifying the `<form>` element with which the input is associated (that is,
        /// its form owner). This string's value, if present, must match the id of a `<form>`
        /// element in the same document. If this attribute isn't specified, the `<input>` element
        /// is associated with the nearest containing form, if any.
        /// 
        /// The form attribute lets you place an input anywhere in the document but have it included
        /// with a form elsewhere in the document.
        /// 
        /// Note: An input can only be associated with one form.
        form: String,

        /// Valid for the image and submit input types only. See the submit input type for more
        /// information.
        formaction: String,

        /// Valid for the image and submit input types only. See the submit input type for more
        /// information.
        formenctype: String,

        /// Valid for the image and submit input types only. See the submit input type for more
        /// information.
        formmethod: String,

        /// Valid for the image and submit input types only. See the submit input type for more
        /// information.
        formnovalidate: String,

        /// Valid for the image and submit input types only. See the submit input type for more
        /// information.
        formtarget: String,

        /// Valid for the image input button only, the height is the height of the image file to
        /// display to represent the graphical submit button. See the image input type.
        height: String,

        /// Global value valid for all elements, it provides a hint to browsers as to the type of
        /// virtual keyboard configuration to use when editing this element or its contents. Values
        /// include none, text, tel, url, email, numeric, decimal, and search.
        inputmode: String,

        /// The values of the list attribute is the id of a `<datalist>` element located in the same
        /// document. The `<datalist>`  provides a list of predefined values to suggest to the user
        /// for this input. Any values in the list that are not compatible with the type are not
        /// included in the suggested options.  The values provided are suggestions, not
        /// requirements: users can select from this predefined list or provide a different value.
        /// 
        /// It is valid on text, search, url, tel, email, date, month, week, time, datetime-local,
        /// number, range, and color.
        /// 
        /// Per the specifications, the list attribute is not supported by the hidden, password,
        /// checkbox, radio, file, or any of the button types.
        /// 
        /// Depending on the browser, the user may see a custom color palette suggested, tic marks
        /// along a range, or even a input that opens like a select but allows for non-listed
        /// values. Check out the browser compatibility table for the other input types.
        /// 
        /// See the `<datalist>` element.
        list: String,

        /// Valid for date, month, week, time, datetime-local, number, and range, it defines the
        /// greatest value in the range of permitted values. If the value entered into the element
        /// exceeds this, the element fails constraint validation. If the value of the max attribute
        /// isn't a number, then the element has no maximum value.
        /// 
        /// There is a special case: if the data type is periodic (such as for dates or times), the
        /// value of max may be lower than the value of min, which indicates that the range may wrap
        /// around; for example, this allows you to specify a time range from 10 PM to 4 AM.
        max: String,

        /// Valid for text, search, url, tel, email, and password, it defines the maximum number of
        /// characters (as UTF-16 code units) the user can enter into the field. This must be an
        /// integer value 0 or higher. If no maxlength is specified, or an invalid value is
        /// specified, the field has no maximum length. This value must also be greater than or
        /// equal to the value of minlength.
        /// 
        /// The input will fail constraint validation if the length of the text entered into the
        /// field is greater than maxlength UTF-16 code units long. By default, browsers prevent
        /// users from entering more characters than allowed by the maxlength attribute.
        maxlength: String,

        /// Valid for date, month, week, time, datetime-local, number, and range, it defines the
        /// most negative value in the range of permitted values. If the value entered into the
        /// element is less than this this, the element fails constraint validation. If the value of
        /// the min attribute isn't a number, then the element has no minimum value.
        /// 
        /// This value must be less than or equal to the value of the max attribute. If the min
        /// attribute is present but is not specified or is invalid, no min value is applied. If the
        /// min attribute is valid and a non-empty value is less than the minimum allowed by the min
        /// attribute, constraint validation will prevent form submission.
        /// 
        /// There is a special case: if the data type is periodic (such as for dates or times), the
        /// value of max may be lower than the value of min, which indicates that the range may wrap
        /// around; for example, this allows you to specify a time range from 10 PM to 4 AM.
        min: String,

        /// Valid for text, search, url, tel, email, and password, it defines the minimum number of
        /// characters (as UTF-16 code units) the user can enter into the entry field. This must be
        /// an non-negative integer value smaller than or equal to the value specified by maxlength.
        /// If no minlength is specified, or an invalid value is specified, the input has no minimum
        /// length.
        /// 
        /// The input will fail constraint validation if the length of the text entered into the
        /// field is fewer than minlength UTF-16 code units long, preventing form submission.
        minlength: String,

        /// If set, means the user can enter comma separated email addresses in the email widget or
        /// can choose more than one file with the file input. See the email and file input type.
        multiple: bool,

        /// A string specifying a name for the input control. This name is submitted along with the
        /// control's value when the form data is submitted.
        /// 
        /// # What's in a name
        /// 
        /// Consider the name a required attribute (even though it's not). If an input has no name
        /// specified, or name is empty, the input's value is not submitted with the form! (Disabled
        /// controls, unchecked radio buttons, unchecked checkboxes, and reset buttons are also not
        /// sent.)
        /// 
        /// There are two special cases:
        /// 
        /// * `_charset_`: If used as the name of an `<input>` element of type hidden, the input's
        /// value is automatically set by the user agent to the character encoding being used to
        /// submit the form.
        /// * `isindex`: For historical reasons, the name isindex is not allowed.
        /// 
        /// # name and radio buttons
        /// 
        /// The name attribute creates a unique behavior for radio buttons.
        /// 
        /// Only one radio button in a same-named group of radio buttons can be checked at a time.
        /// Selecting any radio button in that group automatically deselects any currently-selected
        /// radio button in the same group. The value of that one checked radio button is sent along
        /// with the name if the form is submitted.
        /// 
        /// When tabbing into a series of same-named group of radio buttons, if one is checked, that
        /// one will receive focus. If they aren't grouped together in source order, if one of the
        /// group is checked, tabbing into the group starts when the first one in the group is
        /// encountered, skipping all those that aren't checked. In other words, if one is checked,
        /// tabbing skips the unchecked radio buttons in the group. If none are checked, the radio
        /// button group receives focus when the first button in the same name group is reached.
        /// 
        /// Once one of the radio buttons in a group has focus, using the arrow keys will navigate
        /// through all the radio buttons of the same name, even if the radio buttons are not
        /// grouped together in the source order.
        /// 
        /// # HTMLFormElement.elements
        /// 
        /// When an input element is given a name, that name becomes a property of the owning form
        /// element's HTMLFormElement.elements property.
        /// 
        /// Warning: Avoid giving form elements a name that corresponds to a built-in property of
        /// the form, since you would then override the predefined property or method with this
        /// reference to the corresponding input.
        name: String,

        /// The pattern attribute, when specified, is a regular expression that the input's value
        /// must match in order for the value to pass constraint validation. It must be a valid
        /// JavaScript regular expression, as used by the RegExp type, and as documented in our
        /// guide on regular expressions; the 'u' flag is specified when compiling the regular
        /// expression, so that the pattern is treated as a sequence of Unicode code points, instead
        /// of as ASCII. No forward slashes should be specified around the pattern text.
        /// 
        /// If the pattern attribute is present but is not specified or is invalid, no regular
        /// expression is applied and this attribute is ignored completely. If the pattern attribute
        /// is valid and a non-empty value does not match the pattern, constraint validation will
        /// prevent form submission.
        /// 
        /// Tip: If using the pattern attribute, inform the user about the expected format by
        /// including explanatory text nearby. You can also include a title attribute to explain
        /// what the requirements are to match the pattern; most browsers will display this title as
        /// a tooltip. The visible explanation is required for accessibility. The tooltip is an
        /// enhancement.
        pattern: String,

        /// The placeholder attribute is a string that provides a brief hint to the user as to what
        /// kind of information is expected in the field. It should be a word or short phrase that
        /// demonstrates the expected type of data, rather than an explanatory message. The text
        /// must not include carriage returns or line feeds.
        /// 
        /// Note: The placeholder attribute is not as semantically useful as other ways to explain
        /// your form, and can cause unexpected technical issues with your content.
        placeholder: String,

        /// If present, indicates that the user should not be able to edit the value of the input.
        /// The readonly attribute is supported  text, search, url, tel, email, date, month, week,
        /// time, datetime-local, number, and password input types.
        readonly: bool,

        /// If present, indicates that the user must specify a value for the input before the owning
        /// form can be submitted. The required attribute is supported  text, search, url, tel,
        /// email, date, month, week, time, datetime-local, number, password, checkbox, radio, and
        /// file.
        required: bool,

        /// Valid for email, password, tel, and text input types only. Specifies how much of the
        /// input is shown. Basically creates same result as setting CSS width property with a few
        /// specialities. The actual unit of the value depends on the input type. For password and
        /// text it's number of characters (or em units) and pixels for others. CSS width takes
        /// precedence over size attribute.
        size: String,

        /// Valid for the image input button only, the src is string specifying the URL of the image
        /// file to display to represent the graphical submit button. See the image input type.
        src: String,

        /// Valid for the numeric input types, including number, date/time input types, and range,
        /// the step attribute is a number that specifies the granularity that the value must adhere
        /// to.
        /// 
        /// If not explicitly included, step defaults to 1 for number and range, and 1 unit type
        /// (second, week, month, day) for the date/time input types. The value can must be a
        /// positive number—integer or float—or the special value any, which means no stepping is
        /// implied, and any value is allowed (barring other constraints, such as min and max).
        /// 
        /// If any is not explicity set, valid values for the number, date/time input types, and
        /// range input types are equal to the basis for stepping - the min value and increments of
        /// the step value, up to the max value, if specified.
        /// 
        /// For example, if you have `<input type="number" min="10" step="2">`, then any even
        /// integer, 10 or greater, is valid. If omitted, `<input type="number">`, any integer is
        /// valid, but floats (like 4.2) are not valid, because step defaults to 1. For 4.2 to be
        /// valid, step would have had to be set to any, 0.1, 0.2, or any the min value would have
        /// had to be a number ending in .2, such as `<input type="number" min="-5.2">`.
        /// 
        /// Note: When the data entered by the user doesn't adhere to the stepping configuration,
        /// the value is considered invalid in contraint validation and will match the :invalid
        /// pseudoclass.
        /// 
        /// The default stepping value for number inputs is 1, allowing only integers to be entered,
        /// unless the stepping base is not an integer. The default stepping value for time is 1
        /// second (with 900 being equal to 15 minutes).
        step: String,

        /// Global attribute valid for all elements, including all the input types, an integer
        /// attribute indicating if the element can take input focus (is focusable), if it should
        /// participate to sequential keyboard navigation. As all input types except for input of
        /// type hidden are focusable, this attribute should not be used on form controls, because
        /// doing so would require the management of the focus order for all elements within the
        /// document with the risk of harming usability and accessibility if done incorrectly.
        tabindex: String,

        /// Global attribute valid for all elements, including all input types, containing a text
        /// representing advisory information related to the element it belongs to. Such information
        /// can typically, but not necessarily, be presented to the user as a tooltip. The title
        /// should NOT be used as the primary explanation of the purpose of the form control.
        /// Instead, use the `<label>` element with a for attribute set to the form control's id
        /// attribute.
        title: String,

        /// A string specifying the type of control to render. For example, to create a checkbox, a
        /// value of checkbox is used. If omitted (or an unknown value is specified), the input type
        /// text is used, creating a plaintext input field.
        /// 
        /// Permitted values are listed in `<input>` types above.
        type_: String,

        /// The input control's value. When specified in the HTML, this is the initial value, and
        /// from then on it can be altered or retrieved at any time using JavaScript to access the
        /// respective HTMLInputElement object's value property. The value attribute is always
        /// optional, though should be considered mandatory for checkbox, radio, and hidden.
        value: String,

        /// Valid for the image input button only, the width is the width of the image file to
        /// display to represent the graphical submit button. See the image input type.
        width: String,

    }
);

dom_type!(input <dom::HtmlInputElement>);

html_element!(
    /// The [HTML `<label>` element][mdn] represents a caption for an item in a user interface.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/label
    label {
        /// The id of a labelable form-related element in the same document as the `<label>`
        /// element. The first element in the document with an id matching the value of the for
        /// attribute is the labeled control for this label element, if it is a labelable element.
        /// If it is not labelable then the for attribute has no effect. If there are other elements
        /// which also match the id value, later in the document, they are not considered.
        /// 
        /// Note: A `<label>` element can have both a for attribute and a contained control element,
        /// as long as the for attribute points to the contained control element.
        for_: String,

        /// The `<form>` element with which the label is associated (its form owner). If specified,
        /// the value of the attribute is the id of a `<form>` element in the same document. This
        /// lets you place label elements anywhere within a document, not just as descendants of
        /// their form elements.
        form: String,

    }
);

dom_type!(label <dom::HtmlLabelElement>);
text_parent!(label);

html_element!(
    /// The [HTML `<legend>` element][mdn] represents a caption for the content of its parent
    /// [`<fieldset>`][fieldset].
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/legend
    /// [fieldset]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/fieldset
    legend {
    }
);

dom_type!(legend <dom::HtmlLegendElement>);
text_parent!(legend);

html_element!(
    /// The [HTML `<meter>` element][mdn] represents either a scalar value within a known range or a
    /// fractional value.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meter
    meter {
        /// The current numeric value. This must be between the minimum and maximum values (min
        /// attribute and max attribute) if they are specified. If unspecified or malformed, the
        /// value is 0. If specified, but not within the range given by the min attribute and max
        /// attribute, the value is equal to the nearest end of the range.
        /// 
        /// Note: Unless the value attribute is between 0 and 1 (inclusive), the min and max
        /// attributes should define the range so that the value attribute's value is within it.
        value: String,

        /// The lower numeric bound of the measured range. This must be less than the maximum value
        /// (max attribute), if specified. If unspecified, the minimum value is 0.
        min: String,

        /// The upper numeric bound of the measured range. This must be greater than the minimum
        /// value (min attribute), if specified. If unspecified, the maximum value is 1.
        max: String,

        /// The `<form>` element to associate the `<meter>` element with (its form owner). The value
        /// of this attribute must be the id of a `<form>` in the same document. If this attribute
        /// is not set, the `<button>` is associated with its ancestor `<form>` element, if any.
        /// This attribute is only used if the `<meter>` element is being used as a form-associated
        /// element, such as one displaying a range corresponding to an `<input type="number">`.
        form: String,

        /// The upper numeric bound of the low end of the measured range. This must be greater than
        /// the minimum value (min attribute), and it also must be less than the high value and
        /// maximum value (high attribute and max attribute, respectively), if any are specified. If
        /// unspecified, or if less than the minimum value, the low value is equal to the minimum
        /// value.
        high: u32,

        /// The lower numeric bound of the high end of the measured range. This must be less than
        /// the maximum value (max attribute), and it also must be greater than the low value and
        /// minimum value (low attribute and min attribute, respectively), if any are specified. If
        /// unspecified, or if greater than the maximum value, the high value is equal to the
        /// maximum value.
        low: u32,

        /// This attribute indicates the optimal numeric value. It must be within the range (as
        /// defined by the min attribute and max attribute). When used with the low attribute and
        /// high attribute, it gives an indication where along the range is considered preferable.
        /// For example, if it is between the min attribute and the low attribute, then the lower
        /// range is considered preferred.
        optimum: u32,

    }
);

dom_type!(meter <dom::HtmlMeterElement>);
text_parent!(meter);

html_element!(
    /// The [HTML `<optgroup>` element][mdn] creates a grouping of options within a
    /// [`<select>`][select] element.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/optgroup
    /// [select]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select
    optgroup {
        /// If set, none of the items in this option group is selectable. Often browsers grey out
        /// such control and it won't receive any browsing events, like mouse clicks or
        /// focus-related ones.
        disabled: bool,

        /// The name of the group of options, which the browser can use when labeling the options in
        /// the user interface. This attribute is mandatory if this element is used.
        label: String,

    }
);

dom_type!(optgroup <dom::HtmlOptGroupElement>);
text_parent!(optgroup);

html_element!(
    /// The [HTML `<option>` element][mdn] is used to define an item contained in a
    /// [`<select>`][select], an [`<optgroup>`][optgroup], or a [`<datalist>`][datalist] element. As
    /// such, `<option>` can represent menu items in popups and other lists of items in an HTML
    /// document.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/option
    /// [select]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select
    /// [optgroup]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/optgroup
    /// [datalist]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/datalist
    option {
        /// If set, this option is not checkable. Often browsers grey out such control and it won't
        /// receive any browsing event, like mouse clicks or focus-related ones. If this attribute
        /// is not set, the element can still be disabled if one of its ancestors is a disabled
        /// `<optgroup>` element.
        disabled: bool,

        /// This attribute is text for the label indicating the meaning of the option. If the label
        /// attribute isn't defined, its value is that of the element text content.
        label: String,

        /// If present, indicates that the option is initially selected. If the `<option>` element
        /// is the descendant of a `<select>` element whose multiple attribute is not set, only one
        /// single `<option>` of this `<select>` element may have the selected attribute.
        selected: bool,

        /// The content of this attribute represents the value to be submitted with the form, should
        /// this option be selected. If this attribute is omitted, the value is taken from the text
        /// content of the option element.
        value: String,

    }
);

dom_type!(option <dom::HtmlOptionElement>);
text_parent!(option);

html_element!(
    /// The [HTML Output element (`<output>`)][mdn] is a container element into which a site or app
    /// can inject the results of a calculation or the outcome of a user action.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/output
    output {
        /// A space-separated list of other elements’ ids, indicating that those elements
        /// contributed input values to (or otherwise affected) the calculation.
        for_: String,

        /// The `<form>` element to associate the output with (its form owner). The value of this
        /// attribute must be the id of a `<form>` in the same document. (If this attribute is not
        /// set, the `<output>` is associated with its ancestor `<form>` element, if any.)
        /// 
        /// This attribute lets you associate `<output>` elements to `<form>`s anywhere in the
        /// document, not just inside a `<form>`. It can also override an ancestor `<form>` element.
        form: String,

        /// The element's name. Used in the form.elements API.
        name: String,

    }
);

dom_type!(output <dom::HtmlOutputElement>);
text_parent!(output);

html_element!(
    /// The [HTML `<progress>` element][progress] displays an indicator showing the completion
    /// progress of a task, typically displayed as a progress bar.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/progress
    progress {
        /// This attribute describes how much work the task indicated by the progress element
        /// requires. The max attribute, if present, must have a value greater than 0 and be a valid
        /// floating point number. The default value is 1.
        max: f32,

        /// This attribute specifies how much of the task that has been completed. It must be a
        /// valid floating point number between 0 and max, or between 0 and 1 if max is omitted. If
        /// there is no value attribute, the progress bar is indeterminate; this indicates that an
        /// activity is ongoing with no indication of how long it is expected to take.
        value: f32,

    }
);

dom_type!(progress <dom::HtmlProgressElement>);
text_parent!(progress);

html_element!(
    /// The [HTML `<select>` element][mdn] represents a control that provides a menu of options.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select
    select {
        /// A DOMString providing a hint for a user agent's autocomplete feature.
        autocomplete: String,

        /// Lets you specify that a form control should have input focus when the page loads. Only
        /// one form element in a document can have the autofocus attribute.
        autofocus: bool,

        /// Indicates that the user cannot interact with the control. If this attribute is not
        /// specified, the control inherits its setting from the containing element, for example
        /// `<fieldset>`; if there is no containing element with the disabled attribute set, then
        /// the control is enabled.
        disabled: bool,

        /// The `<form>` element to associate the `<select>` with (its form owner). The value of
        /// this attribute must be the id of a `<form>` in the same document. (If this attribute is
        /// not set, the `<select>` is associated with its ancestor `<form>` element, if any.)
        /// 
        /// This attribute lets you associate `<select>` elements to `<form>`s anywhere in the
        /// document, not just inside a `<form>`. It can also override an ancestor `<form>` element.
        form: String,

        /// Indicates that multiple options can be selected in the list. If it is not specified,
        /// then only one option can be selected at a time. When multiple is specified, most
        /// browsers will show a scrolling list box instead of a single line dropdown.
        multiple: bool,

        /// This attribute is used to specify the name of the control.
        name: String,

        /// Indicates that an option with a non-empty string value must be selected.
        required: bool,

        /// If the control is presented as a scrolling list box (e.g. when multiple is specified),
        /// this attribute represents the number of rows in the list that should be visible at one
        /// time. Browsers are not required to present a select element as a scrolled list box. The
        /// default value is 0.
        size: String,

    }
);

dom_type!(select <dom::HtmlSelectElement>);
text_parent!(select);

html_element!(
    /// The [HTML `<textarea>` element][mdn] represents a multi-line plain-text editing control,
    /// useful when you want to allow users to enter a sizeable amount of free-form text, for
    /// example a comment on a review or feedback form.
    /// 
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea
    textarea {
        /// This attribute indicates whether the value of the control can be automatically completed
        /// by the browser. Possible values are:
        /// 
        /// * off: The user must explicitly enter a value into this field for every use, or the
        /// document provides its own auto-completion method; the browser does not automatically
        /// complete the entry.
        /// * on: The browser can automatically complete the value based on values that the user has
        /// entered during previous uses.
        /// 
        /// If the autocomplete attribute is not specified on a `<textarea>` element, then the
        /// browser uses the autocomplete attribute value of the `<textarea>` element's form owner.
        /// The form owner is either the `<form>` element that this `<textarea>` element is a
        /// descendant of or the form element whose id is specified by the form attribute of the
        /// input element. For more information, see the autocomplete attribute in `<form>`.
        autocomplete: String,

        /// Lets you specify that a form control should have input focus when the page loads. Only
        /// one form-associated element in a document can have this attribute specified.
        autofocus: bool,

        /// The visible width of the text control, in average character widths. If it is not
        /// specified, the default value is 20.
        cols: u32,

        /// Indicates that the user cannot interact with the control. If this attribute is not
        /// specified, the control inherits its setting from the containing element, for example
        /// `<fieldset>`; if there is no containing element when the disabled attribute is set, the
        /// control is enabled.
        disabled: bool,

        /// The form element that the `<textarea>` element is associated with (its "form owner").
        /// The value of the attribute must be the id of a form element in the same document. If
        /// this attribute is not specified, the `<textarea>` element must be a descendant of a form
        /// element. This attribute enables you to place `<textarea>` elements anywhere within a
        /// document, not just as descendants of form elements.
        form: String,

        /// The maximum number of characters (UTF-16 code units) that the user can enter. If this
        /// value isn't specified, the user can enter an unlimited number of characters.
        maxlength: u32,

        /// The minimum number of characters (UTF-16 code units) required that the user should
        /// enter.
        minlength: u32,

        /// The name of the control.
        name: String,

        /// A hint to the user of what can be entered in the control. Carriage returns or line-feeds
        /// within the placeholder text must be treated as line breaks when rendering the hint.
        /// 
        /// Note: Placeholders should only be used to show an example of the type of data that
        /// should be entered into a form; they are not a substitute for a proper `<label>` element
        /// tied to the input.
        placeholder: String,

        /// Indicates that the user cannot modify the value of the control. Unlike the disabled
        /// attribute, the readonly attribute does not prevent the user from clicking or selecting
        /// in the control. The value of a read-only control is still submitted with the form.
        readonly: bool,

        /// This attribute specifies that the user must fill in a value before submitting a form.
        required: String,

        /// The number of visible text lines for the control.
        rows: String,

        /// Specifies whether the `<textarea>` is subject to spell checking by the underlying
        /// browser/OS. the value can be:
        /// 
        /// * true: Indicates that the element needs to have its spelling and grammar checked.
        /// * default : Indicates that the element is to act according to a default behavior,
        /// possibly based on the parent element's own spellcheck value.
        /// * false : Indicates that the element should not be spell checked.
        spellcheck: String,

        /// Indicates how the control wraps text. Possible values are:
        /// 
        /// * hard: The browser automatically inserts line breaks (CR+LF) so that each line has no
        /// more than the width of the control; the cols attribute must also be specified for this
        /// to take effect.
        /// * soft: The browser ensures that all line breaks in the value consist of a CR+LF pair,
        /// but does not insert any additional line breaks.
        /// 
        /// If this attribute is not specified, soft is its default value.
        wrap: String,

    }
);

dom_type!(textarea <dom::HtmlTextAreaElement>);
text_parent!(textarea);

