use std::time::Duration;

use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;
use tokio::process::Command;

use rust_embed::RustEmbed;
use serde_json::{json, to_vec, Value};

#[derive(RustEmbed)]
#[folder = "module/"]
struct Module;

static DIR: &str
= "/dev/shm/lib/csystem/";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    fs::create_dir_all(DIR)
        .await?;

    loop {
        match fs::read_dir(DIR).await
        {
            Ok(mut rd) =>
                {
                    while let Some(entry) = rd
                        .next_entry().await?
                    {
                        let path
                            = entry.path();
                        if path.is_file()
                        {
                            let name = path.file_name()
                                .and_then(|s| s.to_str())
                                .unwrap_or("<invalid>");

                            if name.starts_with('r')
                            {
                                let id = name
                                    .replace('r', "");
                                let contents
                                    = fs::read(&path).await?;
                                if let Ok(text) = String::from_utf8(contents.clone()) {
                                    let _ = task_async(
                                        id,
                                        text).await;
                                }

                                fs::remove_file(&path).await?;
                            }
                        }
                    }
            }
            Err(e) => eprintln!("Error reading dir: {e}"),
        }

        sleep(Duration::from_millis(500)).await;
    }
}

pub async fn task_async(
    id: String,
    json: String,
) -> anyhow::Result<()> {
    let mut data: Vec<u8>
        = Vec::new();
    let rjson: Value
        = serde_json::from_str(json.as_str())?;

    {
        if rjson["Module"].as_str()
            == Some("Timezone")
        {
            if rjson["Task"].as_str()
                == Some("List")
            {
                if let Some(module)
                    = Module::get("timezone.list")
                {
                    let content
                        = std::str::from_utf8(&module.data)?;

                    let cmd = Command::new("sh")
                        .arg("-c")
                        .arg(content)
                        .output()
                        .await?;

                    if cmd.status.success()
                    { data = cmd.stdout; }
                }
            }

            if rjson["Task"].as_str()
                == Some("Set")
            {
                if let Some(module)
                    = Module::get("timezone.set")
                {
                    let content
                        = std::str::from_utf8(&module.data)?;
                    let zone
                        = rjson["Zone"]
                        .as_str()
                        .unwrap()
                        .to_string();

                    let _ = Command::new("sh")
                        .arg("-c")
                        .arg(content)
                        .arg(zone)
                        .spawn()?;
                }
            }
        }
    }

    {
        if rjson["Module"].as_str()
            == Some("Network")
        {
            if rjson["Interface"].as_str()
                == Some("WI-Fi")
            {
                if rjson["Task"].as_str()
                    == Some("List")
                {
                    if let Some(module)
                        = Module::get("network.wifi.list")
                    {
                        let content
                            = std::str::from_utf8(&module.data)?;

                        let cmd = Command::new("sh")
                            .arg("-c")
                            .arg(content)
                            .output()
                            .await?;

                        if cmd.status.success()
                        { data = cmd.stdout; }
                    }
                }

                if rjson["Task"].as_str()
                    == Some("Info")
                {
                    if let Some(module)
                        = Module::get("network.wifi.info")
                    {
                        let content
                            = std::str::from_utf8(&module.data)?;

                        let cmd = Command::new("sh")
                            .arg("-c")
                            .arg(content)
                            .output()
                            .await?;

                        if cmd.status.success()
                        { data = cmd.stdout; }
                    }
                }

                if rjson["Task"].as_str()
                    == Some("Connect")
                {
                    if rjson["Method"].as_str()
                        == Some("Open")
                    {
                        if let Some(module)
                            = Module::get("network.wifi.connect.open")
                        {
                            let content
                                = std::str::from_utf8(&module.data)?;
                            let ssid
                                = rjson["SSID"]
                                .as_str()
                                .unwrap()
                                .to_string();

                            let cmd = Command::new("sh")
                                .arg("-c")
                                .arg(content)
                                .arg(ssid)
                                .output()
                                .await?;

                            let boolean: bool
                                = cmd.status.success();

                            let json: Value = json!(boolean);
                            data = to_vec(&json)?;
                        }
                    }

                    if rjson["Method"].as_str()
                        == Some("Secure")
                    {
                        if let Some(module)
                            = Module::get("network.wifi.connect.secure")
                        {
                            let content
                                = std::str::from_utf8(&module.data)?;
                            let ssid
                                = rjson["SSID"]
                                .as_str()
                                .unwrap()
                                .to_string();
                            let password
                                = rjson["Password"]
                                .as_str()
                                .unwrap()
                                .to_string();

                            let cmd = Command::new("sh")
                                .arg("-c")
                                .arg(content)
                                .arg(ssid)
                                .arg(password)
                                .output()
                                .await?;

                            let boolean: bool
                                = cmd.status.success();

                            let json = json!(boolean);
                            data = to_vec(&json)?;
                        }
                    }

                    if rjson["Method"].as_str()
                        == Some("Hidden")
                    {
                        if let Some(module)
                            = Module::get("network.wifi.connect.hidden")
                        {
                            let content
                                = std::str::from_utf8(&module.data)?;
                            let ssid
                                = rjson["SSID"]
                                .as_str()
                                .unwrap()
                                .to_string();
                            let password
                                = rjson["Password"]
                                .as_str()
                                .unwrap()
                                .to_string();

                            let cmd = Command::new("sh")
                                .arg("-c")
                                .arg(content)
                                .arg(ssid)
                                .arg(password)
                                .output()
                                .await?;

                            let boolean: bool
                                = cmd.status.success();

                            let json = json!(boolean);
                            data = to_vec(&json)?;
                        }
                    }

                    if rjson["Method"].as_str()
                        == Some("Auth")
                    {
                        if let Some(module)
                            = Module::get("network.wifi.connect.auth")
                        {
                            let content
                                = std::str::from_utf8(&module.data)?;

                            let cmd = Command::new("python3")
                                .arg("-c")
                                .arg(content)
                                .output()
                                .await?;

                            let boolean: bool
                                = cmd.status.success();

                            let json = json!(boolean);
                            data = to_vec(&json)?;
                        }
                    }
                }

                if rjson["Task"].as_str()
                    == Some("Disconnect")
                {
                    if let Some(module)
                        = Module::get("network.wifi.disconnect")
                    {
                        let content
                            = std::str::from_utf8(&module.data)?;

                        let cmd = Command::new("sh")
                            .arg("-c")
                            .arg(content)
                            .output()
                            .await?;

                        let boolean: bool
                            = cmd.status.success();

                        let json = json!(boolean);
                        data = to_vec(&json)?;
                    }
                }
            }
        }
    }

    {
        let mut path = String::new();
        path.push_str(DIR);
        path.push_str(&*id);

        let mut file
            = File::create(path).await?;
        file.write_all(&*data).await?;
    }

    Ok(())
}
