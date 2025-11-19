use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    // User-controlled fields
    let port = "33030";
    let construction_title = "🚧 Under Construction";
    let construction_message = "Site coming soon.";

    let bind_addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&bind_addr).unwrap();
    println!("Serving on http://{}", bind_addr);

    let html = format!(
        "HTTP/1.1 200 OK\r\n\
Content-Type: text/html; charset=utf-8\r\n\
\r\n\
<!DOCTYPE html>\
<html lang=\"en\">\
<head>\
<meta charset=\"UTF-8\">\
<title>{title}</title>\
<style>\
    body {{\
        background: #121212;\
        color: #e0e0e0;\
        font-family: Arial, sans-serif;\
        display: flex;\
        justify-content: center;\
        align-items: center;\
        height: 100vh;\
        margin: 0;\
    }}\
    .box {{ text-align: center; }}\
    h1 {{\
        font-size: 3rem;\
        margin-bottom: 0.5rem;\
        color: #f5f5f5;\
    }}\
    p {{\
        font-size: 1.2rem;\
        color: #bdbdbd;\
    }}\
    .blink {{\
        animation: pulse 1.4s infinite ease-in-out;\
    }}\
    @keyframes pulse {{\
        0%, 100% {{ opacity: 0.4; }}\
        50% {{ opacity: 1.0; }}\
    }}\
</style>\
</head>\
<body>\
    <div class=\"box\">\
        <h1>{title}</h1>\
        <p class=\"blink\">{msg}</p>\
    </div>\
</body>\
</html>",
        title = construction_title,
        msg = construction_message
    );

    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let mut buffer = [0_u8; 1024];
            let _ = stream.read(&mut buffer);
            let _ = stream.write_all(html.as_bytes());
        }
    }
}
