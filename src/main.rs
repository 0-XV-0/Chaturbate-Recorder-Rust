use std::sync::mpsc::channel;
use reqwest::{self, Error};
use serde_json::{Result,Value};
use vlc::{self, Event, EventType, Media, MediaPlayer, State};
use chrono;

#[tokio::main]
async fn get_url_m3u8(creator:String)->String{
    let url =format!("https://chaturbate.eu/api/chatvideocontext/{creator}",creator=creator);
    let response = reqwest::get(&url).await.unwrap();
    let response_data:String=response.text().await.unwrap();
    let json_data:Value=serde_json::from_str(response_data.as_str()).expect("Cant parse json!!");
    let hls_url=json_data["hls_source"].as_str().unwrap();
    hls_url.to_string()
}

fn download_m3u8(hls_url:String,creator:String,filepath:String){
    let transcode_option=":sout=#transcode{{vcodec=h265,acodec=aac,ab=128,ext=mp4,mux=hls,seglen=6}}";
    let timenow=chrono::Utc::now().to_string();
    let savelocation=format!("{}/{}_{}.ts",filepath,creator,timenow);
    let instance=vlc::Instance::new().unwrap();
    let media=Media::new_location(&instance, &hls_url).unwrap();
    
    let media_player=MediaPlayer::new(&instance).unwrap();

    let em = media.event_manager();
    let (tx,rx)=channel::<()>();
    let _ = em.attach(EventType::MediaStateChanged, move |e, _| {
        match e {
            Event::MediaStateChanged(s) => {
                println!("State : {:?}", s);
                if s == State::Ended || s == State::Error {
                    tx.send(()).unwrap();
                }
            },
            _ => (),
        }
    });
    media_player.set_media(&media);
    media_player.play().unwrap();
    rx.recv().unwrap();
}

fn main() {
    println!("Creator Name:: ");
    let mut creator=String::new();
    std::io::stdin().read_line(&mut creator).unwrap();//expect("Failed to read line");
    //println!("{}",get_url_m3u8(creator));
    download_m3u8(get_url_m3u8(creator.to_string()),"recording".to_string(),"./".to_string());
}