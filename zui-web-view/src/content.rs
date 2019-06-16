//! Content to display within a [`WebView`](struct.WebView.html).

#[cfg(target_os = "macos")]
use cocoa::base::id;

#[derive(Clone, Debug)]
pub(crate) enum ContentInner<'a> {
    #[cfg(target_os = "macos")]
    HtmlNSString {
        content: id,
        base_url: id,
    },

    #[cfg(target_os = "macos")]
    NSUrl(id),

    Html(&'a str),

    Url(&'a str),
}

/// Content that can be displayed within a [`WebView`](struct.WebView.html).
pub struct Content<'a>(pub(crate) ContentInner<'a>);

impl<'a> Content<'a> {
    /// Display `html`.
    #[inline]
    pub const fn html(html: &'a str) -> Self {
        Content(ContentInner::Html(html))
    }

    /// Point to `url`.
    #[inline]
    pub const fn url(url: &'a str) -> Self {
        Content(ContentInner::Url(url))
    }
}

impl Default for Content<'_> {
    #[inline]
    fn default() -> Self {
        // <!DOCTYPE html>
        // <html lang="en">
        //   <head>
        //     <meta charset="utf-8">
        //     <meta http-equiv="X-UA-Compatible" content="IE=edge">
        //   </head>
        //   <body>
        //     <p>Hello ZedUI!</p>
        //   </body>
        // </html>
        Content::url("\
            data:text/html,\
            %3C%21DOCTYPE%20html%3E%3Chtml%20lang%3D%22en%22%3E%3Chead%3E%3Cmet\
            a%20charset%3D%22utf-8%22%3E%3Cmeta%20http-equiv%3D%22X-UA-Compatib\
            le%22%20content%3D%22IE%3Dedge%22%3E%3C%2Fhead%3E%3Cbody%3E%3Cp%3EH\
            ello%20ZedUI%21%3C%2Fp%3E%3C%2Fbody%3E%3C%2Fhtml%3E\
        ")
    }
}
