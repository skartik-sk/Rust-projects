use std::fs::OpenOptions;
use std::io::Write;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};
async fn handle_client(stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    writer.write_all(b"220 Rust SMTP Server Ready\r\n").await?;

    loop {
        line.clear();
        let bytes = reader.read_line(&mut line).await?;

        if bytes == 0 {
            break;
        }

        // println!("CLIENT: {}", line.trim());

        match line.trim() {
            cmd if cmd.starts_with("HELO") => {
                writer.write_all(b"250 Hello\r\n").await?;
            }

            cmd if cmd.starts_with("MAIL FROM") => {
                writer.write_all(b"250 OK\r\n").await?;
            }

            cmd if cmd.starts_with("RCPT TO") => {
                writer.write_all(b"250 OK\r\n").await?;
            }

            "DATA" => {
                writer.write_all(b"354 End with .\r\n").await?;

                let mut message = String::new();

                loop {
                    line.clear();
                    reader.read_line(&mut line).await?;

                    if line.trim() == "." {
                        break;
                    }

                    message.push_str(&line);
                }

                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open("emails.txt")?;

                writeln!(file, "{}", message)?;

                writer.write_all(b"250 Message stored\r\n").await?;
            }
            "." => {
                writer.write_all(b"250 Message accepted\r\n").await?;
            }

            "QUIT" => {
                writer.write_all(b"221 Bye\r\n").await?;
                break;
            }

            _ => {
                writer.write_all(b"500 Unknown command\r\n").await?;
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:2525").await?;

    println!("SMTP Server running on port 2525");
    println!("Listerner Log {:?}", listener);
    loop {
        let (socket, _) = listener.accept().await?;
        println!("Socket log {:?}", socket);
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket).await {
                println!("Error: {:?}", e);
            }
        });
    }
}
