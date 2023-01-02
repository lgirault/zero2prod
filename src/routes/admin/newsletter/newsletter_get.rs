use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn publish_newsletter_form(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta http-equiv="content-type" content="text/html; charset=utf-8">
                    <title>Send newsletter</title>
                </head>
                <body>
                    {msg_html}
                    <form action="/admin/newsletter" method="post"">
                    <label>Title
                    <input
                    type="text"
                    placeholder="Enter title"
                    name="title"
                    >
                    </label>
                    <br>
                    <label>Text Content <br>
                        <textarea rows="4" cols="50" name="text_content">
                            Enter content here...
                        </textarea>
                    </label>
                    <br>
                    <label>Html Content <br>
                        <textarea rows="4" cols="50" name="html_content">
                            Enter html content here...
                        </textarea>
                    </label>
                    <br>
                    <button type="submit">Publish</button>
                    </form>
                    <p><a href="/admin/dashboard">&lt;- Back</a></p>
                </body>
            </html>"#,
        )))
}
